use anyhow::Result;
use reqwest::{header::HeaderMap, Method};

use super::{simulate_typing, BehaviorSimulator, BrowserFingerprint, FingerprintSpoofer};
use crate::api::{ApiClient, ProxyInfo, ResponseBody};

/// Enhanced API client with stealth capabilities
pub struct StealthClient {
    fingerprint: BrowserFingerprint,
    behavior_simulator: BehaviorSimulator,
    base_client: ApiClient,
}

impl StealthClient {
    /// Create a new sealth client with a random fingerprint
    pub fn new() -> Result<Self> {
        let fingerprint = FingerprintSpoofer::generate();
        Self::with_fingerprint(fingerprint)
    }

    /// Create a new stealth client with a specific fingerprint
    pub fn with_fingerprint(fingerprint: BrowserFingerprint) -> Result<Self> {
        let behavior_simulator = BehaviorSimulator::new();
        let base_client = ApiClient::new(Some(fingerprint.user_agent.clone()))?;

        Ok(Self {
            fingerprint,
            behavior_simulator,
            base_client,
        })
    }

    /// Create a stealth client for a specific browser
    pub fn for_browser(browser: &str) -> Result<Self> {
        let fingerprint = FingerprintSpoofer::generate_for_browser(browser);
        Self::with_fingerprint(fingerprint)
    }

    /// Get the current fingerprint
    pub fn fingerprint(&self) -> &BrowserFingerprint {
        &self.fingerprint
    }

    /// Update the fingerprint
    pub fn update_fingerprint(&mut self, fingerprint: BrowserFingerprint) {
        self.fingerprint = fingerprint;
    }

    /// Generate a new random fingerprint
    pub fn randomize_fingerprint(&mut self) {
        self.fingerprint = FingerprintSpoofer::generate();
    }

    /// Make a request with stealth headers and behavior simulation
    pub async fn stealth_request(
        &mut self,
        method: Method,
        url: &str,
        custom_headers: Option<HeaderMap>,
        body: Option<Vec<u8>>,
        proxy: Option<ProxyInfo>,
    ) -> Result<ResponseBody> {
        // Add random delay before request
        self.behavior_simulator.random_delay(100, 500).await;

        // Convert fingerprint to headers
        let stealth_headers = self.fingerprint.to_headers();

        // Merge with custom headers
        let mut headers = HeaderMap::new();

        // Add stealth headers
        for (key, value) in stealth_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                key.parse::<reqwest::header::HeaderName>(),
                value.parse::<reqwest::header::HeaderValue>(),
            ) {
                headers.insert(header_name, header_value);
            }
        }

        // Add custom headers (these will override stealth headers if there are conflicts)
        if let Some(custom) = custom_headers {
            for (key, value) in custom.iter() {
                headers.insert(key.clone(), value.clone());
            }
        }

        // Make the request
        let response = self
            .base_client
            .request(method, url, Some(headers), body, proxy)
            .await?;

        // Add random delay after request
        self.behavior_simulator.random_delay(200, 800).await;

        Ok(response)
    }

    /// Make a GET request with stealth
    pub async fn stealth_get(
        &mut self,
        url: &str,
        custom_headers: Option<HeaderMap>,
        proxy: Option<ProxyInfo>,
    ) -> Result<ResponseBody> {
        self.stealth_request(Method::GET, url, custom_headers, None, proxy)
            .await
    }

    /// Make a POST request with stealth
    pub async fn stealth_post(
        &mut self,
        url: &str,
        body: Vec<u8>,
        custom_headers: Option<HeaderMap>,
        proxy: Option<ProxyInfo>,
    ) -> Result<ResponseBody> {
        self.stealth_request(Method::POST, url, custom_headers, Some(body), proxy)
            .await
    }

    /// Simulate human-like form filling
    pub async fn stealth_form_fill(&mut self, form_data: &str) -> String {
        // Simulate reading the form
        self.behavior_simulator.reading_delay(form_data).await;

        // Simulate typing the form data
        simulate_typing(form_data).await
    }

    /// Simulate page navigation with realistic delays
    pub async fn stealth_navigate(&mut self, url: &str) -> Result<ResponseBody> {
        // Simulate page load delay
        self.behavior_simulator.page_load_delay().await;

        // Make the request
        self.stealth_get(url, None, None).await
    }

    /// Get behavior simulator for custom behavior simulation
    pub fn behavior_simulator(&mut self) -> &mut BehaviorSimulator {
        &mut self.behavior_simulator
    }
}

impl Default for StealthClient {
    fn default() -> Self {
        Self::new().expect("Failed to create stealth client")
    }
}

/// Helper function to create a stealth client with specific browser
pub fn create_stealth_client(browser: &str) -> Result<StealthClient> {
    StealthClient::for_browser(browser)
}

/// Helper function to create a stealth client with random fingerprint
pub fn create_random_stealth_client() -> Result<StealthClient> {
    StealthClient::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stealth_client_creation() {
        let client = StealthClient::new().unwrap();
        assert!(!client.fingerprint().user_agent.is_empty());
    }

    #[tokio::test]
    async fn test_browser_specific_client() {
        let chrome_client = StealthClient::for_browser("chrome").unwrap();
        assert!(chrome_client.fingerprint().user_agent.contains("Chrome"));

        let firefox_client = StealthClient::for_browser("firefox").unwrap();
        assert!(firefox_client.fingerprint().user_agent.contains("Firefox"));
    }

    #[tokio::test]
    async fn test_fingerprint_update() {
        let mut client = StealthClient::new().unwrap();
        let original_ua = client.fingerprint().user_agent.clone();

        client.randomize_fingerprint();
        assert_ne!(client.fingerprint().user_agent, original_ua);
    }

    #[tokio::test]
    async fn test_stealth_headers() {
        let client = StealthClient::new().unwrap();
        let headers = client.fingerprint().to_headers();

        assert!(headers.contains_key("User-Agent"));
        assert!(headers.contains_key("Accept-Language"));
        assert!(headers.contains_key("Accept-Encoding"));
    }
}
