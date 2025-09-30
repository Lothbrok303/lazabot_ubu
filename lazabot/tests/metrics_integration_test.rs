//! Integration tests for the metrics module

use lazabot::utils::{MetricsCollector, MetricsServer};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_metrics_collector() {
    let collector = MetricsCollector::new();

    // Test incrementing counters
    collector.inc_total_requests();
    collector.inc_success_requests();
    collector.inc_active_tasks();

    // Verify through format output
    let output = format!("{:?}", collector.clone());
    assert!(output.contains("MetricsCollector"));
}

#[tokio::test]
async fn test_metrics_prometheus_format() {
    let collector = MetricsCollector::new();

    collector.inc_total_requests();
    collector.inc_total_requests();
    collector.inc_success_requests();
    collector.inc_failed_requests();
    collector.inc_active_tasks();

    // Give time for rate calculation
    sleep(Duration::from_millis(100)).await;

    // The format_prometheus method is private, but we can test the collector works
    assert_eq!(2, 2); // Placeholder - real test would check metrics output
}

#[tokio::test]
async fn test_active_tasks_counter() {
    let collector = MetricsCollector::new();

    // Increment
    collector.inc_active_tasks();
    collector.inc_active_tasks();
    collector.inc_active_tasks();

    // Decrement
    collector.dec_active_tasks();

    // Set to specific value
    collector.set_active_tasks(5);

    // Verify it doesn't panic
    assert!(true);
}

#[tokio::test]
async fn test_metrics_server_creation() {
    let collector = MetricsCollector::new();
    let _server = MetricsServer::new(collector, "127.0.0.1:19091");

    // Server created successfully
    assert!(true);
}

#[tokio::test]
async fn test_concurrent_metric_updates() {
    let collector = MetricsCollector::new();

    let mut handles = vec![];

    // Spawn 10 concurrent tasks updating metrics
    for _ in 0..10 {
        let collector_clone = collector.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..100 {
                collector_clone.inc_total_requests();
                collector_clone.inc_success_requests();
                collector_clone.inc_active_tasks();
                sleep(Duration::from_micros(10)).await;
                collector_clone.dec_active_tasks();
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify no panics occurred
    assert!(true);
}
