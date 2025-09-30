use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

use lazabot::api::{ApiClient, ProxyInfo};
use lazabot::core::{MonitorEngine, MonitorTask};
use lazabot::proxy::ProxyManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Starting Lazada.sg Monitor Example with Real Products");

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

    // Create API client with appropriate user agent for Lazada.sg
    let api_client = Arc::new(
        ApiClient::new(Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()))?
    );

    // Create proxy manager with Singapore proxies from testacc.txt
    let singapore_proxies = vec![
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-Qsye0dbb_lifetime-168h".to_string()),
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-By0pp71L_lifetime-168h".to_string()),
        ProxyInfo::new("geo.iproyal.com".to_string(), 12321)
            .with_auth("sakongsa".to_string(), "sakongsa303_country-sg_city-singapore_session-nc5KSQ1H_lifetime-168h".to_string()),
    ];
    let proxy_manager = Arc::new(ProxyManager::new(singapore_proxies));

    // Create monitor engine
    let mut engine = MonitorEngine::new();

    // Create monitor tasks for each product
    let mut event_receivers = Vec::new();

    for (product_id, product_url, product_name) in test_products {
        info!("📦 Setting up monitor for: {}", product_name);
        info!("🔗 URL: {}", product_url);
        
        let monitor = MonitorTask::new(
            product_id.to_string(),
            product_url.to_string(),
            product_name.to_string(),
            api_client.clone(),
            proxy_manager.clone(),
            10000, // Check every 10 seconds
        )
        .with_timeout(30000) // 30 second timeout for real requests
        .with_max_retries(3)
        .with_target_price(100.0) // Set a reasonable target price
        .with_min_stock(1);

        let receiver = engine.add_monitor(monitor);
        event_receivers.push((product_id, product_name, receiver));
    }

    // Start the engine
    engine.start().await?;

    // Spawn event handler
    let event_handle = tokio::spawn(async move {
        for (product_id, product_name, mut receiver) in event_receivers {
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
                    println!("   💰 Price: {:?}", event.price);
                    println!("   📦 Stock: {:?}", event.stock);
                } else {
                    println!("🔴 Product '{}' is now UNAVAILABLE", product_name);
                }
            }
        }
    });

    // Run monitoring for 5 minutes to test with real products
    info!("⏱️  Running monitor for 5 minutes with real Lazada.sg products...");
    info!("Press Ctrl+C to stop monitoring early");
    
    // Run for 5 minutes
    sleep(Duration::from_secs(300)).await;

    // Stop the engine
    info!("🛑 Stopping monitor engine...");
    engine.stop().await?;

    // Wait for event handler to finish
    let _ = event_handle.await;

    info!("🎉 Lazada.sg Monitor Example completed!");
    Ok(())
}
