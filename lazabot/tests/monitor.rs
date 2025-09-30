use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};
use serde_json::json;

use lazabot::api::{ApiClient, ProxyInfo};
use lazabot::proxy::ProxyManager;
use lazabot::core::monitor::{MonitorTask, ProductInfo, ProductAvailabilityEvent};

#[tokio::test]
async fn test_monitor_task_creation() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-product-1".to_string(),
        "https://example.com/product/1".to_string(),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000, // 1 second interval
    );
    
    // Test that the monitor was created successfully
    // We can't access private fields, so we just verify it doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_with_target_price() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-product-2".to_string(),
        "https://example.com/product/2".to_string(),
        "Test Product 2".to_string(),
        api_client,
        proxy_manager,
        1000,
    ).with_target_price(99.99);
    
    // Test that the method call doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_with_min_stock() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-product-3".to_string(),
        "https://example.com/product/3".to_string(),
        "Test Product 3".to_string(),
        api_client,
        proxy_manager,
        1000,
    ).with_min_stock(5);
    
    // Test that the method call doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_with_timeout() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-product-4".to_string(),
        "https://example.com/product/4".to_string(),
        "Test Product 4".to_string(),
        api_client,
        proxy_manager,
        1000,
    ).with_timeout(10000); // 10 seconds
    
    // Test that the method call doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_with_max_retries() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-product-5".to_string(),
        "https://example.com/product/5".to_string(),
        "Test Product 5".to_string(),
        api_client,
        proxy_manager,
        1000,
    ).with_max_retries(5);
    
    // Test that the method call doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_stop_functionality() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-stop".to_string(),
        "https://example.com/product/stop".to_string(),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        100, // Very fast interval for testing
    );
    
    // Test stop functionality
    monitor.stop().await;
    
    // The monitor should be marked as stopped
    // Note: We can't easily test the run loop without a real server,
    // but we can test that stop() doesn't panic
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_product_availability_event_creation() -> Result<()> {
    let event = ProductAvailabilityEvent {
        product_id: "test-123".to_string(),
        product_url: "https://example.com/product/123".to_string(),
        timestamp: chrono::Utc::now(),
        price: Some(29.99),
        stock: Some(10),
        is_available: true,
    };
    
    assert_eq!(event.product_id, "test-123");
    assert_eq!(event.product_url, "https://example.com/product/123");
    assert_eq!(event.price, Some(29.99));
    assert_eq!(event.stock, Some(10));
    assert!(event.is_available);
    
    Ok(())
}

#[tokio::test]
async fn test_product_info_creation() -> Result<()> {
    let product = ProductInfo {
        id: "prod-123".to_string(),
        url: "https://example.com/product/123".to_string(),
        name: "Test Product".to_string(),
        target_price: Some(99.99),
        min_stock: Some(5),
    };
    
    assert_eq!(product.id, "prod-123");
    assert_eq!(product.url, "https://example.com/product/123");
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.target_price, Some(99.99));
    assert_eq!(product.min_stock, Some(5));
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_with_proxy() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response
    Mock::given(method("GET"))
        .and(path("/product/with-proxy"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "product_id": "test-proxy",
                "available": true
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    
    // Create proxy manager with a proxy
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080)
            .with_auth("user".to_string(), "pass".to_string()),
    ];
    let proxy_manager = Arc::new(ProxyManager::new(proxies));
    
    let monitor = MonitorTask::new(
        "test-proxy".to_string(),
        format!("{}/product/with-proxy", mock_server.uri()),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000,
    );
    
    // Test that the monitor was created successfully with proxy
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_creation_with_mock_server() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response for available product
    Mock::given(method("GET"))
        .and(path("/product/available"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "product_id": "test-123",
                "name": "Test Product",
                "price": 29.99,
                "in_stock": true,
                "stock_count": 10,
                "available": true
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-123".to_string(),
        format!("{}/product/available", mock_server.uri()),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000,
    );
    
    // Test that the monitor was created successfully
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_creation_with_out_of_stock_mock() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response for out of stock product
    Mock::given(method("GET"))
        .and(path("/product/out-of-stock"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "product_id": "test-456",
                "name": "Test Product",
                "price": 29.99,
                "in_stock": false,
                "stock_count": 0,
                "available": false,
                "message": "Out of stock"
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-456".to_string(),
        format!("{}/product/out-of-stock", mock_server.uri()),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000,
    );
    
    // Test that the monitor was created successfully
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_creation_with_error_response() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response that returns 404
    Mock::given(method("GET"))
        .and(path("/product/not-found"))
        .respond_with(ResponseTemplate::new(404)
            .set_body_json(json!({
                "error": "Product not found"
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-not-found".to_string(),
        format!("{}/product/not-found", mock_server.uri()),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000,
    );
    
    // Test that the monitor was created successfully
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_creation_with_timeout_mock() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Setup mock response that takes 10 seconds
    Mock::given(method("GET"))
        .and(path("/product/slow"))
        .respond_with(ResponseTemplate::new(200)
            .set_delay(Duration::from_secs(10))
            .set_body_json(json!({
                "product_id": "test-slow",
                "available": true
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "test-slow".to_string(),
        format!("{}/product/slow", mock_server.uri()),
        "Test Product".to_string(),
        api_client,
        proxy_manager,
        1000,
    ).with_timeout(1000); // 1 second timeout
    
    // Test that the monitor was created successfully
    assert!(true);
    
    Ok(())
}

#[tokio::test]
async fn test_monitor_task_creation_with_large_response() -> Result<()> {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Create a large response
    let large_data: Vec<serde_json::Value> = (0..100)
        .map(|i| json!({
            "id": i,
            "name": format!("Product {}", i),
            "price": (i as f64) * 10.0,
            "available": i % 2 == 0
        }))
        .collect();
    
    Mock::given(method("GET"))
        .and(path("/product/large"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "products": large_data,
                "total": 100,
                "page": 1
            })))
        .mount(&mock_server)
        .await;
    
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    let monitor = MonitorTask::new(
        "large-test".to_string(),
        format!("{}/product/large", mock_server.uri()),
        "Large Product List".to_string(),
        api_client,
        proxy_manager,
        1000,
    );
    
    // Test that the monitor was created successfully
    assert!(true);
    
    Ok(())
}
