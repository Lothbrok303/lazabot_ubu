pub mod loader;

use serde::{Deserialize, Serialize};

/// Main configuration structure for the Lazada bot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Bot configuration settings
    pub bot: BotConfig,
    /// Account configurations
    pub accounts: Vec<AccountConfig>,
    /// Proxy configurations
    pub proxies: Vec<ProxyConfig>,
    /// Captcha solving configuration
    pub captcha: CaptchaConfig,
    /// Stealth and anti-detection settings
    pub stealth: StealthConfig,
    /// Monitoring and logging configuration
    pub monitoring: MonitoringConfig,
}

/// Bot-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    /// Bot name/identifier
    pub name: String,
    /// Default delay between actions (milliseconds)
    pub default_delay: u64,
    /// Maximum retry attempts for failed operations
    pub max_retries: u32,
    /// Enable debug mode
    pub debug: bool,
    /// User agent string to use
    pub user_agent: String,
}

/// Account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// Account identifier
    pub id: String,
    /// Username/email (encrypted)
    pub username: String,
    /// Password (encrypted)
    pub password: String,
    /// Account status (active, inactive, banned)
    pub status: String,
    /// Last login timestamp
    pub last_login: Option<String>,
    /// Account-specific settings
    pub settings: AccountSettings,
}

/// Account-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    /// Preferred payment method
    pub payment_method: String,
    /// Default shipping address
    pub shipping_address: String,
    /// Notification preferences
    pub notifications: bool,
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Proxy identifier
    pub id: String,
    /// Proxy type (http, socks5, etc.)
    pub proxy_type: String,
    /// Proxy host
    pub host: String,
    /// Proxy port
    pub port: u16,
    /// Username for proxy authentication (encrypted)
    pub username: Option<String>,
    /// Password for proxy authentication (encrypted)
    pub password: Option<String>,
    /// Proxy status
    pub status: String,
    /// Last tested timestamp
    pub last_tested: Option<String>,
}

/// Captcha solving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaConfig {
    /// Captcha service provider
    pub service: String,
    /// API key for captcha service (encrypted)
    pub api_key: String,
    /// Service endpoint URL
    pub endpoint: String,
    /// Timeout for captcha solving (seconds)
    pub timeout: u64,
    /// Enable automatic captcha solving
    pub auto_solve: bool,
}

/// Stealth and anti-detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Enable random delays between actions
    pub random_delays: bool,
    /// Minimum delay (milliseconds)
    pub min_delay: u64,
    /// Maximum delay (milliseconds)
    pub max_delay: u64,
    /// Rotate user agents
    pub rotate_user_agents: bool,
    /// User agent pool
    pub user_agents: Vec<String>,
    /// Enable browser fingerprint randomization
    pub randomize_fingerprint: bool,
    /// Enable proxy rotation
    pub proxy_rotation: bool,
}

/// Monitoring and logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable detailed logging
    pub enable_logging: bool,
    /// Log level (debug, info, warn, error)
    pub log_level: String,
    /// Log file path
    pub log_file: String,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval (seconds)
    pub monitoring_interval: u64,
    /// Enable alerts
    pub enable_alerts: bool,
    /// Alert webhook URL
    pub alert_webhook: Option<String>,
}

// Re-export the loader functions that are actually used
pub use loader::{load_config, create_default_config};
pub mod encryption;
