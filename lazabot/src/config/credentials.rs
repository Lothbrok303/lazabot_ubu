use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use thiserror::Error;

use crate::config::encryption::EncryptionManager;

/// Credential management errors
#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid credential format: {0}")]
    InvalidFormat(String),
    #[error("Encryption error: {0}")]
    EncryptionError(#[from] crate::config::encryption::EncryptionError),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
}

/// Result type for credential operations
pub type CredentialResult<T> = Result<T, CredentialError>;

/// Lazada account credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazadaCredentials {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub account_id: String,
}

/// 2Captcha API credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaCredentials {
    pub api_key: String,
    pub endpoint: Option<String>,
}

/// Proxy credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyCredentials {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub proxy_type: String, // http, socks5, etc.
}

/// Master encryption key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterKey {
    pub key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Secure credential vault for storing encrypted credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialVault {
    pub accounts: HashMap<String, LazadaCredentials>,
    pub captcha: Option<CaptchaCredentials>,
    pub proxies: HashMap<String, ProxyCredentials>,
    pub master_key: MasterKey,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl CredentialVault {
    /// Create a new empty vault
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            accounts: HashMap::new(),
            captcha: None,
            proxies: HashMap::new(),
            master_key: MasterKey {
                key: String::new(),
                created_at: now,
            },
            created_at: now,
            last_updated: now,
        }
    }

    /// Add a Lazada account to the vault
    pub fn add_account(&mut self, account_id: String, credentials: LazadaCredentials) {
        self.accounts.insert(account_id, credentials);
        self.last_updated = chrono::Utc::now();
    }

    /// Add proxy credentials to the vault
    pub fn add_proxy(&mut self, proxy_id: String, credentials: ProxyCredentials) {
        self.proxies.insert(proxy_id, credentials);
        self.last_updated = chrono::Utc::now();
    }

    /// Set captcha credentials
    pub fn set_captcha(&mut self, credentials: CaptchaCredentials) {
        self.captcha = Some(credentials);
        self.last_updated = chrono::Utc::now();
    }

    /// Get account credentials by ID
    pub fn get_account(&self, account_id: &str) -> CredentialResult<&LazadaCredentials> {
        self.accounts.get(account_id)
            .ok_or_else(|| CredentialError::AccountNotFound(account_id.to_string()))
    }

    /// Get all account IDs
    pub fn get_account_ids(&self) -> Vec<String> {
        self.accounts.keys().cloned().collect()
    }

    /// Get proxy credentials by ID
    pub fn get_proxy(&self, proxy_id: &str) -> Option<&ProxyCredentials> {
        self.proxies.get(proxy_id)
    }

    /// Get all proxy IDs
    pub fn get_proxy_ids(&self) -> Vec<String> {
        self.proxies.keys().cloned().collect()
    }

    /// Get captcha credentials
    pub fn get_captcha(&self) -> Option<&CaptchaCredentials> {
        self.captcha.as_ref()
    }
}

/// Credential manager for handling secure credential operations
pub struct CredentialManager {
    vault: CredentialVault,
    encryption_manager: EncryptionManager,
    vault_path: String,
}

impl CredentialManager {
    /// Create a new credential manager
    pub fn new(vault_path: &str) -> CredentialResult<Self> {
        let encryption_manager = EncryptionManager::new()?;
        let vault = Self::load_vault(vault_path, &encryption_manager)?;
        
        Ok(Self {
            vault,
            encryption_manager,
            vault_path: vault_path.to_string(),
        })
    }

    /// Load vault from file or create new one
    fn load_vault(vault_path: &str, encryption_manager: &EncryptionManager) -> CredentialResult<CredentialVault> {
        if std::path::Path::new(vault_path).exists() {
            let content = std::fs::read_to_string(vault_path)
                .context("Failed to read vault file")?;
            
            // Decrypt the vault content
            let decrypted_content = encryption_manager.decrypt(&content)?;
            let vault: CredentialVault = serde_json::from_str(&decrypted_content)
                .context("Failed to parse vault JSON")?;
            
            Ok(vault)
        } else {
            Ok(CredentialVault::new())
        }
    }

    /// Save vault to file
    pub fn save_vault(&self) -> CredentialResult<()> {
        let json_content = serde_json::to_string_pretty(&self.vault)
            .context("Failed to serialize vault")?;
        
        let encrypted_content = self.encryption_manager.encrypt(&json_content)?;
        
        std::fs::write(&self.vault_path, encrypted_content)
            .context("Failed to write vault file")?;
        
        Ok(())
    }

    /// Load credentials from environment variables
    pub fn load_from_env(&mut self) -> CredentialResult<()> {
        // Load master key
        let master_key = env::var("LAZABOT_MASTER_KEY")
            .map_err(|_| CredentialError::MissingEnvVar("LAZABOT_MASTER_KEY".to_string()))?;
        
        self.vault.master_key = MasterKey {
            key: master_key,
            created_at: chrono::Utc::now(),
        };

        // Load 2Captcha credentials
        if let Ok(api_key) = env::var("LAZABOT_CAPTCHA_API_KEY") {
            let captcha_creds = CaptchaCredentials {
                api_key,
                endpoint: env::var("LAZABOT_CAPTCHA_ENDPOINT").ok(),
            };
            self.vault.set_captcha(captcha_creds);
        }

        // Load Lazada accounts (support multiple accounts)
        self.load_lazada_accounts_from_env()?;

        // Load proxy credentials
        self.load_proxy_credentials_from_env()?;

        Ok(())
    }

    /// Load Lazada accounts from environment variables
    fn load_lazada_accounts_from_env(&mut self) -> CredentialResult<()> {
        // Support multiple accounts with numbered environment variables
        let mut account_index = 1;
        
        loop {
            let username_var = format!("LAZABOT_ACCOUNT_{}_USERNAME", account_index);
            let password_var = format!("LAZABOT_ACCOUNT_{}_PASSWORD", account_index);
            let email_var = format!("LAZABOT_ACCOUNT_{}_EMAIL", account_index);
            
            let username = match env::var(&username_var) {
                Ok(val) => val,
                Err(_) => break, // No more accounts
            };
            
            let password = env::var(&password_var)
                .map_err(|_| CredentialError::MissingEnvVar(password_var))?;
            
            let email = env::var(&email_var).ok();
            
            let account_id = format!("account_{}", account_index);
            let credentials = LazadaCredentials {
                username,
                password,
                email,
                account_id: account_id.clone(),
            };
            
            self.vault.add_account(account_id, credentials);
            account_index += 1;
        }

        // Also support single account with LAZABOT_USERNAME/LAZABOT_PASSWORD
        if account_index == 1 {
            if let (Ok(username), Ok(password)) = (
                env::var("LAZABOT_USERNAME"),
                env::var("LAZABOT_PASSWORD")
            ) {
                let email = env::var("LAZABOT_EMAIL").ok();
                let account_id = "default_account".to_string();
                let credentials = LazadaCredentials {
                    username,
                    password,
                    email,
                    account_id: account_id.clone(),
                };
                self.vault.add_account(account_id, credentials);
            }
        }

        Ok(())
    }

    /// Load proxy credentials from environment variables
    fn load_proxy_credentials_from_env(&mut self) -> CredentialResult<()> {
        // Support multiple proxies with numbered environment variables
        let mut proxy_index = 1;
        
        loop {
            let host_var = format!("LAZABOT_PROXY_{}_HOST", proxy_index);
            let port_var = format!("LAZABOT_PROXY_{}_PORT", proxy_index);
            let username_var = format!("LAZABOT_PROXY_{}_USERNAME", proxy_index);
            let password_var = format!("LAZABOT_PROXY_{}_PASSWORD", proxy_index);
            let type_var = format!("LAZABOT_PROXY_{}_TYPE", proxy_index);
            
            let host = match env::var(&host_var) {
                Ok(val) => val,
                Err(_) => break, // No more proxies
            };
            
            let port = env::var(&port_var)
                .map_err(|_| CredentialError::MissingEnvVar(port_var))?
                .parse::<u16>()
                .map_err(|e| CredentialError::InvalidFormat(format!("Invalid port: {}", e)))?;
            
            let username = env::var(&username_var).ok();
            let password = env::var(&password_var).ok();
            let proxy_type = env::var(&type_var).unwrap_or_else(|_| "http".to_string());
            
            let proxy_id = format!("proxy_{}", proxy_index);
            let credentials = ProxyCredentials {
                host,
                port,
                username,
                password,
                proxy_type,
            };
            
            self.vault.add_proxy(proxy_id, credentials);
            proxy_index += 1;
        }

        // Also support single proxy with LAZABOT_PROXY_HOST/LAZABOT_PROXY_PORT
        if proxy_index == 1 {
            if let (Ok(host), Ok(port_str)) = (
                env::var("LAZABOT_PROXY_HOST"),
                env::var("LAZABOT_PROXY_PORT")
            ) {
                let port = port_str.parse::<u16>()
                    .map_err(|e| CredentialError::InvalidFormat(format!("Invalid port: {}", e)))?;
                let username = env::var("LAZABOT_PROXY_USERNAME").ok();
                let password = env::var("LAZABOT_PROXY_PASSWORD").ok();
                let proxy_type = env::var("LAZABOT_PROXY_TYPE").unwrap_or_else(|_| "http".to_string());
                
                let proxy_id = "default_proxy".to_string();
                let credentials = ProxyCredentials {
                    host,
                    port,
                    username,
                    password,
                    proxy_type,
                };
                
                self.vault.add_proxy(proxy_id, credentials);
            }
        }

        Ok(())
    }

    /// Validate that all required environment variables are set
    pub fn validate_env_vars() -> CredentialResult<()> {
        let mut missing_vars = Vec::new();

        // Check master key
        if env::var("LAZABOT_MASTER_KEY").is_err() {
            missing_vars.push("LAZABOT_MASTER_KEY");
        }

        // Check for at least one account
        let has_single_account = env::var("LAZABOT_USERNAME").is_ok() && env::var("LAZABOT_PASSWORD").is_ok();
        let has_numbered_account = env::var("LAZABOT_ACCOUNT_1_USERNAME").is_ok() && env::var("LAZABOT_ACCOUNT_1_PASSWORD").is_ok();
        
        if !has_single_account && !has_numbered_account {
            missing_vars.push("LAZABOT_USERNAME and LAZABOT_PASSWORD (or LAZABOT_ACCOUNT_1_USERNAME and LAZABOT_ACCOUNT_1_PASSWORD)");
        }

        // Check captcha API key
        if env::var("LAZABOT_CAPTCHA_API_KEY").is_err() {
            missing_vars.push("LAZABOT_CAPTCHA_API_KEY");
        }

        if !missing_vars.is_empty() {
            return Err(CredentialError::MissingEnvVar(
                format!("Missing required environment variables: {}", missing_vars.join(", "))
            ));
        }

        Ok(())
    }

    /// Get account credentials by ID
    pub fn get_account(&self, account_id: &str) -> CredentialResult<&LazadaCredentials> {
        self.vault.get_account(account_id)
    }

    /// Get all account IDs
    pub fn get_account_ids(&self) -> Vec<String> {
        self.vault.get_account_ids()
    }

    /// Get proxy credentials by ID
    pub fn get_proxy(&self, proxy_id: &str) -> Option<&ProxyCredentials> {
        self.vault.get_proxy(proxy_id)
    }

    /// Get all proxy IDs
    pub fn get_proxy_ids(&self) -> Vec<String> {
        self.vault.get_proxy_ids()
    }

    /// Get captcha credentials
    pub fn get_captcha(&self) -> Option<&CaptchaCredentials> {
        self.vault.get_captcha()
    }

    /// Get vault info
    pub fn get_vault_info(&self) -> &CredentialVault {
        &self.vault
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_credential_vault_creation() {
        let vault = CredentialVault::new();
        assert!(vault.accounts.is_empty());
        assert!(vault.captcha.is_none());
        assert!(vault.proxies.is_empty());
    }

    #[test]
    fn test_credential_vault_operations() {
        let mut vault = CredentialVault::new();
        
        let credentials = LazadaCredentials {
            username: "test@example.com".to_string(),
            password: "password123".to_string(),
            email: Some("test@example.com".to_string()),
            account_id: "test_account".to_string(),
        };
        
        vault.add_account("test_account".to_string(), credentials);
        assert_eq!(vault.accounts.len(), 1);
        
        let retrieved = vault.get_account("test_account").unwrap();
        assert_eq!(retrieved.username, "test@example.com");
    }

    #[test]
    fn test_env_validation() {
        // Clear environment variables
        env::remove_var("LAZABOT_MASTER_KEY");
        env::remove_var("LAZABOT_USERNAME");
        env::remove_var("LAZABOT_PASSWORD");
        env::remove_var("LAZABOT_CAPTCHA_API_KEY");
        
        let result = CredentialManager::validate_env_vars();
        assert!(result.is_err());
        
        // Set required variables
        env::set_var("LAZABOT_MASTER_KEY", "test_key_32_bytes_long_123456789012");
        env::set_var("LAZABOT_USERNAME", "test@example.com");
        env::set_var("LAZABOT_PASSWORD", "password123");
        env::set_var("LAZABOT_CAPTCHA_API_KEY", "test_api_key");
        
        let result = CredentialManager::validate_env_vars();
        assert!(result.is_ok());
    }
}
