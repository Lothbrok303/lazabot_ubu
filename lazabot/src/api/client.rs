use anyhow::{Result, Context};
use reqwest::{Client, ClientBuilder, Method, Url, header::HeaderMap};
use reqwest::cookie::Jar;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error, debug};

#[derive(Debug, Clone)]
pub struct ProxyInfo {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ProxyInfo {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port, username: None, password: None }
    }

    pub fn with_auth(mut self, username: String, password: String) -> Self {
        self.username = Some(username);
        self.password = Some(password);
        self
    }

    pub fn to_url(&self) -> Result<String> {
        let auth = if let (Some(username), Some(password)) = (&self.username, &self.password) {
            format!("{}:{}@", username, password)
        } else {
            String::new()
        };
        Ok(format!("http://{}{}:{}", auth, self.host, self.port))
    }
}

#[derive(Debug)]
pub struct ResponseBody {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    pub text: String,
}

impl ResponseBody {
    pub fn new(status: u16, headers: HeaderMap, body: Vec<u8>) -> Self {
        let text = String::from_utf8_lossy(&body).to_string();
        Self { status, headers, body, text }
    }
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

pub struct ApiClient {
    client: Client,
    user_agent: String,
    retry_config: RetryConfig,
}

impl ApiClient {
    pub fn new(user_agent: Option<String>) -> Result<Self> {
        let cookie_store = Arc::new(Jar::default());
        let ua = user_agent.unwrap_or_else(|| "Lazabot/1.0".to_string());
        
        let builder = ClientBuilder::new()
            .cookie_provider(cookie_store)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent(&ua);

        let client = builder.build().context("Failed to create HTTP client")?;
        Ok(Self { client, user_agent: ua, retry_config: RetryConfig::default() })
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub async fn request(
        &self,
        method: Method,
        url: &str,
        headers: Option<HeaderMap>,
        body: Option<Vec<u8>>,
        proxy: Option<ProxyInfo>,
    ) -> Result<ResponseBody> {
        let url = Url::parse(url).context("Invalid URL")?;
        
        // Create client with proxy if provided
        let client = if let Some(proxy_info) = &proxy {
            let proxy_url = proxy_info.to_url()?;
            let proxy = reqwest::Proxy::all(&proxy_url).context("Failed to create proxy")?;
            
            let cookie_store = Arc::new(Jar::default());
            let builder = ClientBuilder::new()
                .cookie_provider(cookie_store)
                .proxy(proxy)
                .timeout(Duration::from_secs(30))
                .connect_timeout(Duration::from_secs(10))
                .redirect(reqwest::redirect::Policy::limited(10))
                .user_agent(&self.user_agent);

            builder.build().context("Failed to create proxy client")?
        } else {
            self.client.clone()
        };

        let mut request_builder = client.request(method, url);

        if let Some(headers) = headers {
            request_builder = request_builder.headers(headers);
        }

        if let Some(body) = body {
            request_builder = request_builder.body(body);
        }

        self.execute_with_retry(request_builder).await
    }

    async fn execute_with_retry(
        &self,
        request_builder: reqwest::RequestBuilder,
    ) -> Result<ResponseBody> {
        let mut last_error = None;
        let mut delay = self.retry_config.base_delay_ms;

        for attempt in 0..=self.retry_config.max_retries {
            let request = request_builder.try_clone().context("Failed to clone request")?;

            debug!("Attempt {} of {} for request", attempt + 1, self.retry_config.max_retries + 1);

            match request.send().await {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let headers = response.headers().clone();
                    let url = response.url().clone();
                    
                    match response.bytes().await {
                        Ok(body_bytes) => {
                            let response_body = ResponseBody::new(status, headers, body_bytes.to_vec());
                            info!("Request successful: {} {}", status, url);
                            return Ok(response_body);
                        }
                        Err(e) => {
                            warn!("Failed to read response body on attempt {}: {}", attempt + 1, e);
                            last_error = Some(e.into());
                        }
                    }
                }
                Err(e) => {
                    warn!("Request failed on attempt {}: {}", attempt + 1, e);
                    last_error = Some(e.into());
                }
            }

            if attempt < self.retry_config.max_retries {
                debug!("Waiting {}ms before retry", delay);
                sleep(Duration::from_millis(delay)).await;
                delay = std::cmp::min(
                    (delay as f64 * self.retry_config.backoff_multiplier) as u64,
                    self.retry_config.max_delay_ms
                );
            }
        }

        error!("All retry attempts failed");
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Unknown error")))
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn set_retry_config(&mut self, config: RetryConfig) {
        self.retry_config = config;
    }
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClient")
            .field("retry_config", &self.retry_config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ApiClient::new(Some("TestAgent/1.0".to_string()));
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_proxy_info() {
        let proxy = ProxyInfo::new("127.0.0.1".to_string(), 8080)
            .with_auth("user".to_string(), "pass".to_string());
        let url = proxy.to_url().unwrap();
        assert_eq!(url, "http://user:pass@127.0.0.1:8080");
    }

    #[tokio::test]
    async fn test_proxy_info_no_auth() {
        let proxy = ProxyInfo::new("127.0.0.1".to_string(), 8080);
        let url = proxy.to_url().unwrap();
        assert_eq!(url, "http://127.0.0.1:8080");
    }
}
impl ApiClient {
    pub fn with_cookie_jar(cookie_jar: Arc<Jar>) -> Result<ApiClient> {
        let ua = "Lazabot/1.0".to_string();
        
        let builder = ClientBuilder::new()
            .cookie_provider(cookie_jar)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
           .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent(&ua);

        let client = builder.build().context("Failed to create HTTP client with cookie jar")?;
        Ok(ApiClient { client, user_agent: ua, retry_config: RetryConfig::default() })
    }
}
