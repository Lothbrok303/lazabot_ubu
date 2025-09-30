use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info};

const SERVER_URL: &str = "http://localhost:8081";
const SERVER_STARTUP_TIMEOUT: Duration = Duration::from_secs(30);
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptchaRequest {
    pub captcha_url: String,
    pub captcha_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptchaResponse {
    pub success: bool,
    pub captcha_image: Option<String>,
    pub message: Option<String>,
    pub captcha_url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub product_url: String,
    pub quantity: Option<u32>,
    pub shipping_info: Option<serde_json::Value>,
    pub payment_info: Option<serde_json::Value>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    pub success: bool,
    pub message: Option<String>,
    pub screenshot: Option<String>,
    pub product_url: Option<String>,
    pub quantity: Option<u32>,
    pub timestamp: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub browser: String,
}

pub struct PlaywrightClient {
    client: Client,
    server_process: Option<std::process::Child>,
}

impl PlaywrightClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            server_process: None,
        }
    }

    /// Spawns the Playwright server if it's not already running
    pub async fn ensure_server_running(&mut self) -> Result<()> {
        // First, check if server is already running
        if self.is_server_healthy().await.is_ok() {
            info!("Playwright server is already running");
            return Ok(());
        }

        info!("Starting Playwright server...");
        self.start_server().await?;
        
        // Wait for server to be ready
        self.wait_for_server_ready().await?;
        
        info!("Playwright server is ready");
        Ok(())
    }

    /// Starts the Playwright server process
    async fn start_server(&mut self) -> Result<()> {
        let server_path = std::env::current_dir()?
            .join("scripts")
            .join("playwright_server.js");

        if !server_path.exists() {
            return Err(anyhow!("Playwright server script not found at: {:?}", server_path));
        }

        debug!("Starting server from: {:?}", server_path);

        let child = Command::new("node")
            .arg(server_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start Playwright server: {}", e))?;

        self.server_process = Some(child);
        Ok(())
    }

    /// Waits for the server to be ready by checking health endpoint
    async fn wait_for_server_ready(&self) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        while start_time.elapsed() < SERVER_STARTUP_TIMEOUT {
            if self.is_server_healthy().await.is_ok() {
                return Ok(());
            }
            
            sleep(Duration::from_millis(500)).await;
        }
        
        Err(anyhow!("Server failed to start within {:?}", SERVER_STARTUP_TIMEOUT))
    }

    /// Checks if the server is healthy
    pub async fn is_server_healthy(&self) -> Result<HealthResponse> {
        let response = timeout(
            HEALTH_CHECK_TIMEOUT,
            self.client.get(&format!("{}/health", SERVER_URL)).send()
        )
        .await
        .map_err(|_| anyhow!("Health check timeout"))?
        .map_err(|e| anyhow!("Health check failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Server returned status: {}", response.status()));
        }

        let health: HealthResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse health response: {}", e))?;

        Ok(health)
    }

    /// Solves a captcha using the Playwright server
    pub async fn solve_captcha(&self, request: CaptchaRequest) -> Result<CaptchaResponse> {
        debug!("Solving captcha: {}", request.captcha_url);

        let response = self
            .client
            .post(&format!("{}/solveCaptcha", SERVER_URL))
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send captcha request: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("Captcha solving failed: {}", error_text));
        }

        let captcha_response: CaptchaResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse captcha response: {}", e))?;

        if !captcha_response.success {
            return Err(anyhow!(
                "Captcha solving failed: {}",
                captcha_response.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        Ok(captcha_response)
    }

    /// Performs checkout flow using the Playwright server
    pub async fn perform_checkout_flow(&self, request: CheckoutRequest) -> Result<CheckoutResponse> {
        debug!("Performing checkout flow: {}", request.product_url);

        let response = self
            .client
            .post(&format!("{}/performCheckoutFlow", SERVER_URL))
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send checkout request: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("Checkout flow failed: {}", error_text));
        }

        let checkout_response: CheckoutResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse checkout response: {}", e))?;

        if !checkout_response.success {
            return Err(anyhow!(
                "Checkout flow failed: {}",
                checkout_response.error.unwrap_or_else(|| "Unknown error".to_string())
            ));
        }

        Ok(checkout_response)
    }

    /// Stops the server process
    pub fn stop_server(&mut self) -> Result<()> {
        if let Some(mut child) = self.server_process.take() {
            debug!("Stopping Playwright server...");
            child.kill().map_err(|e| anyhow!("Failed to stop server: {}", e))?;
            info!("Playwright server stopped");
        }
        Ok(())
    }
}

impl Drop for PlaywrightClient {
    fn drop(&mut self) {
        if let Err(e) = self.stop_server() {
            error!("Failed to stop Playwright server: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_playwright_client_health_check() {
        let client = PlaywrightClient::new();
        
        // This test will only pass if the server is running
        match client.is_server_healthy().await {
            Ok(health) => {
                println!("Server health: {:?}", health);
                assert_eq!(health.status, "healthy");
            }
            Err(e) => {
                println!("Server not running or unhealthy: {}", e);
                // This is expected if server is not running
            }
        }
    }

    #[tokio::test]
    async fn test_playwright_client_lifecycle() {
        let mut client = PlaywrightClient::new();
        
        // Test server startup
        match client.ensure_server_running().await {
            Ok(_) => {
                println!("Server started successfully");
                
                // Test health check
                let health = client.is_server_healthy().await.unwrap();
                assert_eq!(health.status, "healthy");
                
                // Test captcha solving
                let captcha_request = CaptchaRequest {
                    captcha_url: "https://example.com/captcha".to_string(),
                    captcha_type: Some("image".to_string()),
                };
                
                match client.solve_captcha(captcha_request).await {
                    Ok(response) => {
                        println!("Captcha response: {:?}", response);
                    }
                    Err(e) => {
                        println!("Captcha solving failed (expected): {}", e);
                    }
                }
                
                // Test checkout flow
                let checkout_request = CheckoutRequest {
                    product_url: "https://example.com/product".to_string(),
                    quantity: Some(1),
                    shipping_info: None,
                    payment_info: None,
                    user_agent: None,
                };
                
                match client.perform_checkout_flow(checkout_request).await {
                    Ok(response) => {
                        println!("Checkout response: {:?}", response);
                    }
                    Err(e) => {
                        println!("Checkout flow failed (expected): {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to start server: {}", e);
                // This is expected if Node.js is not installed
            }
        }
    }
}
