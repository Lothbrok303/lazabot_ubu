use crate::api::ProxyInfo;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Thread-safe proxy manager with round-robin selection and health tracking
#[derive(Debug)]
pub struct ProxyManager {
    /// List of available proxies
    proxies: Vec<ProxyInfo>,
    /// Current index for round-robin selection
    current_index: AtomicUsize,
    /// Health status of each proxy (proxy_id -> is_healthy)
    health_status: Arc<RwLock<HashMap<String, bool>>>,
    /// Total number of proxies
    total_proxies: usize,
}

impl ProxyManager {
    /// Create a new ProxyManager by loading proxies from a file
    pub async fn from_file(file_path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(file_path)
            .await
            .context("Failed to read proxy file")?;

        let proxies = Self::parse_proxies(&content)?;

        if proxies.is_empty() {
            return Err(anyhow::anyhow!("No valid proxies found in file"));
        }

        let total_proxies = proxies.len();
        let health_status = Arc::new(RwLock::new(HashMap::new()));

        // Initialize all proxies as healthy
        {
            let mut status = health_status.write().await;
            for proxy in &proxies {
                let proxy_id = format!("{}:{}", proxy.host, proxy.port);
                status.insert(proxy_id, true);
            }
        }

        info!("Loaded {} proxies from {}", total_proxies, file_path);

        Ok(Self {
            proxies,
            current_index: AtomicUsize::new(0),
            health_status,
            total_proxies,
        })
    }

    /// Create a new ProxyManager with a list of proxies
    pub fn new(proxies: Vec<ProxyInfo>) -> Self {
        let total_proxies = proxies.len();
        let health_status = Arc::new(RwLock::new(HashMap::new()));

        // Initialize all proxies as healthy
        {
            let mut status = futures::executor::block_on(health_status.write());
            for proxy in &proxies {
                let proxy_id = format!("{}:{}", proxy.host, proxy.port);
                status.insert(proxy_id, true);
            }
        }

        Self {
            proxies,
            current_index: AtomicUsize::new(0),
            health_status,
            total_proxies,
        }
    }

    /// Get the next available proxy using round-robin selection
    /// Only returns healthy proxies
    pub async fn get_next_proxy(&self) -> Option<ProxyInfo> {
        if self.total_proxies == 0 {
            return None;
        }

        let mut attempts = 0;
        let max_attempts = self.total_proxies;

        while attempts < max_attempts {
            let current_idx =
                self.current_index.fetch_add(1, Ordering::Relaxed) % self.total_proxies;
            let proxy = &self.proxies[current_idx];
            let proxy_id = format!("{}:{}", proxy.host, proxy.port);

            // Check if this proxy is healthy
            {
                let status = self.health_status.read().await;
                if status.get(&proxy_id).copied().unwrap_or(false) {
                    debug!("Selected proxy: {}:{}", proxy.host, proxy.port);
                    return Some(proxy.clone());
                }
            }

            attempts += 1;
        }

        warn!("No healthy proxies available");
        None
    }

    /// Get a specific proxy by index
    pub fn get_proxy_by_index(&self, index: usize) -> Option<&ProxyInfo> {
        self.proxies.get(index)
    }

    /// Mark a proxy as healthy or unhealthy
    pub async fn set_proxy_health(&self, proxy: &ProxyInfo, is_healthy: bool) {
        let proxy_id = format!("{}:{}", proxy.host, proxy.port);
        let mut status = self.health_status.write().await;
        status.insert(proxy_id, is_healthy);

        if is_healthy {
            debug!("Marked proxy {}:{} as healthy", proxy.host, proxy.port);
        } else {
            warn!("Marked proxy {}:{} as unhealthy", proxy.host, proxy.port);
        }
    }

    /// Get health status of a specific proxy
    pub async fn is_proxy_healthy(&self, proxy: &ProxyInfo) -> bool {
        let proxy_id = format!("{}:{}", proxy.host, proxy.port);
        let status = self.health_status.read().await;
        status.get(&proxy_id).copied().unwrap_or(false)
    }

    /// Get all healthy proxies
    pub async fn get_healthy_proxies(&self) -> Vec<ProxyInfo> {
        let status = self.health_status.read().await;
        self.proxies
            .iter()
            .filter(|proxy| {
                let proxy_id = format!("{}:{}", proxy.host, proxy.port);
                status.get(&proxy_id).copied().unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Get all proxies (regardless of health status)
    pub fn get_all_proxies(&self) -> &[ProxyInfo] {
        &self.proxies
    }

    /// Get total number of proxies
    pub fn total_proxies(&self) -> usize {
        self.total_proxies
    }

    /// Get number of healthy proxies
    pub async fn healthy_proxies_count(&self) -> usize {
        let status = self.health_status.read().await;
        status.values().filter(|&&is_healthy| is_healthy).count()
    }

    /// Reset all proxies to healthy status
    pub async fn reset_all_health(&self) {
        let mut status = self.health_status.write().await;
        for proxy in &self.proxies {
            let proxy_id = format!("{}:{}", proxy.host, proxy.port);
            status.insert(proxy_id, true);
        }
        info!("Reset all proxies to healthy status");
    }

    /// Parse proxy list from file content
    fn parse_proxies(content: &str) -> Result<Vec<ProxyInfo>> {
        let mut proxies = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse proxy format: host:port or host:port:username:password
            let parts: Vec<&str> = line.split(':').collect();

            match parts.len() {
                2 => {
                    // Format: host:port
                    let host = parts[0].to_string();
                    let port = parts[1]
                        .parse::<u16>()
                        .context(format!("Invalid port number on line {}", line_num + 1))?;

                    proxies.push(ProxyInfo::new(host, port));
                }
                4 => {
                    // Format: host:port:username:password
                    let host = parts[0].to_string();
                    let port = parts[1]
                        .parse::<u16>()
                        .context(format!("Invalid port number on line {}", line_num + 1))?;
                    let username = parts[2].to_string();
                    let password = parts[3].to_string();

                    proxies.push(ProxyInfo::new(host, port).with_auth(username, password));
                }
                _ => {
                    warn!("Invalid proxy format on line {}: {}", line_num + 1, line);
                    continue;
                }
            }
        }

        Ok(proxies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proxy_manager_creation() {
        let proxies = vec![
            ProxyInfo::new("127.0.0.1".to_string(), 8080),
            ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ];

        let manager = ProxyManager::new(proxies);
        assert_eq!(manager.total_proxies(), 2);
    }

    #[tokio::test]
    async fn test_round_robin_selection() {
        let proxies = vec![
            ProxyInfo::new("127.0.0.1".to_string(), 8080),
            ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ];

        let manager = ProxyManager::new(proxies);

        // First call should return first proxy
        let proxy1 = manager.get_next_proxy().await.unwrap();
        assert_eq!(proxy1.host, "127.0.0.1");
        assert_eq!(proxy1.port, 8080);

        // Second call should return second proxy
        let proxy2 = manager.get_next_proxy().await.unwrap();
        assert_eq!(proxy2.host, "192.168.1.1");
        assert_eq!(proxy2.port, 3128);

        // Third call should wrap around to first proxy
        let proxy3 = manager.get_next_proxy().await.unwrap();
        assert_eq!(proxy3.host, "127.0.0.1");
        assert_eq!(proxy3.port, 8080);
    }

    #[tokio::test]
    async fn test_proxy_health_management() {
        let proxies = vec![
            ProxyInfo::new("127.0.0.1".to_string(), 8080),
            ProxyInfo::new("192.168.1.1".to_string(), 3128),
        ];

        let manager = ProxyManager::new(proxies);

        // Mark first proxy as unhealthy
        let proxy1 = &manager.proxies[0];
        manager.set_proxy_health(proxy1, false).await;

        // Should only return healthy proxies
        let healthy_proxies = manager.get_healthy_proxies().await;
        assert_eq!(healthy_proxies.len(), 1);
        assert_eq!(healthy_proxies[0].host, "192.168.1.1");
    }

    #[test]
    fn test_parse_proxies() {
        let content =
            "127.0.0.1:8080\n192.168.1.1:3128\n# This is a comment\n10.0.0.1:8080:user:pass";

        let proxies = ProxyManager::parse_proxies(content).unwrap();
        assert_eq!(proxies.len(), 3);

        // Check first proxy (no auth)
        assert_eq!(proxies[0].host, "127.0.0.1");
        assert_eq!(proxies[0].port, 8080);
        assert!(proxies[0].username.is_none());

        // Check second proxy (no auth)
        assert_eq!(proxies[1].host, "192.168.1.1");
        assert_eq!(proxies[1].port, 3128);
        assert!(proxies[1].username.is_none());

        // Check third proxy (with auth)
        assert_eq!(proxies[2].host, "10.0.0.1");
        assert_eq!(proxies[2].port, 8080);
        assert_eq!(proxies[2].username, Some("user".to_string()));
        assert_eq!(proxies[2].password, Some("pass".to_string()));
    }
}
