use anyhow::Result;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use lazabot::api::{ApiClient, ProxyInfo};
use lazabot::proxy::ProxyManager;

#[tokio::test]
async fn test_proxy_manager_creation() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = ProxyManager::new(proxies);

    assert_eq!(manager.total_proxies(), 3);
    assert_eq!(manager.healthy_proxies_count().await, 3);

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_round_robin() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = ProxyManager::new(proxies);

    // Test round-robin selection
    let proxy1 = manager.get_next_proxy().await.unwrap();
    assert_eq!(proxy1.host, "127.0.0.1");
    assert_eq!(proxy1.port, 8080);

    let proxy2 = manager.get_next_proxy().await.unwrap();
    assert_eq!(proxy2.host, "192.168.1.1");
    assert_eq!(proxy2.port, 3128);

    let proxy3 = manager.get_next_proxy().await.unwrap();
    assert_eq!(proxy3.host, "10.0.0.1");
    assert_eq!(proxy3.port, 8080);

    // Should wrap around to first proxy
    let proxy4 = manager.get_next_proxy().await.unwrap();
    assert_eq!(proxy4.host, "127.0.0.1");
    assert_eq!(proxy4.port, 8080);

    Ok(())
}

#[tokio::test]
async fn test_proxy_health_management() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = ProxyManager::new(proxies);

    // Initially all proxies should be healthy
    assert_eq!(manager.healthy_proxies_count().await, 3);

    // Mark first proxy as unhealthy
    let proxy1 = &manager.get_all_proxies()[0];
    manager.set_proxy_health(proxy1, false).await;

    assert_eq!(manager.healthy_proxies_count().await, 2);
    assert!(!manager.is_proxy_healthy(proxy1).await);

    // Mark second proxy as unhealthy
    let proxy2 = &manager.get_all_proxies()[1];
    manager.set_proxy_health(proxy2, false).await;

    assert_eq!(manager.healthy_proxies_count().await, 1);

    // Only third proxy should be returned
    let healthy_proxy = manager.get_next_proxy().await.unwrap();
    assert_eq!(healthy_proxy.host, "10.0.0.1");
    assert_eq!(healthy_proxy.port, 8080);

    // Mark first proxy as healthy again
    manager.set_proxy_health(proxy1, true).await;
    assert_eq!(manager.healthy_proxies_count().await, 2);

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_no_healthy_proxies() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
    ];

    let manager = ProxyManager::new(proxies);

    // Mark all proxies as unhealthy
    for proxy in manager.get_all_proxies() {
        manager.set_proxy_health(proxy, false).await;
    }

    assert_eq!(manager.healthy_proxies_count().await, 0);

    // Should return None when no healthy proxies available
    let result = manager.get_next_proxy().await;
    assert!(result.is_none());

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_reset_health() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
    ];

    let manager = ProxyManager::new(proxies);

    // Mark all proxies as unhealthy
    for proxy in manager.get_all_proxies() {
        manager.set_proxy_health(proxy, false).await;
    }

    assert_eq!(manager.healthy_proxies_count().await, 0);

    // Reset all to healthy
    manager.reset_all_health().await;
    assert_eq!(manager.healthy_proxies_count().await, 2);

    // Should be able to get proxies again
    let proxy = manager.get_next_proxy().await.unwrap();
    assert!(proxy.host == "127.0.0.1" || proxy.host == "192.168.1.1");

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_get_healthy_proxies() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = ProxyManager::new(proxies);

    // Initially all should be healthy
    let healthy_proxies = manager.get_healthy_proxies().await;
    assert_eq!(healthy_proxies.len(), 3);

    // Mark one as unhealthy
    let proxy1 = &manager.get_all_proxies()[0];
    manager.set_proxy_health(proxy1, false).await;

    let healthy_proxies = manager.get_healthy_proxies().await;
    assert_eq!(healthy_proxies.len(), 2);

    // Verify the unhealthy proxy is not in the list
    let proxy1_id = format!("{}:{}", proxy1.host, proxy1.port);
    for proxy in &healthy_proxies {
        let proxy_id = format!("{}:{}", proxy.host, proxy.port);
        assert_ne!(proxy_id, proxy1_id);
    }

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_get_proxy_by_index() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = ProxyManager::new(proxies);

    // Test valid indices
    let proxy0 = manager.get_proxy_by_index(0).unwrap();
    assert_eq!(proxy0.host, "127.0.0.1");
    assert_eq!(proxy0.port, 8080);

    let proxy1 = manager.get_proxy_by_index(1).unwrap();
    assert_eq!(proxy1.host, "192.168.1.1");
    assert_eq!(proxy1.port, 3128);

    let proxy2 = manager.get_proxy_by_index(2).unwrap();
    assert_eq!(proxy2.host, "10.0.0.1");
    assert_eq!(proxy2.port, 8080);

    // Test invalid index
    let invalid_proxy = manager.get_proxy_by_index(10);
    assert!(invalid_proxy.is_none());

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_with_auth_proxies() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080)
            .with_auth("user1".to_string(), "pass1".to_string()),
        ProxyInfo::new("192.168.1.1".to_string(), 3128)
            .with_auth("user2".to_string(), "pass2".to_string()),
        ProxyInfo::new("10.0.0.1".to_string(), 8080), // No auth
    ];

    let manager = ProxyManager::new(proxies);

    assert_eq!(manager.total_proxies(), 3);
    assert_eq!(manager.healthy_proxies_count().await, 3);

    // Test that proxies with auth are handled correctly
    let proxy1 = manager.get_next_proxy().await.unwrap();
    assert!(
        proxy1.host == "127.0.0.1" || proxy1.host == "192.168.1.1" || proxy1.host == "10.0.0.1"
    );

    // Test URL generation for auth proxies
    let auth_proxy = ProxyInfo::new("test.example.com".to_string(), 8080)
        .with_auth("testuser".to_string(), "testpass".to_string());

    let url = auth_proxy.to_url()?;
    assert_eq!(url, "http://testuser:testpass@test.example.com:8080");

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_concurrent_access() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ProxyInfo::new("10.0.0.1".to_string(), 8080),
    ];

    let manager = Arc::new(ProxyManager::new(proxies));

    // Spawn multiple tasks that concurrently access the proxy manager
    let mut handles = vec![];

    for i in 0..10 {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..5 {
                let proxy = manager_clone.get_next_proxy().await;
                assert!(proxy.is_some());
                // Simulate some work
                sleep(Duration::from_millis(10)).await;
            }
            i
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await?;
        assert!(result < 10);
    }

    Ok(())
}

#[tokio::test]
async fn test_proxy_manager_health_status_persistence() -> Result<()> {
    let proxies = vec![
        ProxyInfo::new("127.0.0.1".to_string(), 8080),
        ProxyInfo::new("192.168.1.1".to_string(), 3128),
    ];

    let manager = ProxyManager::new(proxies);

    // Mark first proxy as unhealthy
    let proxy1 = &manager.get_all_proxies()[0];
    manager.set_proxy_health(proxy1, false).await;

    // Verify the status persists
    assert!(!manager.is_proxy_healthy(proxy1).await);
    assert_eq!(manager.healthy_proxies_count().await, 1);

    // Mark second proxy as unhealthy
    let proxy2 = &manager.get_all_proxies()[1];
    manager.set_proxy_health(proxy2, false).await;

    assert!(!manager.is_proxy_healthy(proxy2).await);
    assert_eq!(manager.healthy_proxies_count().await, 0);

    // Mark first proxy as healthy again
    manager.set_proxy_health(proxy1, true).await;
    assert!(manager.is_proxy_healthy(proxy1).await);
    assert_eq!(manager.healthy_proxies_count().await, 1);

    Ok(())
}
