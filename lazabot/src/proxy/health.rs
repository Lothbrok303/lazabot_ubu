use super::manager::ProxyManager;
use crate::api::{ApiClient, ProxyInfo};
use anyhow::Result;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// Proxy health checker that tests proxies against httpbin.org/ip
#[derive(Debug)]
pub struct ProxyHealth {
    /// HTTP client for health checks
    client: ApiClient,
    /// Timeout for health check requests
    timeout_duration: Duration,
    /// Test URL for health checks
    test_url: String,
}

impl ProxyHealth {
    /// Create a new ProxyHealth checker
    pub fn new() -> Result<Self> {
        let client = ApiClient::new(Some("Lazabot-HealthCheck/1.0".to_string()))?;

        Ok(Self {
            client,
            timeout_duration: Duration::from_secs(10),
            test_url: "https://httpbin.org/ip".to_string(),
        })
    }

    /// Create a new ProxyHealth checker with custom timeout
    pub fn with_timeout(timeout_duration: Duration) -> Result<Self> {
        let client = ApiClient::new(Some("Lazabot-HealthCheck/1.0".to_string()))?;

        Ok(Self {
            client,
            timeout_duration,
            test_url: "https://httpbin.org/ip".to_string(),
        })
    }

    /// Check the health of a single proxy
    pub async fn check_proxy_health(&self, proxy: &ProxyInfo) -> bool {
        debug!("Checking health of proxy {}:{}", proxy.host, proxy.port);

        let result = timeout(
            self.timeout_duration,
            self.client.request(
                reqwest::Method::GET,
                &self.test_url,
                None,
                None,
                Some(proxy.clone()),
            ),
        )
        .await;

        match result {
            Ok(Ok(response)) => {
                if response.status == 200 {
                    debug!(
                        "Proxy {}:{} is healthy (status: {})",
                        proxy.host, proxy.port, response.status
                    );
                    true
                } else {
                    warn!(
                        "Proxy {}:{} returned non-200 status: {}",
                        proxy.host, proxy.port, response.status
                    );
                    false
                }
            }
            Ok(Err(e)) => {
                warn!(
                    "Proxy {}:{} health check failed: {}",
                    proxy.host, proxy.port, e
                );
                false
            }
            Err(_) => {
                warn!(
                    "Proxy {}:{} health check timed out after {:?}",
                    proxy.host, proxy.port, self.timeout_duration
                );
                false
            }
        }
    }

    /// Check health of all proxies in the manager
    pub async fn check_all_proxies(&self, manager: &ProxyManager) -> Result<()> {
        let proxies = manager.get_all_proxies();
        info!("Starting health check for {} proxies", proxies.len());

        let mut healthy_count = 0;
        let mut unhealthy_count = 0;

        for proxy in proxies {
            let is_healthy = self.check_proxy_health(proxy).await;

            if is_healthy {
                healthy_count += 1;
            } else {
                unhealthy_count += 1;
            }

            // Update the manager with the health status
            manager.set_proxy_health(proxy, is_healthy).await;
        }

        info!(
            "Health check completed: {} healthy, {} unhealthy",
            healthy_count, unhealthy_count
        );
        Ok(())
    }

    /// Check health of only healthy proxies (for periodic maintenance)
    pub async fn check_healthy_proxies(&self, manager: &ProxyManager) -> Result<()> {
        let healthy_proxies = manager.get_healthy_proxies().await;
        info!(
            "Re-checking health of {} currently healthy proxies",
            healthy_proxies.len()
        );

        let mut still_healthy = 0;
        let mut now_unhealthy = 0;

        for proxy in healthy_proxies {
            let is_healthy = self.check_proxy_health(&proxy).await;

            if is_healthy {
                still_healthy += 1;
            } else {
                now_unhealthy += 1;
            }

            // Update the manager with the health status
            manager.set_proxy_health(&proxy, is_healthy).await;
        }

        info!(
            "Re-check completed: {} still healthy, {} now unhealthy",
            still_healthy, now_unhealthy
        );
        Ok(())
    }

    /// Check health of only unhealthy proxies (for recovery detection)
    pub async fn check_unhealthy_proxies(&self, manager: &ProxyManager) -> Result<()> {
        let all_proxies = manager.get_all_proxies();
        let mut unhealthy_proxies = Vec::new();

        // Find currently unhealthy proxies
        for proxy in all_proxies {
            if !manager.is_proxy_healthy(proxy).await {
                unhealthy_proxies.push(proxy.clone());
            }
        }

        info!(
            "Re-checking health of {} currently unhealthy proxies",
            unhealthy_proxies.len()
        );

        let mut still_unhealthy = 0;
        let mut now_healthy = 0;

        for proxy in unhealthy_proxies {
            let is_healthy = self.check_proxy_health(&proxy).await;

            if is_healthy {
                now_healthy += 1;
            } else {
                still_unhealthy += 1;
            }

            // Update the manager with the health status
            manager.set_proxy_health(&proxy, is_healthy).await;
        }

        info!(
            "Recovery check completed: {} now healthy, {} still unhealthy",
            now_healthy, still_unhealthy
        );
        Ok(())
    }

    /// Run a comprehensive health check with detailed reporting
    pub async fn run_comprehensive_check(&self, manager: &ProxyManager) -> Result<HealthReport> {
        let proxies = manager.get_all_proxies();
        info!(
            "Running comprehensive health check for {} proxies",
            proxies.len()
        );

        let mut report = HealthReport {
            total_proxies: proxies.len(),
            healthy_proxies: 0,
            unhealthy_proxies: 0,
            healthy_list: Vec::new(),
            unhealthy_list: Vec::new(),
            check_duration: Duration::from_secs(0),
        };

        let start_time = std::time::Instant::now();

        for proxy in proxies {
            let is_healthy = self.check_proxy_health(proxy).await;

            if is_healthy {
                report.healthy_proxies += 1;
                report
                    .healthy_list
                    .push(format!("{}:{}", proxy.host, proxy.port));
            } else {
                report.unhealthy_proxies += 1;
                report
                    .unhealthy_list
                    .push(format!("{}:{}", proxy.host, proxy.port));
            }

            // Update the manager with the health status
            manager.set_proxy_health(proxy, is_healthy).await;
        }

        report.check_duration = start_time.elapsed();

        info!(
            "Comprehensive health check completed in {:?}",
            report.check_duration
        );
        info!(
            "Results: {} healthy, {} unhealthy",
            report.healthy_proxies, report.unhealthy_proxies
        );

        Ok(report)
    }

    /// Set a custom test URL for health checks
    pub fn set_test_url(&mut self, url: String) {
        self.test_url = url;
    }

    /// Set a custom timeout for health checks
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout_duration = timeout;
    }
}

/// Health check report with detailed results
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub total_proxies: usize,
    pub healthy_proxies: usize,
    pub unhealthy_proxies: usize,
    pub healthy_list: Vec<String>,
    pub unhealthy_list: Vec<String>,
    pub check_duration: Duration,
}

impl HealthReport {
    /// Print a formatted health report
    pub fn print_report(&self) {
        println!("\n=== Proxy Health Report ===");
        println!("Total proxies: {}", self.total_proxies);
        println!(
            "Healthy: {} ({:.1}%)",
            self.healthy_proxies,
            (self.healthy_proxies as f64 / self.total_proxies as f64) * 100.0
        );
        println!(
            "Unhealthy: {} ({:.1}%)",
            self.unhealthy_proxies,
            (self.unhealthy_proxies as f64 / self.total_proxies as f64) * 100.0
        );
        println!("Check duration: {:?}", self.check_duration);

        if !self.healthy_list.is_empty() {
            println!("\nHealthy proxies:");
            for proxy in &self.healthy_list {
                println!("  ✓ {}", proxy);
            }
        }

        if !self.unhealthy_list.is_empty() {
            println!("\nUnhealthy proxies:");
            for proxy in &self.unhealthy_list {
                println!("  ✗ {}", proxy);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = ProxyHealth::new();
        assert!(checker.is_ok());
    }

    #[tokio::test]
    async fn test_health_checker_with_timeout() {
        let timeout = Duration::from_secs(5);
        let checker = ProxyHealth::with_timeout(timeout);
        assert!(checker.is_ok());

        let checker = checker.unwrap();
        assert_eq!(checker.timeout_duration, timeout);
    }

    #[tokio::test]
    async fn test_health_report() {
        let report = HealthReport {
            total_proxies: 4,
            healthy_proxies: 3,
            unhealthy_proxies: 1,
            healthy_list: vec![
                "127.0.0.1:8080".to_string(),
                "192.168.1.1:3128".to_string(),
                "10.0.0.1:8080".to_string(),
            ],
            unhealthy_list: vec!["bad.proxy.com:8080".to_string()],
            check_duration: Duration::from_millis(500),
        };

        assert_eq!(report.total_proxies, 4);
        assert_eq!(report.healthy_proxies, 3);
        assert_eq!(report.unhealthy_proxies, 1);
        assert_eq!(report.healthy_list.len(), 3);
        assert_eq!(report.unhealthy_list.len(), 1);
    }
}
