use anyhow::{anyhow, Context, Result};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::api::ApiClient;
use crate::captcha::CaptchaSolverTrait;
use crate::config::AccountSettings;
use crate::core::Session;

/// Product information for checkout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub url: String,
    pub price: Option<f64>,
    pub quantity: u32,
}

impl Product {
    pub fn new(id: String, name: String, url: String) -> Self {
        Self {
            id,
            name,
            url,
            price: None,
            quantity: 1,
        }
    }

    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }
}

/// Account information for checkout
#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub settings: AccountSettings,
}

/// Checkout errors
#[derive(Debug, thiserror::Error)]
pub enum CheckoutError {
    #[error("Add to cart failed: {0}")]
    AddToCartFailed(String),

    #[error("Checkout URL retrieval failed: {0}")]
    CheckoutUrlFailed(String),

    #[error("Shipping info update failed: {0}")]
    ShippingFailed(String),

    #[error("Payment selection failed: {0}")]
    PaymentFailed(String),

    #[error("Captcha detection failed: {0}")]
    CaptchaDetectionFailed(String),

    #[error("Captcha solving failed: {0}")]
    CaptchaSolvingFailed(String),

    #[error("Order submission failed: {0}")]
    OrderSubmissionFailed(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Session expired")]
    SessionExpired,

    #[error("Product unavailable")]
    ProductUnavailable,

    #[error("Other error: {0}")]
    Other(String),
}

/// Result of a checkout attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResult {
    pub success: bool,
    pub order_id: Option<String>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
}

impl CheckoutResult {
    pub fn success(order_id: String, duration_ms: u64) -> Self {
        Self {
            success: true,
            order_id: Some(order_id),
            error: None,
            timestamp: chrono::Utc::now(),
            duration_ms,
        }
    }

    pub fn failure(error: String, duration_ms: u64) -> Self {
        Self {
            success: false,
            order_id: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
            duration_ms,
        }
    }
}

/// Configuration for checkout process
#[derive(Debug, Clone)]
pub struct CheckoutConfig {
    pub add_to_cart_retries: u32,
    pub checkout_url_retries: u32,
    pub payment_retries: u32,
    pub submission_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub captcha_timeout_secs: u64,
}

impl Default for CheckoutConfig {
    fn default() -> Self {
        Self {
            add_to_cart_retries: 3,
            checkout_url_retries: 2,
            payment_retries: 2,
            submission_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            captcha_timeout_secs: 120,
        }
    }
}

/// Response from add-to-cart API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddToCartResponse {
    success: bool,
    cart_id: Option<String>,
    message: Option<String>,
}

/// Response from checkout URL API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckoutUrlResponse {
    checkout_url: Option<String>,
    token: Option<String>,
}

/// Response from captcha detection
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CaptchaDetectionResponse {
    has_captcha: bool,
    captcha_type: Option<String>,
    site_key: Option<String>,
    page_url: Option<String>,
}

/// Response from order submission
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderSubmissionResponse {
    success: bool,
    order_id: Option<String>,
    error: Option<String>,
}

/// Checkout engine for instant checkout functionality
pub struct CheckoutEngine {
    api_client: Arc<ApiClient>,
    captcha_solver: Arc<dyn CaptchaSolverTrait + Send + Sync>,
    config: CheckoutConfig,
}

impl CheckoutEngine {
    /// Create a new checkout engine
    pub fn new(
        api_client: Arc<ApiClient>,
        captcha_solver: Arc<dyn CaptchaSolverTrait + Send + Sync>,
    ) -> Self {
        Self {
            api_client,
            captcha_solver,
            config: CheckoutConfig::default(),
        }
    }

    /// Create a new checkout engine with custom configuration
    pub fn with_config(
        api_client: Arc<ApiClient>,
        captcha_solver: Arc<dyn CaptchaSolverTrait + Send + Sync>,
        config: CheckoutConfig,
    ) -> Self {
        Self {
            api_client,
            captcha_solver,
            config,
        }
    }

    /// Perform instant checkout
    pub async fn instant_checkout(
        &self,
        product: &Product,
        account: &Account,
        session: &Session,
    ) -> Result<CheckoutResult> {
        let start_time = std::time::Instant::now();
        info!(
            "Starting instant checkout for product: {} ({})",
            product.name, product.id
        );

        // Verify session is valid
        if !session.is_valid {
            error!("Session is not valid");
            return Ok(CheckoutResult::failure(
                "Session expired".to_string(),
                start_time.elapsed().as_millis() as u64,
            ));
        }

        // Step 1: Add to cart with retries
        let cart_id = match self.add_to_cart_with_retry(product, session).await {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to add product to cart: {}", e);
                return Ok(CheckoutResult::failure(
                    format!("Add to cart failed: {}", e),
                    start_time.elapsed().as_millis() as u64,
                ));
            }
        };

        // Step 2: Get checkout URL
        let checkout_url = match self.get_checkout_url_with_retry(&cart_id, session).await {
            Ok(url) => url,
            Err(e) => {
                error!("Failed to get checkout URL: {}", e);
                return Ok(CheckoutResult::failure(
                    format!("Get checkout URL failed: {}", e),
                    start_time.elapsed().as_millis() as u64,
                ));
            }
        };

        // Step 3: Fill shipping information
        if let Err(e) = self
            .fill_shipping_info(&checkout_url, &account.settings, session)
            .await
        {
            error!("Failed to fill shipping info: {}", e);
            return Ok(CheckoutResult::failure(
                format!("Shipping info failed: {}", e),
                start_time.elapsed().as_millis() as u64,
            ));
        }

        // Step 4: Select payment method
        if let Err(e) = self
            .select_payment_method(&checkout_url, &account.settings, session)
            .await
        {
            error!("Failed to select payment method: {}", e);
            return Ok(CheckoutResult::failure(
                format!("Payment selection failed: {}", e),
                start_time.elapsed().as_millis() as u64,
            ));
        }

        // Step 5: Detect and solve captcha if present
        let captcha_token = match self.detect_and_solve_captcha(&checkout_url, session).await {
            Ok(token) => token,
            Err(e) => {
                error!("Failed to handle captcha: {}", e);
                return Ok(CheckoutResult::failure(
                    format!("Captcha handling failed: {}", e),
                    start_time.elapsed().as_millis() as u64,
                ));
            }
        };

        // Step 6: Submit order with retries
        let order_id = match self
            .submit_order_with_retry(&checkout_url, captcha_token.as_deref(), session)
            .await
        {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to submit order: {}", e);
                return Ok(CheckoutResult::failure(
                    format!("Order submission failed: {}", e),
                    start_time.elapsed().as_millis() as u64,
                ));
            }
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;
        info!(
            "Checkout completed successfully! Order ID: {} (took {}ms)",
            order_id, duration_ms
        );
        Ok(CheckoutResult::success(order_id, duration_ms))
    }

    /// Add product to cart with retry logic
    async fn add_to_cart_with_retry(&self, product: &Product, session: &Session) -> Result<String> {
        let mut delay = self.config.base_delay_ms;

        for attempt in 0..self.config.add_to_cart_retries {
            debug!(
                "Add to cart attempt {} of {}",
                attempt + 1,
                self.config.add_to_cart_retries
            );

            match self.add_to_cart(product, session).await {
                Ok(cart_id) => {
                    info!("Successfully added product to cart: {}", cart_id);
                    return Ok(cart_id);
                }
                Err(e) => {
                    warn!("Add to cart attempt {} failed: {}", attempt + 1, e);

                    if attempt < self.config.add_to_cart_retries - 1 {
                        debug!("Waiting {}ms before retry", delay);
                        sleep(Duration::from_millis(delay)).await;
                        delay = std::cmp::min(
                            (delay as f64 * self.config.backoff_multiplier) as u64,
                            self.config.max_delay_ms,
                        );
                    }
                }
            }
        }

        Err(anyhow!(
            "Failed to add to cart after {} retries",
            self.config.add_to_cart_retries
        ))
    }

    /// Add product to cart
    async fn add_to_cart(&self, product: &Product, session: &Session) -> Result<String> {
        debug!("Adding product {} to cart", product.id);

        let url = format!("https://api.lazada.com/cart/add");
        let body = serde_json::json!({
            "product_id": product.id,
            "quantity": product.quantity,
            "session_token": session.id,
        });

        let response = self
            .api_client
            .request(
                Method::POST,
                &url,
                None,
                Some(body.to_string().into_bytes()),
                None,
            )
            .await
            .context("Failed to send add-to-cart request")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Add to cart failed with status {}",
                response.status
            ));
        }

        let cart_response: AddToCartResponse = serde_json::from_slice(&response.body)
            .context("Failed to parse add-to-cart response")?;

        if !cart_response.success {
            return Err(anyhow!(
                "Add to cart unsuccessful: {}",
                cart_response
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        cart_response
            .cart_id
            .ok_or_else(|| anyhow!("Cart ID not provided in response"))
    }

    /// Get checkout URL with retry logic
    async fn get_checkout_url_with_retry(
        &self,
        cart_id: &str,
        session: &Session,
    ) -> Result<String> {
        let mut delay = self.config.base_delay_ms;

        for attempt in 0..self.config.checkout_url_retries {
            debug!(
                "Get checkout URL attempt {} of {}",
                attempt + 1,
                self.config.checkout_url_retries
            );

            match self.get_checkout_url(cart_id, session).await {
                Ok(url) => {
                    info!("Successfully retrieved checkout URL");
                    return Ok(url);
                }
                Err(e) => {
                    warn!("Get checkout URL attempt {} failed: {}", attempt + 1, e);

                    if attempt < self.config.checkout_url_retries - 1 {
                        debug!("Waiting {}ms before retry", delay);
                        sleep(Duration::from_millis(delay)).await;
                        delay = std::cmp::min(
                            (delay as f64 * self.config.backoff_multiplier) as u64,
                            self.config.max_delay_ms,
                        );
                    }
                }
            }
        }

        Err(anyhow!(
            "Failed to get checkout URL after {} retries",
            self.config.checkout_url_retries
        ))
    }

    /// Get checkout URL
    async fn get_checkout_url(&self, cart_id: &str, _session: &Session) -> Result<String> {
        debug!("Getting checkout URL for cart {}", cart_id);

        let url = format!("https://api.lazada.com/cart/{}/checkout", cart_id);

        let response = self
            .api_client
            .request(Method::GET, &url, None, None, None)
            .await
            .context("Failed to get checkout URL")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Get checkout URL failed with status {}",
                response.status
            ));
        }

        let checkout_response: CheckoutUrlResponse = serde_json::from_slice(&response.body)
            .context("Failed to parse checkout URL response")?;

        checkout_response
            .checkout_url
            .ok_or_else(|| anyhow!("Checkout URL not provided in response"))
    }

    /// Fill shipping information
    async fn fill_shipping_info(
        &self,
        checkout_url: &str,
        settings: &AccountSettings,
        session: &Session,
    ) -> Result<()> {
        debug!("Filling shipping information");

        let url = format!("{}/shipping", checkout_url);
        let body = serde_json::json!({
            "address": settings.shipping_address,
            "session_token": session.id,
        });

        let response = self
            .api_client
            .request(
                Method::POST,
                &url,
                None,
                Some(body.to_string().into_bytes()),
                None,
            )
            .await
            .context("Failed to update shipping info")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Fill shipping info failed with status {}",
                response.status
            ));
        }

        info!("Shipping information filled successfully");
        Ok(())
    }

    /// Select payment method
    async fn select_payment_method(
        &self,
        checkout_url: &str,
        settings: &AccountSettings,
        session: &Session,
    ) -> Result<()> {
        debug!("Selecting payment method: {}", settings.payment_method);

        let url = format!("{}/payment", checkout_url);
        let body = serde_json::json!({
            "payment_method": settings.payment_method,
            "session_token": session.id,
        });

        let response = self
            .api_client
            .request(
                Method::POST,
                &url,
                None,
                Some(body.to_string().into_bytes()),
                None,
            )
            .await
            .context("Failed to select payment method")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Select payment method failed with status {}",
                response.status
            ));
        }

        info!("Payment method selected successfully");
        Ok(())
    }

    /// Detect and solve captcha if present
    async fn detect_and_solve_captcha(
        &self,
        checkout_url: &str,
        _session: &Session,
    ) -> Result<Option<String>> {
        debug!("Detecting captcha");

        let url = format!("{}/captcha-check", checkout_url);

        let response = self
            .api_client
            .request(Method::GET, &url, None, None, None)
            .await
            .context("Failed to detect captcha")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Captcha detection failed with status {}",
                response.status
            ));
        }

        let captcha_detection: CaptchaDetectionResponse = serde_json::from_slice(&response.body)
            .context("Failed to parse captcha detection response")?;

        if !captcha_detection.has_captcha {
            info!("No captcha detected");
            return Ok(None);
        }

        info!("Captcha detected, solving...");

        // Solve captcha based on type
        let captcha_token = match captcha_detection.captcha_type.as_deref() {
            Some("recaptcha_v2") => {
                let site_key = captcha_detection
                    .site_key
                    .ok_or_else(|| anyhow!("Site key not provided for reCAPTCHA"))?;
                let page_url = captcha_detection
                    .page_url
                    .unwrap_or_else(|| checkout_url.to_string());

                self.captcha_solver
                    .solve_recaptcha(&site_key, &page_url)
                    .await
                    .context("Failed to solve reCAPTCHA")?
            }
            Some("image") => {
                // For image captcha, we'd need to fetch the image first
                // This is a simplified version
                warn!("Image captcha detected but not fully implemented");
                return Err(anyhow!("Image captcha handling not fully implemented"));
            }
            _ => {
                return Err(anyhow!("Unknown captcha type"));
            }
        };

        info!("Captcha solved successfully");
        Ok(Some(captcha_token))
    }

    /// Submit order with retry logic
    async fn submit_order_with_retry(
        &self,
        checkout_url: &str,
        captcha_token: Option<&str>,
        session: &Session,
    ) -> Result<String> {
        let mut delay = self.config.base_delay_ms;

        for attempt in 0..self.config.submission_retries {
            debug!(
                "Submit order attempt {} of {}",
                attempt + 1,
                self.config.submission_retries
            );

            match self
                .submit_order(checkout_url, captcha_token, session)
                .await
            {
                Ok(order_id) => {
                    info!("Successfully submitted order: {}", order_id);
                    return Ok(order_id);
                }
                Err(e) => {
                    warn!("Submit order attempt {} failed: {}", attempt + 1, e);

                    if attempt < self.config.submission_retries - 1 {
                        debug!("Waiting {}ms before retry", delay);
                        sleep(Duration::from_millis(delay)).await;
                        delay = std::cmp::min(
                            (delay as f64 * self.config.backoff_multiplier) as u64,
                            self.config.max_delay_ms,
                        );
                    }
                }
            }
        }

        Err(anyhow!(
            "Failed to submit order after {} retries",
            self.config.submission_retries
        ))
    }

    /// Submit order
    async fn submit_order(
        &self,
        checkout_url: &str,
        captcha_token: Option<&str>,
        session: &Session,
    ) -> Result<String> {
        debug!("Submitting order");

        let url = format!("{}/submit", checkout_url);
        let mut body_data = serde_json::json!({
            "session_token": session.id,
        });

        if let Some(token) = captcha_token {
            body_data["captcha_token"] = serde_json::json!(token);
        }

        let response = self
            .api_client
            .request(
                Method::POST,
                &url,
                None,
                Some(body_data.to_string().into_bytes()),
                None,
            )
            .await
            .context("Failed to submit order")?;

        if response.status != 200 {
            return Err(anyhow!(
                "Submit order failed with status {}",
                response.status
            ));
        }

        let submission_response: OrderSubmissionResponse =
            serde_json::from_slice(&response.body)
                .context("Failed to parse order submission response")?;

        if !submission_response.success {
            return Err(anyhow!(
                "Order submission unsuccessful: {}",
                submission_response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        submission_response
            .order_id
            .ok_or_else(|| anyhow!("Order ID not provided in response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::captcha::MockCaptchaSolver;
    use crate::config::AccountSettings;

    #[tokio::test]
    async fn test_checkout_result_success() {
        let result = CheckoutResult::success("ORDER123".to_string(), 5000);
        assert!(result.success);
        assert_eq!(result.order_id, Some("ORDER123".to_string()));
        assert!(result.error.is_none());
        assert_eq!(result.duration_ms, 5000);
    }

    #[tokio::test]
    async fn test_checkout_result_failure() {
        let result = CheckoutResult::failure("Network error".to_string(), 3000);
        assert!(!result.success);
        assert!(result.order_id.is_none());
        assert_eq!(result.error, Some("Network error".to_string()));
        assert_eq!(result.duration_ms, 3000);
    }

    #[tokio::test]
    async fn test_product_builder() {
        let product = Product::new(
            "PROD123".to_string(),
            "Test Product".to_string(),
            "https://lazada.com/prod123".to_string(),
        )
        .with_price(99.99)
        .with_quantity(2);

        assert_eq!(product.id, "PROD123");
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, Some(99.99));
        assert_eq!(product.quantity, 2);
    }

    #[tokio::test]
    async fn test_checkout_config_default() {
        let config = CheckoutConfig::default();
        assert_eq!(config.add_to_cart_retries, 3);
        assert_eq!(config.base_delay_ms, 1000);
        assert_eq!(config.max_delay_ms, 10000);
        assert_eq!(config.backoff_multiplier, 2.0);
    }
}
