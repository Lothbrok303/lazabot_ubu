use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use tokio::sync::{Semaphore, broadcast};
use tokio::task::JoinHandle;
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unique identifier for tasks
pub type TaskId = u64;

/// Status of a task
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is waiting to be executed
    Pending,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed with an error
    Failed,
    /// Task was cancelled
    Cancelled,
}

/// Result of a task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl TaskResult {
    /// Create a new pending task result
    pub fn pending(task_id: TaskId) -> Self {
        Self {
            task_id,
            status: TaskStatus::Pending,
            started_at: None,
            completed_at: None,
            error_message: None,
            metadata: None,
        }
    }

    /// Mark task as running
    pub fn running(mut self) -> Self {
        self.status = TaskStatus::Running;
        self.started_at = Some(Utc::now());
        self
    }

    /// Mark task as completed
    pub fn completed(mut self) -> Self {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
        self
    }

    /// Mark task as failed
    pub fn failed(mut self, error: String) -> Self {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
        self
    }

    /// Mark task as cancelled
    pub fn cancelled(mut self) -> Self {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        self
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// A task that can be executed by the TaskManager
#[async_trait::async_trait]
pub trait Task: Send + Sync {
    /// Execute the task and return the result
    async fn execute(&self) -> Result<serde_json::Value>;

    /// Get the task name for logging
    fn name(&self) -> &str;
}

/// Task manager that handles concurrent task execution
pub struct TaskManager {
    /// Maximum number of concurrent tasks
    max_concurrent: usize,
    /// Semaphore to limit concurrency
    semaphore: Arc<Semaphore>,
    /// In-memory store for task results
    task_store: Arc<DashMap<TaskId, TaskResult>>,
    /// Counter for generating unique task IDs
    task_id_counter: AtomicU64,
    /// Shutdown signal
    shutdown: Arc<AtomicBool>,
    /// Broadcast channel for shutdown notifications
    shutdown_tx: broadcast::Sender<()>,
    /// Join handles for running tasks
    task_handles: Arc<DashMap<TaskId, JoinHandle<()>>>,
}

impl TaskManager {
    /// Create a new TaskManager with the specified concurrency limit
    pub fn new(max_concurrent: usize) -> Self {
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let task_store = Arc::new(DashMap::new());
        let task_id_counter = AtomicU64::new(0);
        let shutdown = Arc::new(AtomicBool::new(false));
        let (shutdown_tx, _) = broadcast::channel(1);
        let task_handles = Arc::new(DashMap::new());

        info!("TaskManager created with max_concurrent={}", max_concurrent);

        Self {
            max_concurrent,
            semaphore,
            task_store,
            task_id_counter,
            shutdown,
            shutdown_tx,
            task_handles,
        }
    }

    /// Submit a task for execution
    pub async fn submit_task<T>(&self, task: T) -> Result<TaskId>
    where
        T: Task + 'static,
    {
        if self.shutdown.load(Ordering::SeqCst) {
            return Err(anyhow::anyhow!("TaskManager is shutting down"));
        }

        // Generate unique task ID
        let task_id = self.task_id_counter.fetch_add(1, Ordering::SeqCst);

        // Create initial task result
        let task_result = TaskResult::pending(task_id);
        self.task_store.insert(task_id, task_result);

        debug!("Task {} '{}' submitted", task_id, task.name());

        // Clone Arc references for the spawned task
        let semaphore = Arc::clone(&self.semaphore);
        let task_store = Arc::clone(&self.task_store);
        let shutdown = Arc::clone(&self.shutdown);
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let task_handles = Arc::clone(&self.task_handles);

        // Spawn the task
        let handle = tokio::spawn(async move {
            // Try to acquire semaphore permit
            let permit = match semaphore.try_acquire() {
                Ok(permit) => permit,
                Err(_) => {
                    // Wait for permit with shutdown check
                    tokio::select! {
                        result = semaphore.acquire() => {
                            match result {
                                Ok(permit) => permit,
                                Err(e) => {
                                    error!("Failed to acquire semaphore permit for task {}: {}", task_id, e);
                                    let result = TaskResult::pending(task_id)
                                        .failed(format!("Failed to acquire semaphore: {}", e));
                                    task_store.insert(task_id, result);
                                    return;
                                }
                            }
                        }
                        _ = shutdown_rx.recv() => {
                            info!("Task {} cancelled before execution due to shutdown", task_id);
                            let result = TaskResult::pending(task_id).cancelled();
                            task_store.insert(task_id, result);
                            return;
                        }
                    }
                }
            };

            // Check shutdown flag before starting
            if shutdown.load(Ordering::SeqCst) {
                info!("Task {} cancelled due to shutdown", task_id);
                let result = TaskResult::pending(task_id).cancelled();
                task_store.insert(task_id, result);
                return;
            }

            // Update task status to running
            let result = TaskResult::pending(task_id).running();
            task_store.insert(task_id, result.clone());
            info!("Task {} '{}' started", task_id, task.name());

            // Execute the task
            let execution_result = tokio::select! {
                result = task.execute() => result,
                _ = shutdown_rx.recv() => {
                    info!("Task {} '{}' interrupted by shutdown", task_id, task.name());
                    let result = result.cancelled();
                    task_store.insert(task_id, result);
                    return;
                }
            };

            // Update task result based on execution outcome
            let final_result = match execution_result {
                Ok(metadata) => {
                    info!("Task {} '{}' completed successfully", task_id, task.name());
                    result.completed().with_metadata(metadata)
                }
                Err(e) => {
                    error!("Task {} '{}' failed: {}", task_id, task.name(), e);
                    result.failed(e.to_string())
                }
            };

            task_store.insert(task_id, final_result);

            // Release semaphore permit explicitly
            drop(permit);

            // Remove task handle from tracking
            task_handles.remove(&task_id);
        });

        // Store the handle
        self.task_handles.insert(task_id, handle);

        Ok(task_id)
    }

    /// Get the result of a task
    pub fn get_task_result(&self, task_id: TaskId) -> Option<TaskResult> {
        self.task_store.get(&task_id).map(|r| r.clone())
    }

    /// Get all task results
    pub fn get_all_task_results(&self) -> Vec<TaskResult> {
        self.task_store
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get task results by status
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<TaskResult> {
        self.task_store
            .iter()
            .filter(|entry| entry.value().status == status)
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get the number of currently running tasks
    pub fn running_tasks_count(&self) -> usize {
        self.get_tasks_by_status(TaskStatus::Running).len()
    }

    /// Get the number of pending tasks
    pub fn pending_tasks_count(&self) -> usize {
        self.get_tasks_by_status(TaskStatus::Pending).len()
    }

    /// Get total number of tasks
    pub fn total_tasks(&self) -> usize {
        self.task_store.len()
    }

    /// Initiate graceful shutdown
    pub async fn shutdown(&self) {
        info!("Initiating TaskManager shutdown");
        self.shutdown.store(true, Ordering::SeqCst);

        // Send shutdown signal to all waiting tasks
        let _ = self.shutdown_tx.send(());

        // Wait for all running tasks to complete
        let handles: Vec<_> = self.task_handles
            .iter()
            .map(|entry| {
                let _handle_ref = entry.value();
                // We need to create a new handle that waits for the original
                // Since JoinHandle is not Clone, we'll collect task_ids and check them
                entry.key().clone()
            })
            .collect();

        info!("Waiting for {} tasks to complete", handles.len());

        // Wait for tasks with a timeout
        let mut remaining_tasks = handles.len();
        let shutdown_timeout = std::time::Duration::from_secs(30);
        let start = std::time::Instant::now();

        while remaining_tasks > 0 && start.elapsed() < shutdown_timeout {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            remaining_tasks = self.task_handles.len();
        }

        if remaining_tasks > 0 {
            warn!("Shutdown timeout reached, {} tasks still running", remaining_tasks);
            // Abort remaining tasks
            for entry in self.task_handles.iter() {
                entry.value().abort();
            }
        }

        info!("TaskManager shutdown complete");
    }

    /// Check if the task manager is shutting down
    pub fn is_shutting_down(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }

    /// Get maximum concurrent tasks limit
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }

    /// Get available permits (approximately)
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }
}

impl Drop for TaskManager {
    fn drop(&mut self) {
        if !self.shutdown.load(Ordering::SeqCst) {
            warn!("TaskManager dropped without explicit shutdown call");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use tokio::time::{sleep, Duration};

    /// Dummy task for testing
    struct DummyTask {
        name: String,
        duration_ms: u64,
        should_fail: bool,
    }

    impl DummyTask {
        fn new(name: impl Into<String>, duration_ms: u64) -> Self {
            Self {
                name: name.into(),
                duration_ms,
                should_fail: false,
            }
        }

        fn with_failure(mut self) -> Self {
            self.should_fail = true;
            self
        }
    }

    #[async_trait::async_trait]
    impl Task for DummyTask {
        async fn execute(&self) -> Result<serde_json::Value> {
            sleep(Duration::from_millis(self.duration_ms)).await;

            if self.should_fail {
                Err(anyhow::anyhow!("Task failed intentionally"))
            } else {
                Ok(serde_json::json!({
                    "task_name": self.name,
                    "duration_ms": self.duration_ms
                }))
            }
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn test_task_manager_basic() {
        let manager = TaskManager::new(5);
        let task = DummyTask::new("test_task", 100);

        let task_id = manager.submit_task(task).await.unwrap();
        assert!(task_id == 0);

        // Wait for task to complete
        sleep(Duration::from_millis(150)).await;

        let result = manager.get_task_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Completed);
    }

    #[tokio::test]
    async fn test_task_manager_concurrency_limit() {
        let max_concurrent = 5;
        let manager = Arc::new(TaskManager::new(max_concurrent));
        let concurrent_counter = Arc::new(AtomicUsize::new(0));
        let max_observed = Arc::new(AtomicUsize::new(0));

        // Submit 50 tasks
        let mut task_ids = Vec::new();
        for i in 0..50 {
            let counter = Arc::clone(&concurrent_counter);
            let max_obs = Arc::clone(&max_observed);

            struct CountingTask {
                name: String,
                counter: Arc<AtomicUsize>,
                max_observed: Arc<AtomicUsize>,
            }

            #[async_trait::async_trait]
            impl Task for CountingTask {
                async fn execute(&self) -> Result<serde_json::Value> {
                    // Increment counter
                    let current = self.counter.fetch_add(1, Ordering::SeqCst) + 1;

                    // Update max observed
                    self.max_observed.fetch_max(current, Ordering::SeqCst);

                    // Simulate work
                    sleep(Duration::from_millis(50)).await;

                    // Decrement counter
                    self.counter.fetch_sub(1, Ordering::SeqCst);

                    Ok(serde_json::json!({
                        "task_name": self.name,
                        "concurrent": current
                    }))
                }

                fn name(&self) -> &str {
                    &self.name
                }
            }

            let task = CountingTask {
                name: format!("task_{}", i),
                counter,
                max_observed: max_obs,
            };

            let task_id = manager.submit_task(task).await.unwrap();
            task_ids.push(task_id);
        }

        // Wait for all tasks to complete
        sleep(Duration::from_secs(2)).await;

        // Verify all tasks completed
        let completed_count = manager.get_tasks_by_status(TaskStatus::Completed).len();
        assert_eq!(completed_count, 50);

        // Verify concurrency limit was respected
        let max_concurrent_observed = max_observed.load(Ordering::SeqCst);
        println!("Max concurrent observed: {}", max_concurrent_observed);
        assert!(max_concurrent_observed <= max_concurrent);
    }

    #[tokio::test]
    async fn test_task_manager_failed_task() {
        let manager = TaskManager::new(5);
        let task = DummyTask::new("failing_task", 100).with_failure();

        let task_id = manager.submit_task(task).await.unwrap();

        // Wait for task to complete
        sleep(Duration::from_millis(150)).await;

        let result = manager.get_task_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Failed);
        assert!(result.error_message.is_some());
    }

    #[tokio::test]
    async fn test_task_manager_shutdown() {
        let manager = Arc::new(TaskManager::new(2));

        // Submit some long-running tasks
        for i in 0..5 {
            let task = DummyTask::new(format!("task_{}", i), 500);
            manager.submit_task(task).await.unwrap();
        }

        // Wait a bit for tasks to start
        sleep(Duration::from_millis(100)).await;

        // Initiate shutdown
        manager.shutdown().await;

        // Verify shutdown flag is set
        assert!(manager.is_shutting_down());

        // Verify we can't submit new tasks
        let task = DummyTask::new("late_task", 100);
        let result = manager.submit_task(task).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_task_manager_status_queries() {
        let manager = TaskManager::new(2);

        // Submit a mix of tasks
        let task1 = DummyTask::new("task1", 200);
        let task2 = DummyTask::new("task2", 200);
        let task3 = DummyTask::new("task3", 50).with_failure();

        manager.submit_task(task1).await.unwrap();
        manager.submit_task(task2).await.unwrap();
        manager.submit_task(task3).await.unwrap();

        // Wait for tasks to complete
        sleep(Duration::from_millis(300)).await;

        // Check status counts
        let completed = manager.get_tasks_by_status(TaskStatus::Completed);
        let failed = manager.get_tasks_by_status(TaskStatus::Failed);

        assert_eq!(completed.len(), 2);
        assert_eq!(failed.len(), 1);
        assert_eq!(manager.total_tasks(), 3);
    }
}
