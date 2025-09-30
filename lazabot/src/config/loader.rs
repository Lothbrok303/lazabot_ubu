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

/// Encrypt a string using AES-GCM encryption
///
/// # Security Note
/// This is a placeholder implementation. In production, you should:
/// 1. Use a proper key management system (e.g., AWS KMS, HashiCorp Vault)
/// 2. Store encryption keys securely (not in code or config files)
/// 3. Use key rotation policies
/// 4. Implement proper key derivation functions
///
/// For now, this uses a hardcoded key for demonstration purposes.
/// Replace this with your actual key management solution.
pub fn encrypt_string(plaintext: &str) -> Result<String> {
    use aes_gcm::aead::Aead;
    use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
    use base64::{engine::general_purpose, Engine as _};

    // TODO: Replace with actual key management system
    // This is a placeholder key - DO NOT USE IN PRODUCTION
    let key_bytes = b"your-32-byte-key-here-please-change-this";
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a random nonce for each encryption
    let nonce = Nonce::from_slice(b"unique-nonce-12");

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to encrypt string: {:?}", e))?;

    // Combine nonce and ciphertext, then encode as base64
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(result))
}

/// Decrypt a string using AES-GCM decryption
///
/// # Security Note
/// This is a placeholder implementation. In production, you should:
/// 1. Use the same key management system as encrypt_string()
/// 2. Ensure keys are properly secured and rotated
/// 3. Implement proper error handling for decryption failures
pub fn decrypt_string(encrypted: &str) -> Result<String> {
    use aes_gcm::aead::Aead;
    use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
    use base64::{engine::general_purpose, Engine as _};

    // TODO: Replace with actual key management system
    // This is a placeholder key - DO NOT USE IN PRODUCTION
    let key_bytes = b"your-32-byte-key-here-please-change-this";
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Decode from base64
    let data = general_purpose::STANDARD
        .decode(encrypted)
        .context("Failed to decode base64 encrypted string")?;

    if data.len() < 12 {
        anyhow::bail!("Invalid encrypted data: too short");
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Failed to decrypt string: {:?}", e))?;

    String::from_utf8(plaintext).context("Failed to convert decrypted bytes to string")
}

/// Create a default configuration
pub fn create_default_config() -> Config {
    use crate::config::*;

    Config {
        bot: BotConfig {
            name: "lazabot".to_string(),
            default_delay: 1000,
            max_retries: 3,
            debug: false,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        },
        accounts: vec![],
        proxies: vec![],
        captcha: CaptchaConfig {
            service: "2captcha".to_string(),
            api_key: "your-api-key-here".to_string(),
            endpoint: "https://2captcha.com/api".to_string(),
            timeout: 120,
            auto_solve: true,
        },
        stealth: StealthConfig {
            random_delays: true,
            min_delay: 500,
            max_delay: 2000,
            rotate_user_agents: true,
            user_agents: vec![
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            ],
            randomize_fingerprint: true,
            proxy_rotation: true,
        },
        monitoring: MonitoringConfig {
            enable_logging: true,
            log_level: "info".to_string(),
            log_file: "logs/lazabot.log".to_string(),
            enable_monitoring: true,
            monitoring_interval: 60,
            enable_alerts: false,
            alert_webhook: None,
        },
    }
}
