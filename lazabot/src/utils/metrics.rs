//! Lightweight metrics server for exposing operational metrics
//! 
//! This module provides a simple HTTP server that exposes metrics in Prometheus format:
//! - Request counters (total, success, failure)
//! - Request rate (requests per second)
//! - Active tasks counter
//! - Uptime tracking

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, error, warn};

use parking_lot::Mutex;
/// Shared metrics collector
#[derive(Clone)]
pub struct MetricsCollector {
    inner: Arc<MetricsInner>,
}

struct MetricsInner {
    // Counters
    total_requests: AtomicU64,
    success_requests: AtomicU64,
    failed_requests: AtomicU64,
    active_tasks: AtomicUsize,
    
    // Timing
    start_time: Instant,
    
    // Rate tracking
    last_request_count: AtomicU64,
    last_rate_check: Mutex<Instant>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MetricsInner {
                total_requests: AtomicU64::new(0),
                success_requests: AtomicU64::new(0),
                failed_requests: AtomicU64::new(0),
                active_tasks: AtomicUsize::new(0),
                start_time: Instant::now(),
                last_request_count: AtomicU64::new(0),
                last_rate_check: Mutex::new(Instant::now()),
            }),
        }
    }

    /// Increment total request counter
    pub fn inc_total_requests(&self) {
        self.inner.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment success request counter
    pub fn inc_success_requests(&self) {
        self.inner.success_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment failed request counter
    pub fn inc_failed_requests(&self) {
        self.inner.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment active tasks counter
    pub fn inc_active_tasks(&self) {
        self.inner.active_tasks.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active tasks counter
    pub fn dec_active_tasks(&self) {
        self.inner.active_tasks.fetch_sub(1, Ordering::Relaxed);
    }

    /// Set active tasks to a specific value
    pub fn set_active_tasks(&self, count: usize) {
        self.inner.active_tasks.store(count, Ordering::Relaxed);
    }

    /// Get current metrics snapshot
    fn get_snapshot(&self) -> MetricsSnapshot {
        let total = self.inner.total_requests.load(Ordering::Relaxed);
        let success = self.inner.success_requests.load(Ordering::Relaxed);
        let failed = self.inner.failed_requests.load(Ordering::Relaxed);
        let active = self.inner.active_tasks.load(Ordering::Relaxed);
        let uptime = self.inner.start_time.elapsed();

        // Calculate requests per second
        let mut last_check = self.inner.last_rate_check.lock();
        let time_since_last_check = last_check.elapsed();
        let last_count = self.inner.last_request_count.load(Ordering::Relaxed);
        
        let requests_per_sec = if time_since_last_check.as_secs() > 0 {
            let new_requests = total.saturating_sub(last_count);
            (new_requests as f64) / time_since_last_check.as_secs_f64()
        } else {
            0.0
        };

        // Update for next calculation
        if time_since_last_check >= Duration::from_secs(1) {
            self.inner.last_request_count.store(total, Ordering::Relaxed);
            *last_check = Instant::now();
        }

        MetricsSnapshot {
            total_requests: total,
            success_requests: success,
            failed_requests: failed,
            active_tasks: active,
            uptime_seconds: uptime.as_secs(),
            requests_per_sec,
        }
    }

    /// Format metrics in Prometheus format
    fn format_prometheus(&self) -> String {
        let snapshot = self.get_snapshot();
        
        format!(
            "# HELP lazabot_requests_total Total number of requests\n\
             # TYPE lazabot_requests_total counter\n\
             lazabot_requests_total {}\n\
             \n\
             # HELP lazabot_requests_success_total Total number of successful requests\n\
             # TYPE lazabot_requests_success_total counter\n\
             lazabot_requests_success_total {}\n\
             \n\
             # HELP lazabot_requests_failed_total Total number of failed requests\n\
             # TYPE lazabot_requests_failed_total counter\n\
             lazabot_requests_failed_total {}\n\
             \n\
             # HELP lazabot_active_tasks Number of currently active tasks\n\
             # TYPE lazabot_active_tasks gauge\n\
             lazabot_active_tasks {}\n\
             \n\
             # HELP lazabot_requests_per_second Current request rate\n\
             # TYPE lazabot_requests_per_second gauge\n\
             lazabot_requests_per_second {:.2}\n\
             \n\
             # HELP lazabot_uptime_seconds Uptime in seconds\n\
             # TYPE lazabot_uptime_seconds counter\n\
             lazabot_uptime_seconds {}\n",
            snapshot.total_requests,
            snapshot.success_requests,
            snapshot.failed_requests,
            snapshot.active_tasks,
            snapshot.requests_per_sec,
            snapshot.uptime_seconds,
        )
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of current metrics
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub success_requests: u64,
    pub failed_requests: u64,
    pub active_tasks: usize,
    pub uptime_seconds: u64,
    pub requests_per_sec: f64,
}

/// Metrics HTTP server
pub struct MetricsServer {
    collector: MetricsCollector,
    bind_addr: String,
}

impl MetricsServer {
    /// Create a new metrics server
    pub fn new(collector: MetricsCollector, bind_addr: impl Into<String>) -> Self {
        Self {
            collector,
            bind_addr: bind_addr.into(),
        }
    }

    /// Start the metrics server
    pub async fn start(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        info!("Metrics server listening on http://{}/metrics", self.bind_addr);

        loop {
            match listener.accept().await {
                Ok((mut socket, addr)) => {
                    let collector = self.collector.clone();
                    
                    tokio::spawn(async move {
                        let mut buffer = vec![0u8; 1024];
                        
                        match socket.read(&mut buffer).await {
                            Ok(n) if n > 0 => {
                                let request = String::from_utf8_lossy(&buffer[..n]);
                                
                                // Simple HTTP request parsing
                                if request.starts_with("GET /metrics") {
                                    let metrics = collector.format_prometheus();
                                    let response = format!(
                                        "HTTP/1.1 200 OK\r\n\
                                         Content-Type: text/plain; version=0.0.4\r\n\
                                         Content-Length: {}\r\n\
                                         \r\n\
                                         {}",
                                        metrics.len(),
                                        metrics
                                    );
                                    
                                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                                        error!("Failed to write response: {}", e);
                                    }
                                } else if request.starts_with("GET /health") {
                                    let response = "HTTP/1.1 200 OK\r\n\
                                                   Content-Type: text/plain\r\n\
                                                   Content-Length: 2\r\n\
                                                   \r\n\
                                                   OK";
                                    
                                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                                        error!("Failed to write response: {}", e);
                                    }
                                } else {
                                    let response = "HTTP/1.1 404 Not Found\r\n\
                                                   Content-Type: text/plain\r\n\
                                                   Content-Length: 9\r\n\
                                                   \r\n\
                                                   Not Found";
                                    
                                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                                        error!("Failed to write response: {}", e);
                                    }
                                }
                            }
                            Ok(_) => {
                                warn!("Empty request from {}", addr);
                            }
                            Err(e) => {
                                error!("Failed to read from socket: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let collector = MetricsCollector::new();
        
        collector.inc_total_requests();
        collector.inc_success_requests();
        collector.inc_active_tasks();
        
        let snapshot = collector.get_snapshot();
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.success_requests, 1);
        assert_eq!(snapshot.failed_requests, 0);
        assert_eq!(snapshot.active_tasks, 1);
    }

    #[test]
    fn test_prometheus_format() {
        let collector = MetricsCollector::new();
        
        collector.inc_total_requests();
        collector.inc_success_requests();
        
        let output = collector.format_prometheus();
        
        assert!(output.contains("lazabot_requests_total 1"));
        assert!(output.contains("lazabot_requests_success_total 1"));
        assert!(output.contains("lazabot_active_tasks"));
    }

    #[tokio::test]
    async fn test_metrics_server_creation() {
        let collector = MetricsCollector::new();
        let server = MetricsServer::new(collector, "127.0.0.1:9090");
        
        assert_eq!(server.bind_addr, "127.0.0.1:9090");
    }
}
