use anyhow::Result;
use std::env;
use tempfile::TempDir;

use lazabot::config::credentials::{CredentialManager, LazadaCredentials, CaptchaCredentials, ProxyCredentials};
use lazabot::config::validation::EnvValidator;
use lazabot::config::host_config::HostConfig;

#[tokio::test]
async fn test_credential_vault_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path().join("test_vault.vault");
    
    // Set up environment variables
    env::set_var("LAZABOT_MASTER_KEY", "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    env::set_var("LAZABOT_USERNAME", "test@example.com");
    env::set_var("LAZABOT_PASSWORD", "testpassword123");
    env::set_var("LAZABOT_CAPTCHA_API_KEY", "test_api_key_12345");
    
    // Create credential manager
    let mut manager = CredentialManager::new(vault_path.to_str().unwrap())?;
    
    // Load from environment
    manager.load_from_env()?;
    
    // Save vault
    manager.save_vault()?;
    
    // Verify vault was created
    assert!(vault_path.exists());
    
    // Test account retrieval
    let account = manager.get_account("default_account")?;
    assert_eq!(account.username, "test@example.com");
    assert_eq!(account.password, "testpassword123");
    
    // Test captcha credentials
    let captcha = manager.get_captcha().unwrap();
    assert_eq!(captcha.api_key, "test_api_key_12345");
    
    // Test account IDs
    let account_ids = manager.get_account_ids();
    assert_eq!(account_ids.len(), 1);
    assert_eq!(account_ids[0], "default_account");
    
    Ok(())
}

#[tokio::test]
async fn test_multiple_accounts() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path().join("test_vault.vault");
    
    // Set up environment variables for multiple accounts
    env::set_var("LAZABOT_MASTER_KEY", "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    env::set_var("LAZABOT_USERNAME", "test1@example.com");
    env::set_var("LAZABOT_PASSWORD", "password1");
    env::set_var("LAZABOT_USERNAME_2", "test2@example.com");
    env::set_var("LAZABOT_PASSWORD_2", "password2");
    env::set_var("LAZABOT_CAPTCHA_API_KEY", "test_api_key_12345");
    
    // Create credential manager
    let mut manager = CredentialManager::new(vault_path.to_str().unwrap())?;
    
    // Load from environment
    manager.load_from_env()?;
    
    // Save vault
    manager.save_vault()?;
    
    // Test multiple accounts
    let account1 = manager.get_account("default_account")?;
    assert_eq!(account1.username, "test1@example.com");
    
    let account2 = manager.get_account("account_2")?;
    assert_eq!(account2.username, "test2@example.com");
    
    // Test account IDs
    let account_ids = manager.get_account_ids();
    assert_eq!(account_ids.len(), 2);
    assert!(account_ids.contains(&"default_account".to_string()));
    assert!(account_ids.contains(&"account_2".to_string()));
    
    Ok(())
}

#[tokio::test]
async fn test_proxy_credentials() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path().join("test_vault.vault");
    
    // Set up environment variables
    env::set_var("LAZABOT_MASTER_KEY", "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    env::set_var("LAZABOT_USERNAME", "test@example.com");
    env::set_var("LAZABOT_PASSWORD", "testpassword123");
    env::set_var("LAZABOT_PROXY_HOST", "proxy.example.com");
    env::set_var("LAZABOT_PROXY_PORT", "8080");
    env::set_var("LAZABOT_PROXY_USERNAME", "proxy_user");
    env::set_var("LAZABOT_PROXY_PASSWORD", "proxy_pass");
    env::set_var("LAZABOT_CAPTCHA_API_KEY", "test_api_key_12345");
    
    // Create credential manager
    let mut manager = CredentialManager::new(vault_path.to_str().unwrap())?;
    
    // Load from environment
    manager.load_from_env()?;
    
    // Save vault
    manager.save_vault()?;
    
    // Test proxy credentials
    let proxy = manager.get_proxy("default_proxy").unwrap();
    assert_eq!(proxy.host, "proxy.example.com");
    assert_eq!(proxy.port, 8080);
    assert_eq!(proxy.username, Some("proxy_user".to_string()));
    assert_eq!(proxy.password, Some("proxy_pass".to_string()));
    
    // Test proxy IDs
    let proxy_ids = manager.get_proxy_ids();
    assert_eq!(proxy_ids.len(), 1);
    assert_eq!(proxy_ids[0], "default_proxy");
    
    Ok(())
}

#[tokio::test]
async fn test_environment_validation() -> Result<()> {
    let validator = EnvValidator::new();
    
    // Set up valid environment
    env::set_var("LAZABOT_MASTER_KEY", "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    env::set_var("LAZABOT_USERNAME", "test@example.com");
    env::set_var("LAZABOT_PASSWORD", "testpassword123");
    env::set_var("LAZABOT_CAPTCHA_API_KEY", "test_api_key_12345");
    
    // Validate environment
    let report = validator.validate_all()?;
    assert!(!report.has_errors());
    
    Ok(())
}

#[tokio::test]
async fn test_host_config_creation() -> Result<()> {
    // Test creating a simple host config
    let config = HostConfig {
        host_id: "ubuntu".to_string(),
        environment: "production".to_string(),
        overrides: serde_json::json!({
            "data_dir": "/opt/lazabot/data"
        }),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        last_updated: "2024-01-01T00:00:00Z".to_string(),
    };
    
    assert_eq!(config.host_id, "ubuntu");
    assert_eq!(config.environment, "production");
    assert!(config.is_production());
    assert!(config.overrides["data_dir"].is_string());
    assert_eq!(config.overrides["data_dir"], "/opt/lazabot/data");
    
    Ok(())
}

#[tokio::test]
async fn test_host_config_serialization() -> Result<()> {
    let config = HostConfig {
        host_id: "ubuntu".to_string(),
        environment: "production".to_string(),
        overrides: serde_json::json!({
            "data_dir": "/opt/lazabot/data"
        }),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        last_updated: "2024-01-01T00:00:00Z".to_string(),
    };
    
    // Test serialization
    let toml_str = toml::to_string_pretty(&config)?;
    assert!(toml_str.contains("ubuntu"));
    assert!(toml_str.contains("production"));
    assert!(toml_str.contains("/opt/lazabot/data"));
    
    // Test deserialization
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test_config.toml");
    std::fs::write(&file_path, toml_str)?;
    
    let loaded_config: HostConfig = toml::from_str(&std::fs::read_to_string(&file_path)?)?;
    assert_eq!(loaded_config.host_id, config.host_id);
    assert_eq!(loaded_config.environment, config.environment);
    assert_eq!(loaded_config.environment, config.environment);
    
    Ok(())
}

#[tokio::test]
async fn test_validation_functions() -> Result<()> {
    use lazabot::config::validation::*;
    
    // Test master key validation
    assert!(validate_master_key("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef").is_ok());
    assert!(validate_master_key("invalid").is_err());
    assert!(validate_master_key("").is_err());
    
    // Test email validation
    assert!(validate_email("test@example.com").is_ok());
    assert!(validate_email("invalid-email").is_err());
    assert!(validate_email("").is_err());
    
    // Test password validation
    assert!(validate_password("password123").is_ok());
    assert!(validate_password("12345").is_err());
    assert!(validate_password("").is_err());
    
    // Test log level validation
    assert!(validate_log_level("info").is_ok());
    assert!(validate_log_level("DEBUG").is_ok());
    assert!(validate_log_level("invalid").is_err());
    
    // Test port validation
    assert!(validate_port("8080").is_ok());
    assert!(validate_port("0").is_err());
    assert!(validate_port("invalid").is_err());
    assert!(validate_port("70000").is_err());
    
    // Test proxy type validation
    assert!(validate_proxy_type("http").is_ok());
    assert!(validate_proxy_type("SOCKS5").is_ok());
    assert!(validate_proxy_type("invalid").is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_validation_report() -> Result<()> {
    use lazabot::config::validation::ValidationReport;
    
    let mut report = ValidationReport::new();
    report.add_success("TEST_VAR", "Test variable", Some("test_value"));
    report.add_error("ERROR_VAR", "Error variable", "Test error");
    report.add_info("INFO_VAR", "Info variable", "Not set");
    
    assert_eq!(report.successes.len(), 1);
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.infos.len(), 1);
    assert!(report.has_errors());
    
    Ok(())
}
