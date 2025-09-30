use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use lazabot::api::{ApiClient, ProxyInfo};
use lazabot::core::{MonitorEngine, MonitorTask};
use lazabot::proxy::ProxyManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting monitor example...");

    // Create API client
    let api_client = Arc::new(ApiClient::new(Some("Lazabot-Monitor/1.0".to_string()))?);

    // Create proxy manager with some test proxies
    let test_proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("127.0.0.1".to_string(), 8081),
    ];
    let proxy_manager = Arc::new(ProxyManager::new(test_proxies));

    // Create monitor engine
    let mut engine = MonitorEngine::new();

    // Create monitor tasks for different products
    let products = vec![
        (
            "product1",
            "https://httpbin.org/status/200",
            "Test Product 1",
        ),
        (
            "product2",
            "https://httpbin.org/status/200",
            "Test Product 2",
        ),
        (
            "product3",
            "https://httpbin.org/status/404",
            "Test Product 3 (Not Found)",
        ),
    ];

    let mut event_receivers = Vec::new();

    for (product_id, product_url, product_name) in products {
        let monitor = MonitorTask::new(
            product_id.to_string(),
            product_url.to_string(),
            product_name.to_string(),
            api_client.clone(),
            proxy_manager.clone(),
            2000, // Check every 2 seconds
        )
        .with_timeout(10000) // 10 second timeout
        .with_max_retries(2);

        let receiver = engine.add_monitor(monitor);
        event_receivers.push((product_id, receiver));
    }

    // Start the engine
    engine.start().await?;

    // Spawn event handler
    let event_handle = tokio::spawn(async move {
        for (product_id, mut receiver) in event_receivers {
            while let Some(event) = receiver.recv().await {
                info!(
                    "Product {} availability changed: {}",
                    product_id, event.is_available
                );
                info!("   URL: {}", event.product_url);
                info!("   Timestamp: {}", event.timestamp);

                if event.is_available {
                    println!("Product '{}' is now AVAILABLE!", product_id);
                } else {
                    println!("Product '{}' is now UNAVAILABLE", product_id);
                }
            }
        }
    });

    // Run for 30 seconds
    info!("Running monitor for 30 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    // Stop the engine
    info!("Stopping monitor engine...");
    engine.stop().await?;

    // Wait for event handler to finish
    let _ = event_handle.await;

    info!("Monitor example completed!");
    Ok(())
}
