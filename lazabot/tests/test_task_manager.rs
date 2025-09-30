// Integration tests for TaskManager
//
// This test suite verifies the TaskManager functionality including:
// - Controlled parallel execution of tasks
// - Semaphore-based concurrency limiting
// - TaskResult persistence in DashMap
// - Graceful shutdown handling

use anyhow::Result;
use lazabot::tasks::{Task, TaskManager, TaskStatus};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// A simple task for testing that tracks concurrent execution
struct TestTask {
    name: String,
    duration_ms: u64,
    concurrent_counter: Arc<AtomicUsize>,
    max_observed: Arc<AtomicUsize>,
}

impl TestTask {
    fn new(
        name: impl Into<String>,
        duration_ms: u64,
        counter: Arc<AtomicUsize>,
        max_obs: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            name: name.into(),
            duration_ms,
            concurrent_counter: counter,
            max_observed: max_obs,
        }
    }
}

#[async_trait::async_trait]
impl Task for TestTask {
    async fn execute(&self) -> Result<serde_json::Value> {
        // Increment counter when task starts
        let current = self.concurrent_counter.fetch_add(1, Ordering::SeqCst) + 1;

        // Update max observed concurrency
        self.max_observed.fetch_max(current, Ordering::SeqCst);

        // Simulate work
        sleep(Duration::from_millis(self.duration_ms)).await;

        // Decrement counter when task finishes
        self.concurrent_counter.fetch_sub(1, Ordering::SeqCst);

        Ok(serde_json::json!({
            "task_name": self.name,
            "duration_ms": self.duration_ms,
            "concurrent": current
        }))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[tokio::test]
async fn test_50_tasks_with_max_5_concurrent() {
    // Create TaskManager with max_concurrent = 5
    let max_concurrent = 5;
    let manager = Arc::new(TaskManager::new(max_concurrent));

    // Counters to track concurrent execution
    let concurrent_counter = Arc::new(AtomicUsize::new(0));
    let max_observed = Arc::new(AtomicUsize::new(0));

    println!("Submitting 50 tasks with max_concurrent={}", max_concurrent);

    // Submit 50 dummy tasks
    let mut task_ids = Vec::new();
    for i in 0..50 {
        let counter = Arc::clone(&concurrent_counter);
        let max_obs = Arc::clone(&max_observed);

        let task = TestTask::new(
            format!("task_{}", i),
            50, // 50ms per task
            counter,
            max_obs,
        );

        let task_id = manager
            .submit_task(task)
            .await
            .expect("Failed to submit task");
        task_ids.push(task_id);
    }

    println!("All 50 tasks submitted successfully");

    // Wait for all tasks to complete
    let wait_time = Duration::from_secs(5);
    println!("Waiting up to {:?} for tasks to complete...", wait_time);
    sleep(wait_time).await;

    // Verify all tasks completed successfully
    let completed_tasks = manager.get_tasks_by_status(TaskStatus::Completed);
    let failed_tasks = manager.get_tasks_by_status(TaskStatus::Failed);

    println!("Completed tasks: {}", completed_tasks.len());
    println!("Failed tasks: {}", failed_tasks.len());
    println!("Total tasks: {}", manager.total_tasks());

    assert_eq!(completed_tasks.len(), 50, "All 50 tasks should complete");
    assert_eq!(failed_tasks.len(), 0, "No tasks should fail");

    // Verify concurrency limit was respected
    let max_concurrent_observed = max_observed.load(Ordering::SeqCst);
    println!("Max concurrent tasks observed: {}", max_concurrent_observed);
    println!("Expected max concurrent: {}", max_concurrent);

    assert!(
        max_concurrent_observed <= max_concurrent,
        "Observed concurrency ({}) should not exceed limit ({})",
        max_concurrent_observed,
        max_concurrent
    );

    println!("✓ Test passed: Concurrency limit was respected");
}

#[tokio::test]
async fn test_graceful_shutdown_with_running_tasks() {
    let manager = Arc::new(TaskManager::new(2));

    // Submit several long-running tasks
    for i in 0..5 {
        let counter = Arc::new(AtomicUsize::new(0));
        let max_obs = Arc::new(AtomicUsize::new(0));

        let task = TestTask::new(
            format!("long_task_{}", i),
            300, // 300ms
            counter,
            max_obs,
        );

        manager
            .submit_task(task)
            .await
            .expect("Failed to submit task");
    }

    println!("Submitted 5 long-running tasks");

    // Wait a bit for tasks to start
    sleep(Duration::from_millis(50)).await;

    println!("Initiating graceful shutdown...");

    // Initiate graceful shutdown
    manager.shutdown().await;

    println!("Shutdown complete");

    // Verify shutdown flag is set
    assert!(manager.is_shutting_down(), "Shutdown flag should be set");

    // Verify we cannot submit new tasks after shutdown
    let counter = Arc::new(AtomicUsize::new(0));
    let max_obs = Arc::new(AtomicUsize::new(0));
    let task = TestTask::new("late_task", 100, counter, max_obs);

    let result = manager.submit_task(task).await;
    assert!(
        result.is_err(),
        "Should not be able to submit tasks after shutdown"
    );

    println!("✓ Test passed: Graceful shutdown works correctly");
}
