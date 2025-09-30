use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use lazabot::api::{ApiClient, ProxyInfo};
use lazabot::core::{MonitorEngine, MonitorTask};
use lazabot::proxy::ProxyManager;

/// Test the monitor function with real Lazada.sg products
#[tokio::test]
async fn test_monitor_with_real_lazada_sg_products() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Starting Lazada.sg Monitor Test with Real Products");

    // Real Lazada.sg product URLs from testacc.txt
    let test_products = vec![
        (
            "lazada-flashsale-product",
            "https://www.lazada.sg/products/pdp-i301072965-s527094858.html?scm=1007.17760.398138.0&pvid=e1e36251-ce9e-4ed1-9053-6ad3b0c7c25c&search=flashsale&spm=a2o42.homepage.FlashSale.d_301072965",
            "Lazada Flash Sale Product",
        ),
        (
            "lazada-just4u-product",
            "https://www.lazada.sg/products/pdp-i3117267948-s21200737821.html?pvid=5fb0f00b-fcf4-4ce2-95d1-d21ff9888691&search=jfy&scm=1007.17519.386432.0&priceCompare=skuId%3A21200737821%3Bsource%3Atpp-recommend-plugin-32104%3Bsn%3A5fb0f00b-fcf4-4ce2-95d1-d21ff9888691%3BoriginPrice%3A275%3BdisplayPrice%3A275%3BsinglePromotionId%3A91471183456276%3BsingleToolCode%3ApromPrice%3BvoucherPricePlugin%3A0%3Btimestamp%3A1759169821393&spm=a2o42.homepage.just4u.d_3117267948",
            "Lazada Just4U Product",
        ),
    ];

    // Create API client with appropriate user agent
    let api_client = Arc::new(
        ApiClient::new(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()))?
    );

    // Create proxy manager with Singapore proxies from testacc.txt
    let singapore_proxies = vec![
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-Qsye0dbb_lifetime-168h".to_string()),
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-By0pp71L_lifetime-168h".to_string()),
    ];
    let proxy_manager = Arc::new(ProxyManager::new(singapore_proxies));

    // Create monitor engine
    let mut engine = MonitorEngine::new();

    // Create monitor tasks for each product
    let mut event_receivers = Vec::new();

    for (product_id, product_url, product_name) in test_products {
        info!("📦 Setting up monitor for: {}", product_name);
        
        let monitor = MonitorTask::new(
            product_id.to_string(),
            product_url.to_string(),
            product_name.to_string(),
            api_client.clone(),
            proxy_manager.clone(),
            5000, // Check every 5 seconds
        )
        .with_timeout(15000) // 15 second timeout for real requests
        .with_max_retries(2)
        .with_target_price(100.0) // Set a reasonable target price
        .with_min_stock(1);

        let receiver = engine.add_monitor(monitor);
        event_receivers.push((product_id, product_name, receiver));
    }

    // Start the engine
    engine.start().await?;

    // Spawn event handler
    let event_handle = tokio::spawn(async move {
        for (_product_id, product_name, mut receiver) in event_receivers {
            info!("👀 Starting to monitor events for: {}", product_name);
            
            while let Some(event) = receiver.recv().await {
                info!("📊 Product '{}' availability changed:", product_name);
                info!("   🆔 Product ID: {}", event.product_id);
                info!("   🔗 URL: {}", event.product_url);
                info!("   ⏰ Timestamp: {}", event.timestamp);
                info!("   💰 Price: {:?}", event.price);
                info!("   📦 Stock: {:?}", event.stock);
                info!("   ✅ Available: {}", event.is_available);

                if event.is_available {
                    println!("🟢 Product '{}' is now AVAILABLE!", product_name);
                } else {
                    println!("🔴 Product '{}' is now UNAVAILABLE", product_name);
                }
            }
        }
    });

    // Run monitoring for 60 seconds to test with real products
    info!("⏱️  Running monitor for 60 seconds with real Lazada.sg products...");
    
    // Use timeout to ensure test doesn't run indefinitely
    let test_duration = Duration::from_secs(60);
    let result = timeout(test_duration, async {
        sleep(test_duration).await;
    }).await;

    match result {
        Ok(_) => info!("✅ Test completed successfully"),
        Err(_) => {
            warn!("⏰ Test timed out after 60 seconds");
        }
    }

    // Stop the engine
    info!("🛑 Stopping monitor engine...");
    engine.stop().await?;

    // Wait for event handler to finish
    let _ = event_handle.await;

    info!("🎉 Lazada.sg Monitor Test completed!");
    Ok(())
}

/// Test monitor with a single Lazada.sg product for detailed analysis
#[tokio::test]
async fn test_single_lazada_sg_product_monitoring() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🔍 Starting Single Product Monitor Test");

    // Use one of the real Lazada.sg products
    let product_id = "lazada-flashsale-detailed";
    let product_url = "https://www.lazada.sg/products/pdp-i301072965-s527094858.html?scm=1007.17760.398138.0&pvid=e1e36251-ce9e-4ed1-9053-6ad3b0c7c25c&search=flashsale&spm=a2o42.homepage.FlashSale.d_301072965";
    let product_name = "Lazada Flash Sale Product (Detailed Test)";

    // Create API client
    let api_client = Arc::new(
        ApiClient::new(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()))?
    );

    // Create proxy manager with Singapore proxy
    let singapore_proxy = vec![
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-Qsye0dbb_lifetime-168h".to_string()),
    ];
    let proxy_manager = Arc::new(ProxyManager::new(singapore_proxy));

    // Create monitor task
    let monitor = MonitorTask::new(
        product_id.to_string(),
        product_url.to_string(),
        product_name.to_string(),
        api_client,
        proxy_manager,
        3000, // Check every 3 seconds
    )
    .with_timeout(20000) // 20 second timeout
    .with_max_retries(3)
    .with_target_price(50.0)
    .with_min_stock(1);

    // Get event receiver
    let mut event_receiver = monitor.get_event_receiver();

    // Start monitoring in background
    let monitor_handle = tokio::spawn(async move {
        monitor.run().await
    });

    info!("👀 Monitoring product: {}", product_name);
    info!("🔗 URL: {}", product_url);

    // Monitor for 30 seconds and collect events
    let mut events_received = 0;
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < Duration::from_secs(30) {
        match timeout(Duration::from_secs(5), event_receiver.recv()).await {
            Ok(Some(event)) => {
                events_received += 1;
                info!("📊 Event #{} received:", events_received);
                info!("   🆔 Product ID: {}", event.product_id);
                info!("   🔗 URL: {}", event.product_url);
                info!("   ⏰ Timestamp: {}", event.timestamp);
                info!("   💰 Price: {:?}", event.price);
                info!("   📦 Stock: {:?}", event.stock);
                info!("   ✅ Available: {}", event.is_available);
                
                if event.is_available {
                    println!("🟢 Product is AVAILABLE!");
                } else {
                    println!("🔴 Product is UNAVAILABLE");
                }
            }
            Ok(None) => {
                warn!("📭 No more events available");
                break;
            }
            Err(_) => {
                debug!("⏰ No event received in 5 seconds, continuing...");
            }
        }
    }

    // Stop monitoring
    monitor_handle.abort();
    
    info!("📈 Test Summary:");
    info!("   ⏱️  Duration: {:?}", start_time.elapsed());
    info!("   📊 Events received: {}", events_received);
    info!("   🎯 Product: {}", product_name);

    Ok(())
}

/// Test monitor with different proxy configurations
#[tokio::test]
async fn test_monitor_with_different_proxy_configs() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🌐 Testing Monitor with Different Proxy Configurations");

    let product_id = "lazada-proxy-test";
    let product_url = "https://www.lazada.sg/products/pdp-i301072965-s527094858.html";
    let product_name = "Lazada Proxy Test Product";

    // Test configurations
    let test_configs = vec![
        ("No Proxy", None),
        ("Singapore Proxy 1", Some(ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-Qsye0dbb_lifetime-168h".to_string()))),
        ("Singapore Proxy 2", Some(ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-By0pp71L_lifetime-168h".to_string()))),
    ];

    for (config_name, proxy_info) in test_configs {
        info!("🧪 Testing configuration: {}", config_name);

        // Create API client
        let api_client = Arc::new(
            ApiClient::new(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()))?
        );

        // Create proxy manager
        let proxies = if let Some(proxy) = proxy_info {
            vec![proxy]
        } else {
            vec![]
        };
        let proxy_manager = Arc::new(ProxyManager::new(proxies));

        // Create monitor task
        let monitor = MonitorTask::new(
            format!("{}-{}", product_id, config_name.replace(" ", "-").to_lowercase()),
            product_url.to_string(),
            format!("{} ({})", product_name, config_name),
            api_client,
            proxy_manager,
            2000, // 2 second interval
        )
        .with_timeout(10000) // 10 second timeout
        .with_max_retries(1);

        // Get event receiver
        let mut event_receiver = monitor.get_event_receiver();

        // Start monitoring
        let monitor_handle = tokio::spawn(async move {
            monitor.run().await
        });

        // Wait for a few events or timeout
        let mut events_received = 0;
        let start_time = std::time::Instant::now();

        while start_time.elapsed() < Duration::from_secs(15) && events_received < 3 {
            match timeout(Duration::from_secs(3), event_receiver.recv()).await {
                Ok(Some(event)) => {
                    events_received += 1;
                    info!("   📊 Event #{}: Available={}", events_received, event.is_available);
                }
                Ok(None) => break,
                Err(_) => continue,
            }
        }

        // Stop monitoring
        monitor_handle.abort();

        info!("   ✅ Configuration '{}' completed: {} events received", config_name, events_received);
    }

    info!("🎉 Proxy configuration test completed!");
    Ok(())
}
