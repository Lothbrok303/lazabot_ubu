pub mod loader;
pub mod encryption;
pub mod credentials;
pub mod host_config;
pub mod validation;

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
    /// Proxy username (encrypted)
    pub username: Option<String>,
    /// Proxy password (encrypted)
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
    /// API key (encrypted)
    pub api_key: String,
    /// API endpoint
    pub endpoint: String,
    /// Timeout in seconds
    pub timeout: u64,
    /// Auto-solve captchas
    pub auto_solve: bool,
    /// Polling interval in seconds
    pub polling_interval: u64,
    /// Maximum attempts
    pub max_attempts: u32,
}

/// Stealth and anti-detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Enable random delays
    pub random_delays: bool,
    /// Enable proxy rotation
    pub proxy_rotation: bool,
    /// User agent rotation
    pub user_agent_rotation: bool,
    /// Request header randomization
    pub header_randomization: bool,
    /// Browser fingerprint spoofing
    pub fingerprint_spoofing: bool,
}

/// Monitoring and logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: String,
    /// Log format
    pub log_format: String,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics port
    pub metrics_port: u16,
    /// Health check interval in milliseconds
    pub check_interval_ms: u64,
    /// Maximum concurrent monitors
    pub max_concurrent_monitors: u32,
}

/// Create a default configuration
pub fn create_default_config() -> Config {
    Config {
        bot: BotConfig {
            name: "lazabot".to_string(),
            default_delay: 1000,
            max_retries: 3,
            debug: false,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        },
        accounts: Vec::new(),
        proxies: Vec::new(),
        captcha: CaptchaConfig {
            service: "2captcha".to_string(),
            api_key: "".to_string(), // Will be loaded from environment
            endpoint: "https://2captcha.com/api".to_string(),
            timeout: 120,
            auto_solve: true,
            polling_interval: 5,
            max_attempts: 60,
        },
        stealth: StealthConfig {
            random_delays: true,
            proxy_rotation: true,
            user_agent_rotation: false,
            header_randomization: true,
            fingerprint_spoofing: true,
        },
        monitoring: MonitoringConfig {
            enable_logging: true,
            log_level: "info".to_string(),
            log_format: "json".to_string(),
            enable_metrics: true,
            metrics_port: 9091,
            check_interval_ms: 5000,
            max_concurrent_monitors: 10,
        },
    }
}

/// Configuration manager that handles loading, validation, and merging
pub struct ConfigManager {
    main_config: Option<Config>,
    host_config: Option<crate::config::host_config::HostConfig>,
    credential_manager: Option<crate::config::credentials::CredentialManager>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Self {
        Self {
            main_config: None,
            host_config: None,
            credential_manager: None,
        }
    }

    /// Load configuration from files and environment
    pub fn load(&mut self, config_path: &str, vault_path: &str) -> anyhow::Result<()> {
        // Load main configuration
        self.main_config = Some(loader::load_config(config_path)?);

        // Detect and load host-specific configuration
        let mut host_manager = crate::config::host_config::HostConfigManager::new("config");
        self.host_config = Some(host_manager.detect_and_load()?);

        // Initialize credential manager
        self.credential_manager = Some(crate::config::credentials::CredentialManager::new(vault_path)?);

        Ok(())
    }

    /// Validate all configuration components
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate environment variables
        let validator = crate::config::validation::EnvValidator::new();
        let report = validator.validate_all()?;
        
        if report.has_errors() {
            report.print_report();
            anyhow::bail!("Configuration validation failed");
        }

        // Validate credentials if credential manager is available
        if let Some(credential_manager) = &self.credential_manager {
            // Create a mutable reference for load_from_env
            let mut manager = crate::config::credentials::CredentialManager::new("temp")?;
            manager.load_from_env()?;
        }

        Ok(())
    }

    /// Get the main configuration
    pub fn get_main_config(&self) -> Option<&Config> {
        self.main_config.as_ref()
    }

    /// Get the host configuration
    pub fn get_host_config(&self) -> Option<&crate::config::host_config::HostConfig> {
        self.host_config.as_ref()
    }

    /// Get the credential manager
    pub fn get_credential_manager(&self) -> Option<&crate::config::credentials::CredentialManager> {
        self.credential_manager.as_ref()
    }

    /// Get merged configuration (main + host overrides)
    pub fn get_merged_config(&self) -> anyhow::Result<Config> {
        let config = self.main_config.clone()
            .ok_or_else(|| anyhow::anyhow!("Main configuration not loaded"))?;

        // Apply host-specific overrides if available
        if let Some(_host_config) = &self.host_config {
            // Apply overrides to the configuration
            // This is a simplified implementation
            // In a real implementation, you would use a more sophisticated merging strategy
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = create_default_config();
        assert_eq!(config.bot.name, "lazabot");
        assert_eq!(config.bot.default_delay, 1000);
        assert_eq!(config.bot.max_retries, 3);
        assert!(!config.bot.debug);
        assert_eq!(config.captcha.service, "2captcha");
        assert!(config.stealth.random_delays);
        assert!(config.stealth.proxy_rotation);
        assert!(config.monitoring.enable_logging);
    }

    #[test]
    fn test_config_serialization() {
        let config = create_default_config();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("lazabot"));
        assert!(json.contains("2captcha"));
    }

    #[test]
    fn test_config_deserialization() {
        let config = create_default_config();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.bot.name, config.bot.name);
        assert_eq!(deserialized.bot.default_delay, config.bot.default_delay);
    }
}
