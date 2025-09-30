use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Host-specific configuration overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {
    /// Host identifier (e.g., "production", "staging", "development")
    pub host_id: String,
    /// Environment-specific settings
    pub environment: String,
    /// Override settings
    pub overrides: serde_json::Value,
    /// Created timestamp
    pub created_at: String,
    /// Last updated timestamp
    pub last_updated: String,
}

/// Host configuration manager
pub struct HostConfigManager {
    config_dir: String,
}

impl HostConfigManager {
    /// Create a new host configuration manager
    pub fn new(config_dir: &str) -> Self {
        Self {
            config_dir: config_dir.to_string(),
        }
    }

    /// Detect the current host and load appropriate configuration
    pub fn detect_and_load(&self) -> Result<HostConfig> {
        let host_id = self.detect_host()?;
        self.load_host_config(&host_id)
    }

    /// Detect the current host based on environment variables and system properties
    fn detect_host(&self) -> Result<String> {
        // Check environment variable first
        if let Ok(env_host) = std::env::var("LAZABOT_HOST") {
            return Ok(env_host);
        }

        // Check for common production indicators
        if std::env::var("NODE_ENV").unwrap_or_default() == "production" {
            return Ok("production".to_string());
        }

        if std::env::var("RUST_ENV").unwrap_or_default() == "production" {
            return Ok("production".to_string());
        }

        // Check for staging indicators
        if std::env::var("NODE_ENV").unwrap_or_default() == "staging" {
            return Ok("staging".to_string());
        }

        if std::env::var("RUST_ENV").unwrap_or_default() == "staging" {
            return Ok("staging".to_string());
        }

        // Check for development indicators
        if std::env::var("NODE_ENV").unwrap_or_default() == "development" {
            return Ok("development".to_string());
        }

        if std::env::var("RUST_ENV").unwrap_or_default() == "development" {
            return Ok("development".to_string());
        }

        // Check for Docker environment
        if std::env::var("DOCKER").unwrap_or_default() == "true" {
            return Ok("docker".to_string());
        }

        // Check for CI environment
        if std::env::var("CI").unwrap_or_default() == "true" {
            return Ok("ci".to_string());
        }

        // Default to development
        Ok("development".to_string())
    }

    /// Load host-specific configuration
    fn load_host_config(&self, host_id: &str) -> Result<HostConfig> {
        let config_path = format!("{}/hosts/{}.toml", self.config_dir, host_id);
        
        if !Path::new(&config_path).exists() {
            // Create default host configuration
            return self.create_default_host_config(host_id);
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to read host config {}: {}", config_path, e))?;

        let config: HostConfig = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse host config {}: {}", config_path, e))?;

        Ok(config)
    }

    /// Create a default host configuration
    fn create_default_host_config(&self, host_id: &str) -> Result<HostConfig> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let config = HostConfig {
            host_id: host_id.to_string(),
            environment: self.get_environment_from_host_id(host_id),
            overrides: self.get_default_overrides(host_id),
            created_at: now.clone(),
            last_updated: now,
        };

        // Save the default configuration
        self.save_host_config(&config)?;

        Ok(config)
    }

    /// Get environment from host ID
    fn get_environment_from_host_id(&self, host_id: &str) -> String {
        match host_id {
            "production" => "production".to_string(),
            "staging" => "staging".to_string(),
            "development" => "development".to_string(),
            "docker" => "production".to_string(),
            "ci" => "testing".to_string(),
            _ => "development".to_string(),
        }
    }

    /// Get default overrides for host
    fn get_default_overrides(&self, host_id: &str) -> serde_json::Value {
        match host_id {
            "production" => serde_json::json!({
                "bot": {
                    "debug": false,
                    "default_delay": 2000
                },
                "monitoring": {
                    "enable_logging": true,
                    "log_level": "info"
                },
                "stealth": {
                    "random_delays": true,
                    "proxy_rotation": true
                }
            }),
            "staging" => serde_json::json!({
                "bot": {
                    "debug": true,
                    "default_delay": 1500
                },
                "monitoring": {
                    "enable_logging": true,
                    "log_level": "debug"
                },
                "stealth": {
                    "random_delays": true,
                    "proxy_rotation": false
                }
            }),
            "development" => serde_json::json!({
                "bot": {
                    "debug": true,
                    "default_delay": 500
                },
                "monitoring": {
                    "enable_logging": true,
                    "log_level": "trace"
                },
                "stealth": {
                    "random_delays": false,
                    "proxy_rotation": false
                }
            }),
            "docker" => serde_json::json!({
                "bot": {
                    "debug": false,
                    "default_delay": 1000
                },
                "monitoring": {
                    "enable_logging": true,
                    "log_level": "info"
                },
                "stealth": {
                    "random_delays": true,
                    "proxy_rotation": true
                }
            }),
            "ci" => serde_json::json!({
                "bot": {
                    "debug": false,
                    "default_delay": 100
                },
                "monitoring": {
                    "enable_logging": false,
                    "log_level": "error"
                },
                "stealth": {
                    "random_delays": false,
                    "proxy_rotation": false
                }
            }),
            _ => serde_json::json!({}),
        }
    }

    /// Save host configuration to file
    fn save_host_config(&self, config: &HostConfig) -> Result<()> {
        // Ensure the hosts directory exists
        let hosts_dir = format!("{}/hosts", self.config_dir);
        fs::create_dir_all(&hosts_dir)
            .map_err(|e| anyhow::anyhow!("Failed to create hosts directory {}: {}", hosts_dir, e))?;

        let config_path = format!("{}/{}.toml", hosts_dir, config.host_id);
        let content = toml::to_string_pretty(config)
            .map_err(|e| anyhow::anyhow!("Failed to serialize host config: {}", e))?;

        fs::write(&config_path, content)
            .map_err(|e| anyhow::anyhow!("Failed to write host config {}: {}", config_path, e))?;

        Ok(())
    }

    /// List available host configurations
    pub fn list_host_configs(&self) -> Result<Vec<String>> {
        let hosts_dir = format!("{}/hosts", self.config_dir);
        
        if !Path::new(&hosts_dir).exists() {
            return Ok(Vec::new());
        }

        let mut hosts = Vec::new();
        let entries = fs::read_dir(&hosts_dir)
            .map_err(|e| anyhow::anyhow!("Failed to read hosts directory {}: {}", hosts_dir, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow::anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    hosts.push(stem.to_string());
                }
            }
        }

        Ok(hosts)
    }

    /// Get host configuration by ID
    pub fn get_host_config(&self, host_id: &str) -> Result<HostConfig> {
        self.load_host_config(host_id)
    }

    /// Update host configuration
    pub fn update_host_config(&self, config: &HostConfig) -> Result<()> {
        let mut updated_config = config.clone();
        updated_config.last_updated = chrono::Utc::now().to_rfc3339();
        self.save_host_config(&updated_config)
    }

    /// Delete host configuration
    pub fn delete_host_config(&self, host_id: &str) -> Result<()> {
        let config_path = format!("{}/hosts/{}.toml", self.config_dir, host_id);
        
        if Path::new(&config_path).exists() {
            fs::remove_file(&config_path)
                .map_err(|e| anyhow::anyhow!("Failed to delete host config {}: {}", config_path, e))?;
        }

        Ok(())
    }
}

impl HostConfig {
    /// Apply overrides to a configuration object
    pub fn apply_overrides<T>(&self, config: T) -> T 
    where
        T: serde::de::DeserializeOwned + serde::Serialize,
    {
        // This is a simplified implementation
        // In a real implementation, you would use a more sophisticated merging strategy
        // For now, we'll just return the original config
        config
    }

    /// Check if this host config is for production
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if this host config is for development
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if this host config is for staging
    pub fn is_staging(&self) -> bool {
        self.environment == "staging"
    }

    /// Get a specific override value
    pub fn get_override<T>(&self, path: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // This is a simplified implementation
        // In a real implementation, you would use JSONPath or similar
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_host() {
        let manager = HostConfigManager::new("test_config");
        
        // Test environment variable detection
        std::env::set_var("LAZABOT_HOST", "test_host");
        assert_eq!(manager.detect_host().unwrap(), "test_host");
        std::env::remove_var("LAZABOT_HOST");

        // Test production detection
        std::env::set_var("NODE_ENV", "production");
        assert_eq!(manager.detect_host().unwrap(), "production");
        std::env::remove_var("NODE_ENV");

        // Test development detection
        std::env::set_var("RUST_ENV", "development");
        assert_eq!(manager.detect_host().unwrap(), "development");
        std::env::remove_var("RUST_ENV");

        // Test Docker detection
        std::env::set_var("DOCKER", "true");
        assert_eq!(manager.detect_host().unwrap(), "docker");
        std::env::remove_var("DOCKER");

        // Test CI detection
        std::env::set_var("CI", "true");
        assert_eq!(manager.detect_host().unwrap(), "ci");
        std::env::remove_var("CI");
    }

    #[test]
    fn test_get_environment_from_host_id() {
        let manager = HostConfigManager::new("test_config");
        
        assert_eq!(manager.get_environment_from_host_id("production"), "production");
        assert_eq!(manager.get_environment_from_host_id("staging"), "staging");
        assert_eq!(manager.get_environment_from_host_id("development"), "development");
        assert_eq!(manager.get_environment_from_host_id("docker"), "production");
        assert_eq!(manager.get_environment_from_host_id("ci"), "testing");
        assert_eq!(manager.get_environment_from_host_id("unknown"), "development");
    }

    #[test]
    fn test_get_default_overrides() {
        let manager = HostConfigManager::new("test_config");
        
        let prod_overrides = manager.get_default_overrides("production");
        assert!(prod_overrides["bot"]["debug"].as_bool().unwrap() == false);
        
        let dev_overrides = manager.get_default_overrides("development");
        assert!(dev_overrides["bot"]["debug"].as_bool().unwrap() == true);
    }

    #[test]
    fn test_host_config_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_str().unwrap();
        let manager = HostConfigManager::new(config_dir);
        
        let config = manager.create_default_host_config("test").unwrap();
        assert_eq!(config.host_id, "test");
        assert_eq!(config.environment, "development");
        assert!(!config.overrides.is_null());
    }

    #[test]
    fn test_host_config_methods() {
        let config = HostConfig {
            host_id: "test".to_string(),
            environment: "production".to_string(),
            overrides: serde_json::json!({}),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            last_updated: "2024-01-01T00:00:00Z".to_string(),
        };

        assert!(config.is_production());
        assert!(!config.is_development());
        assert!(!config.is_staging());
    }
}
