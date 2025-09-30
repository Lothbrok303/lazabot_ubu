//! Metrics server demo
//!
//! This example demonstrates the metrics server functionality.
//! Run with: cargo run --example metrics_demo
//! Then curl http://127.0.0.1:9091/metrics to see the metrics

use lazabot::utils::{MetricsCollector, MetricsServer};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create metrics collector
    let collector = MetricsCollector::new();

    // Start metrics server in background
    let server = MetricsServer::new(collector.clone(), "127.0.0.1:9091");
    tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Metrics server error: {}", e);
        }
    });

    println!("Metrics server started on http://127.0.0.1:9091/metrics");
    println!("Try:");
    println!("  curl http://127.0.0.1:9091/metrics");
    println!("  curl http://127.0.0.1:9091/health");
    println!();
    println!("Simulating workload...");

    // Simulate some workload
    for i in 0..100 {
        // Simulate a task
        collector.inc_active_tasks();
        collector.inc_total_requests();

        // Simulate work
        sleep(Duration::from_millis(100)).await;

        // Simulate success or failure
        if i % 10 == 0 {
            collector.inc_failed_requests();
            println!("Task {} failed", i);
        } else {
            collector.inc_success_requests();
            println!("Task {} succeeded", i);
        }

        collector.dec_active_tasks();

        // Small delay between tasks
        sleep(Duration::from_millis(50)).await;
    }

    println!();
    println!("Workload complete. Metrics server will continue running...");
    println!("Press Ctrl+C to exit");

    // Keep the server running
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
