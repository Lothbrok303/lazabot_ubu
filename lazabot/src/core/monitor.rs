use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{sleep, interval};
use tokio::task::JoinHandle;
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

use crate::api::ApiClient;
use crate::proxy::ProxyManager;
use crate::core::PerformanceMonitor;

/// Event emitted when a product becomes available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAvailabilityEvent {
    pub product_id: String,
    pub product_url: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub price: Option<f64>,
    pub stock: Option<u32>,
    pub is_available: bool,
}

/// Product information for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInfo {
    pub id: String,
    pub url: String,
    pub name: String,
    pub target_price: Option<f64>,
    pub min_stock: Option<u32>,
}

/// Configuration for a monitor task
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub product: ProductInfo,
    pub interval_ms: u64,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

/// Monitor task that polls a product endpoint and emits events when availability changes
pub struct MonitorTask {
    config: MonitorConfig,
    api_client: Arc<ApiClient>,
    proxy_manager: Arc<ProxyManager>,
    event_sender: mpsc::UnboundedSender<ProductAvailabilityEvent>,
    performance_monitor: PerformanceMonitor,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl MonitorTask {
    /// Create a new monitor task
    pub fn new(
        product_id: String,
        product_url: String,
        product_name: String,
        api_client: Arc<ApiClient>,
        proxy_manager: Arc<ProxyManager>,
        interval_ms: u64,
    ) -> Self {
        let product = ProductInfo {
            id: product_id.clone(),
            url: product_url,
            name: product_name,
            target_price: None,
            min_stock: None,
        };

        let config = MonitorConfig {
            product,
            interval_ms,
            timeout_ms: 30000, // 30 seconds default timeout
            max_retries: 3,
        };

        let (event_sender, _) = mpsc::unbounded_channel();
        let performance_monitor = PerformanceMonitor::new(&format!("monitor_{}", product_id));
        let is_running = Arc::new(tokio::sync::RwLock::new(false));

        Self {
            config,
            api_client,
            proxy_manager,
            event_sender,
            performance_monitor,
            is_running,
        }
    }

    /// Set target price for the product
    pub fn with_target_price(mut self, price: f64) -> Self {
        self.config.product.target_price = Some(price);
        self
    }

    /// Set minimum stock requirement
    pub fn with_min_stock(mut self, stock: u32) -> Self {
        self.config.product.min_stock = Some(stock);
        self
    }

    /// Set timeout for requests
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.config.timeout_ms = timeout_ms;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Get the event receiver for this monitor
    pub fn get_event_receiver(&self) -> mpsc::UnboundedReceiver<ProductAvailabilityEvent> {
        let (_, receiver) = mpsc::unbounded_channel();
        receiver
    }

    /// Start the monitor task
    pub async fn run(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = true;
        drop(is_running);

        info!("Starting monitor for product: {} ({})", self.config.product.name, self.config.product.id);

        let mut interval_timer = interval(Duration::from_millis(self.config.interval_ms));
        let mut last_availability = None;

        loop {
            // Check if we should stop
            {
                let is_running = self.is_running.read().await;
                if !*is_running {
                    info!("Monitor for product {} stopped", self.config.product.id);
                    break;
                }
            }

            interval_timer.tick().await;

            // Perform the check
            match self.check_product_availability().await {
                Ok(current_availability) => {
                    // Check if availability has changed
                    if last_availability != Some(current_availability) {
                        let event = ProductAvailabilityEvent {
                            product_id: self.config.product.id.clone(),
                            product_url: self.config.product.url.clone(),
                            timestamp: chrono::Utc::now(),
                            price: None, // TODO: Extract from response
                            stock: None, // TODO: Extract from response
                            is_available: current_availability,
                        };

                        if let Err(e) = self.event_sender.send(event) {
                            error!("Failed to send availability event: {}", e);
                        }

                        last_availability = Some(current_availability);
                    }
                }
                Err(e) => {
                    warn!("Failed to check product availability for {}: {}", self.config.product.id, e);
                }
            }
        }

        Ok(())
    }

    /// Check if the product is currently available
    async fn check_product_availability(&self) -> Result<bool> {
        let mut monitor = self.performance_monitor.clone();
        monitor.start();

        let result = self.check_with_retry().await;
        monitor.end();

        result
    }

    /// Check product availability with retry logic
    async fn check_with_retry(&self) -> Result<bool> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match self.single_check().await {
                Ok(availability) => {
                    debug!("Product {} check successful (attempt {}): available={}", 
                        self.config.product.id, attempt + 1, availability);
                    return Ok(availability);
                }
                Err(e) => {
                    warn!("Product {} check failed (attempt {}): {}", 
                        self.config.product.id, attempt + 1, e);
                    last_error = Some(e);
                }
            }

            if attempt < self.config.max_retries {
                let delay = Duration::from_millis(1000 * (attempt + 1) as u64);
                debug!("Retrying in {:?}", delay);
                sleep(delay).await;
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed")))
    }

    /// Perform a single availability check
    async fn single_check(&self) -> Result<bool> {
        // Get a proxy for this request
        let proxy = self.proxy_manager.get_next_proxy().await;

        // Make the request
        let response = self.api_client.request(
            reqwest::Method::GET,
            &self.config.product.url,
            None,
            None,
            proxy,
        ).await?;

        // Check if the response indicates availability
        let is_available = self.parse_availability_from_response(&response)?;

        Ok(is_available)
    }

    /// Parse availability information from the HTTP response
    fn parse_availability_from_response(&self, response: &crate::api::ResponseBody) -> Result<bool> {
        // For now, we'll use a simple heuristic: 200 status means available
        // In a real implementation, you'd parse the HTML/JSON response to check:
        // - Stock status
        // - Price information
        // - Add to cart button availability
        // - etc.

        if response.status == 200 {
            // Basic check: look for common "out of stock" indicators in the response
            let body_lower = response.text.to_lowercase();
            let out_of_stock_indicators = [
                "out of stock",
                "sold out",
                "unavailable",
                "not available",
                "temporarily unavailable",
            ];

            let is_out_of_stock = out_of_stock_indicators.iter()
                .any(|indicator| body_lower.contains(indicator));

            Ok(!is_out_of_stock)
        } else {
            Ok(false)
        }
    }

    /// Stop the monitor task
    pub async fn stop(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        info!("Stopping monitor for product: {}", self.config.product.id);
    }
}

/// Monitor engine that manages multiple monitor tasks
pub struct MonitorEngine {
    tasks: Vec<JoinHandle<Result<()>>>,
    event_receivers: Vec<mpsc::UnboundedReceiver<ProductAvailabilityEvent>>,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl MonitorEngine {
    /// Create a new monitor engine
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            event_receivers: Vec::new(),
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Add a monitor task
    pub fn add_monitor(&mut self, monitor: MonitorTask) -> mpsc::UnboundedReceiver<ProductAvailabilityEvent> {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        // Create a new monitor task with the provided sender
        let task = MonitorTask {
            event_sender: sender,
            ..monitor
        };

        let _is_running = self.is_running.clone();
        let task_handle = tokio::spawn(async move {
            task.run().await
        });

        self.tasks.push(task_handle);
        receiver
    }

    /// Start all monitor tasks
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = true;
        info!("Starting monitor engine with {} tasks", self.tasks.len());
        Ok(())
    }

    /// Stop all monitor tasks
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        info!("Stopping monitor engine");

        // Wait for all tasks to complete
        for task in &self.tasks {
            if !task.is_finished() {
                task.abort();
            }
        }

        Ok(())
    }
}

impl Default for MonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ApiClient;

    #[tokio::test]
    async fn test_monitor_task_creation() {
        let api_client = Arc::new(ApiClient::new(None).unwrap());
        let proxy_manager = Arc::new(ProxyManager::new(vec![]));
        
        let monitor = MonitorTask::new(
            "test-product".to_string(),
            "https://example.com/product".to_string(),
            "Test Product".to_string(),
            api_client,
            proxy_manager,
            1000,
        );

        assert_eq!(monitor.config.product.id, "test-product");
        assert_eq!(monitor.config.interval_ms, 1000);
    }

    #[tokio::test]
    async fn test_monitor_engine_creation() {
        let engine = MonitorEngine::new();
        assert_eq!(engine.tasks.len(), 0);
    }
}
