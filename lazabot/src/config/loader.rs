use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Load configuration from a TOML file
pub fn load_config(path: &str) -> Result<Config> {
    let config_path = Path::new(path);

    if !config_path.exists() {
        anyhow::bail!("Configuration file not found: {}", path);
    }

    let content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read configuration file: {}", path))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse TOML configuration from: {}", path))?;

    Ok(config)
}

/// Load configuration from a YAML file
pub fn load_config_yaml(path: &str) -> Result<Config> {
    let config_path = Path::new(path);

    if !config_path.exists() {
        anyhow::bail!("Configuration file not found: {}", path);
    }

    let content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read configuration file: {}", path))?;

    let config: Config = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML configuration from: {}", path))?;

    Ok(config)
}

/// Save configuration to a TOML file
pub fn save_config(config: &Config, path: &str) -> Result<()> {
    let content =
        toml::to_string_pretty(config).context("Failed to serialize configuration to TOML")?;

    fs::write(path, content)
        .with_context(|| format!("Failed to write configuration to: {}", path))?;

    Ok(())
}

/// Save configuration to a YAML file
pub fn save_config_yaml(config: &Config, path: &str) -> Result<()> {
    let content =
        serde_yaml::to_string(config).context("Failed to serialize configuration to YAML")?;

    fs::write(path, content)
        .with_context(|| format!("Failed to write configuration to: {}", path))?;

    Ok(())
}

/// Create a default configuration and save it to a file
pub fn create_default_config_file(path: &str) -> Result<()> {
    let default_config = crate::config::create_default_config();
    save_config(&default_config, path)?;
    Ok(())
}

/// Create a default configuration with sample data
pub fn create_sample_config() -> Config {
    use crate::config::*;
    
    Config {
        bot: BotConfig {
            name: "lazabot".to_string(),
            default_delay: 1000,
            max_retries: 3,
            debug: false,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        },
        accounts: vec![
            AccountConfig {
                id: "account_1".to_string(),
                username: "user1@example.com".to_string(),
                password: "encrypted_password_1".to_string(),
                status: "active".to_string(),
                last_login: Some("2024-01-15T10:30:00Z".to_string()),
                settings: AccountSettings {
                    payment_method: "credit_card".to_string(),
                    shipping_address: "123 Main St, City, Country".to_string(),
                    notifications: true,
                },
            },
            AccountConfig {
                id: "account_2".to_string(),
                username: "user2@example.com".to_string(),
                password: "encrypted_password_2".to_string(),
                status: "inactive".to_string(),
                last_login: None,
                settings: AccountSettings {
                    payment_method: "paypal".to_string(),
                    shipping_address: "456 Oak Ave, City, Country".to_string(),
                    notifications: false,
                },
            },
        ],
        proxies: vec![
            ProxyConfig {
                id: "proxy_1".to_string(),
                proxy_type: "http".to_string(),
                host: "proxy1.example.com".to_string(),
                port: 8080,
                username: Some("proxy_user1".to_string()),
                password: Some("encrypted_proxy_pass1".to_string()),
                status: "active".to_string(),
                last_tested: Some("2024-01-15T10:30:00Z".to_string()),
            },
            ProxyConfig {
                id: "proxy_2".to_string(),
                proxy_type: "socks5".to_string(),
                host: "proxy2.example.com".to_string(),
                port: 1080,
                username: Some("proxy_user2".to_string()),
                password: Some("encrypted_proxy_pass2".to_string()),
                status: "active".to_string(),
                last_tested: Some("2024-01-15T10:30:00Z".to_string()),
            },
        ],
        captcha: CaptchaConfig {
            service: "2captcha".to_string(),
            api_key: "encrypted_api_key".to_string(),
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
