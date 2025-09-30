use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

/// 2Captcha API endpoints
const API_BASE_URL: &str = "http://2captcha.com";
const SUBMIT_ENDPOINT: &str = "/in.php";
const RESULT_ENDPOINT: &str = "/res.php";

/// Maximum polling attempts for captcha solving
const MAX_POLLING_ATTEMPTS: u32 = 60;
/// Polling interval in seconds
const POLLING_INTERVAL: u64 = 5;
/// Request timeout in seconds
const REQUEST_TIMEOUT: u64 = 30;

/// Types of captcha supported by 2Captcha
#[derive(Debug, Clone)]
pub enum CaptchaType {
    Image,
    ReCaptchaV2,
    ReCaptchaV3,
}

/// Captcha solver trait for testability
#[async_trait]
pub trait CaptchaSolverTrait {
    async fn solve_image(&self, image_bytes: &[u8]) -> Result<String>;
    async fn solve_recaptcha(&self, site_key: &str, page_url: &str) -> Result<String>;
}

/// 2Captcha solver implementation
#[derive(Debug, Clone)]
pub struct CaptchaSolver {
    pub api_key: String,
    client: Client,
}

impl CaptchaSolver {
    /// Create a new captcha solver instance
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT))
            .build()
            .expect("Failed to create HTTP client");

        Self { api_key, client }
    }

    /// Create a new captcha solver from environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("CAPTCHA_API_KEY")
            .map_err(|_| anyhow!("CAPTCHA_API_KEY environment variable not set"))?;
        Ok(Self::new(api_key))
    }

    /// Submit a captcha to 2Captcha API
    async fn submit_captcha(
        &self,
        captcha_type: CaptchaType,
        data: &str,
        additional_params: Option<Vec<(&str, &str)>>,
    ) -> Result<String> {
        let mut params = vec![
            ("key", self.api_key.as_str()),
            ("method", self.get_method(&captcha_type)),
        ];

        match captcha_type {
            CaptchaType::Image => {
                params.push(("body", data));
            }
            CaptchaType::ReCaptchaV2 => {
                params.push(("googlekey", data));
                if let Some(url) = additional_params
                    .and_then(|p| p.iter().find(|(k, _)| *k == "pageurl").map(|(_, v)| *v))
                {
                    params.push(("pageurl", url));
                }
            }
            CaptchaType::ReCaptchaV3 => {
                params.push(("googlekey", data));
                if let Some(url) = additional_params
                    .and_then(|p| p.iter().find(|(k, _)| *k == "pageurl").map(|(_, v)| *v))
                {
                    params.push(("pageurl", url));
                }
                // Default action for ReCaptchaV3
                params.push(("action", "verify"));
                params.push(("min_score", "0.3"));
            }
        }

        let url = format!("{}{}", API_BASE_URL, SUBMIT_ENDPOINT);

        debug!("Submitting captcha to 2Captcha API: {}", url);

        let response = timeout(
            Duration::from_secs(REQUEST_TIMEOUT),
            self.client.post(&url).form(&params).send(),
        )
        .await
        .map_err(|_| anyhow!("Request timeout"))?
        .map_err(|e| anyhow!("Failed to submit captcha: {}", e))?;

        let response_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        debug!("2Captcha submit response: {}", response_text);

        if response_text.starts_with("OK|") {
            let captcha_id = response_text
                .strip_prefix("OK|")
                .ok_or_else(|| anyhow!("Invalid response format"))?;
            info!("Captcha submitted successfully with ID: {}", captcha_id);
            Ok(captcha_id.to_string())
        } else {
            Err(anyhow!("Failed to submit captcha: {}", response_text))
        }
    }

    /// Poll for captcha result
    async fn poll_result(&self, captcha_id: &str) -> Result<String> {
        let url = format!("{}{}", API_BASE_URL, RESULT_ENDPOINT);

        for attempt in 1..=MAX_POLLING_ATTEMPTS {
            debug!("Polling attempt {} for captcha ID: {}", attempt, captcha_id);

            let params = vec![
                ("key", self.api_key.as_str()),
                ("action", "get"),
                ("id", captcha_id),
            ];

            let response = timeout(
                Duration::from_secs(REQUEST_TIMEOUT),
                self.client.get(&url).query(&params).send(),
            )
            .await
            .map_err(|_| anyhow!("Request timeout"))?
            .map_err(|e| anyhow!("Failed to poll result: {}", e))?;

            let response_text = response
                .text()
                .await
                .map_err(|e| anyhow!("Failed to read response: {}", e))?;

            debug!("2Captcha result response: {}", response_text);

            if response_text == "CAPCHA_NOT_READY" {
                if attempt == MAX_POLLING_ATTEMPTS {
                    return Err(anyhow!(
                        "Captcha solving timeout after {} attempts",
                        MAX_POLLING_ATTEMPTS
                    ));
                }
                warn!("Captcha not ready, waiting {} seconds...", POLLING_INTERVAL);
                sleep(Duration::from_secs(POLLING_INTERVAL)).await;
                continue;
            }

            if response_text.starts_with("OK|") {
                let result = response_text
                    .strip_prefix("OK|")
                    .ok_or_else(|| anyhow!("Invalid response format"))?;
                info!("Captcha solved successfully: {}", result);
                return Ok(result.to_string());
            }

            return Err(anyhow!("Failed to solve captcha: {}", response_text));
        }

        Err(anyhow!("Captcha solving timeout"))
    }

    /// Get the method parameter for 2Captcha API
    pub fn get_method(&self, captcha_type: &CaptchaType) -> &'static str {
        match captcha_type {
            CaptchaType::Image => "base64",
            CaptchaType::ReCaptchaV2 => "userrecaptcha",
            CaptchaType::ReCaptchaV3 => "userrecaptcha",
        }
    }
}

#[async_trait]
impl CaptchaSolverTrait for CaptchaSolver {
    /// Solve an image captcha
    async fn solve_image(&self, image_bytes: &[u8]) -> Result<String> {
        info!("Solving image captcha ({} bytes)", image_bytes.len());

        let base64_image =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);
        let captcha_id = self
            .submit_captcha(CaptchaType::Image, &base64_image, None)
            .await?;

        self.poll_result(&captcha_id).await
    }

    /// Solve a reCAPTCHA v2
    async fn solve_recaptcha(&self, site_key: &str, page_url: &str) -> Result<String> {
        info!(
            "Solving reCAPTCHA v2 for site: {} at URL: {}",
            site_key, page_url
        );

        let additional_params = vec![("pageurl", page_url)];
        let captcha_id = self
            .submit_captcha(CaptchaType::ReCaptchaV2, site_key, Some(additional_params))
            .await?;

        self.poll_result(&captcha_id).await
    }
}

/// Mock captcha solver for testing
#[derive(Debug, Clone)]
pub struct MockCaptchaSolver {
    image_result: String,
    recaptcha_result: String,
}

impl MockCaptchaSolver {
    pub fn new(image_result: String, recaptcha_result: String) -> Self {
        Self {
            image_result,
            recaptcha_result,
        }
    }
}

#[async_trait]
impl CaptchaSolverTrait for MockCaptchaSolver {
    async fn solve_image(&self, _image_bytes: &[u8]) -> Result<String> {
        debug!("Mock solving image captcha");
        Ok(self.image_result.clone())
    }

    async fn solve_recaptcha(&self, _site_key: &str, _page_url: &str) -> Result<String> {
        debug!("Mock solving reCAPTCHA");
        Ok(self.recaptcha_result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_mock_image_captcha_solving() {
        let solver = MockCaptchaSolver::new("test123".to_string(), "recaptcha123".to_string());
        let result = solver.solve_image(b"fake_image_data").await.unwrap();
        assert_eq!(result, "test123");
    }

    #[tokio::test]
    async fn test_mock_recaptcha_solving() {
        let solver = MockCaptchaSolver::new("test123".to_string(), "recaptcha123".to_string());
        let result = solver
            .solve_recaptcha("site_key", "https://example.com")
            .await
            .unwrap();
        assert_eq!(result, "recaptcha123");
    }

    #[test]
    fn test_captcha_solver_creation() {
        let solver = CaptchaSolver::new("test_api_key".to_string());
        assert_eq!(solver.api_key, "test_api_key");
    }

    #[test]
    fn test_captcha_type_methods() {
        let solver = CaptchaSolver::new("test_api_key".to_string());
        assert_eq!(solver.get_method(&CaptchaType::Image), "base64");
        assert_eq!(
            solver.get_method(&CaptchaType::ReCaptchaV2),
            "userrecaptcha"
        );
        assert_eq!(
            solver.get_method(&CaptchaType::ReCaptchaV3),
            "userrecaptcha"
        );
    }
}
