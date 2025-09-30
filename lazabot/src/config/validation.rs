use anyhow::Result;
use std::env;
use thiserror::Error;

use crate::config::credentials::CredentialManager;

/// Validation errors
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable format: {0}")]
    InvalidFormat(String),
    #[error("Configuration validation failed: {0}")]
    ConfigValidationFailed(String),
    #[error("Credential validation failed: {0}")]
    CredentialValidationFailed(String),
}

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Environment variable validator
pub struct EnvValidator {
    required_vars: Vec<RequiredVar>,
    optional_vars: Vec<OptionalVar>,
}

/// Required environment variable definition
#[derive(Debug, Clone)]
pub struct RequiredVar {
    pub name: String,
    pub description: String,
    pub validation_fn: Option<fn(&str) -> ValidationResult<()>>,
}

/// Optional environment variable definition
#[derive(Debug, Clone)]
pub struct OptionalVar {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub validation_fn: Option<fn(&str) -> ValidationResult<()>>,
}

impl EnvValidator {
    /// Create a new environment validator with default required variables
    pub fn new() -> Self {
        let mut validator = Self {
            required_vars: Vec::new(),
            optional_vars: Vec::new(),
        };

        // Add required variables
        validator.add_required_var(
            "LAZABOT_MASTER_KEY",
            "Master encryption key (32 bytes, hex-encoded)",
            Some(validate_master_key),
        );

        validator.add_required_var(
            "LAZABOT_CAPTCHA_API_KEY",
            "2Captcha API key for solving captchas",
            Some(validate_api_key),
        );

        // Add account variables (at least one account required)
        validator.add_account_variables();

        // Add optional variables
        validator.add_optional_var(
            "LAZABOT_LOG_LEVEL",
            "Logging level (trace, debug, info, warn, error)",
            Some("info".to_string()),
            Some(validate_log_level),
        );

        validator.add_optional_var(
            "LAZABOT_DATA_DIR",
            "Data directory path",
            Some("./data".to_string()),
            Some(validate_directory_path),
        );

        validator.add_optional_var(
            "LAZABOT_LOG_DIR",
            "Log directory path",
            Some("./logs".to_string()),
            Some(validate_directory_path),
        );

        validator.add_optional_var(
            "LAZABOT_VAULT_PATH",
            "Path to encrypted credential vault",
            Some("./data/credentials.vault".to_string()),
            None,
        );

        validator.add_optional_var(
            "LAZABOT_DATABASE_URL",
            "Database connection URL",
            Some("sqlite://./data/lazabot.db".to_string()),
            None,
        );

        // Add proxy variables (optional)
        validator.add_proxy_variables();

        validator
    }

    /// Add a required environment variable
    pub fn add_required_var(
        &mut self,
        name: &str,
        description: &str,
        validation_fn: Option<fn(&str) -> ValidationResult<()>>,
    ) {
        self.required_vars.push(RequiredVar {
            name: name.to_string(),
            description: description.to_string(),
            validation_fn,
        });
    }

    /// Add an optional environment variable
    pub fn add_optional_var(
        &mut self,
        name: &str,
        description: &str,
        default_value: Option<String>,
        validation_fn: Option<fn(&str) -> ValidationResult<()>>,
    ) {
        self.optional_vars.push(OptionalVar {
            name: name.to_string(),
            description: description.to_string(),
            default_value,
            validation_fn,
        });
    }

    /// Add account-related environment variables
    fn add_account_variables(&mut self) {
        // Single account variables
        self.add_required_var(
            "LAZABOT_USERNAME",
            "Lazada account username/email",
            Some(validate_email),
        );

        self.add_required_var(
            "LAZABOT_PASSWORD",
            "Lazada account password",
            Some(validate_password),
        );

        self.add_optional_var(
            "LAZABOT_EMAIL",
            "Lazada account email (if different from username)",
            None,
            Some(validate_email),
        );

        // Multiple account support (optional)
        for i in 1..=10 {
            self.add_optional_var(
                &format!("LAZABOT_ACCOUNT_{}_USERNAME", i),
                &format!("Lazada account {} username/email", i),
                None,
                Some(validate_email),
            );

            self.add_optional_var(
                &format!("LAZABOT_ACCOUNT_{}_PASSWORD", i),
                &format!("Lazada account {} password", i),
                None,
                Some(validate_password),
            );

            self.add_optional_var(
                &format!("LAZABOT_ACCOUNT_{}_EMAIL", i),
                &format!("Lazada account {} email (if different from username)", i),
                None,
                Some(validate_email),
            );
        }
    }

    /// Add proxy-related environment variables
    fn add_proxy_variables(&mut self) {
        // Single proxy variables
        self.add_optional_var(
            "LAZABOT_PROXY_HOST",
            "Proxy server hostname or IP",
            None,
            Some(validate_hostname),
        );

        self.add_optional_var(
            "LAZABOT_PROXY_PORT",
            "Proxy server port",
            None,
            Some(validate_port),
        );

        self.add_optional_var(
            "LAZABOT_PROXY_USERNAME",
            "Proxy server username",
            None,
            None,
        );

        self.add_optional_var(
            "LAZABOT_PROXY_PASSWORD",
            "Proxy server password",
            None,
            None,
        );

        self.add_optional_var(
            "LAZABOT_PROXY_TYPE",
            "Proxy type (http, socks5, socks4)",
            Some("http".to_string()),
            Some(validate_proxy_type),
        );

        // Multiple proxy support (optional)
        for i in 1..=5 {
            self.add_optional_var(
                &format!("LAZABOT_PROXY_{}_HOST", i),
                &format!("Proxy server {} hostname or IP", i),
                None,
                Some(validate_hostname),
            );

            self.add_optional_var(
                &format!("LAZABOT_PROXY_{}_PORT", i),
                &format!("Proxy server {} port", i),
                None,
                Some(validate_port),
            );

            self.add_optional_var(
                &format!("LAZABOT_PROXY_{}_USERNAME", i),
                &format!("Proxy server {} username", i),
                None,
                None,
            );

            self.add_optional_var(
                &format!("LAZABOT_PROXY_{}_PASSWORD", i),
                &format!("Proxy server {} password", i),
                None,
                None,
            );

            self.add_optional_var(
                &format!("LAZABOT_PROXY_{}_TYPE", i),
                &format!("Proxy server {} type", i),
                Some("http".to_string()),
                Some(validate_proxy_type),
            );
        }
    }

    /// Validate all environment variables
    pub fn validate_all(&self) -> ValidationResult<ValidationReport> {
        let mut report = ValidationReport::new();
        let mut has_errors = false;

        // Validate required variables
        for var in &self.required_vars {
            match env::var(&var.name) {
                Ok(value) => {
                    if let Some(validator) = var.validation_fn {
                        match validator(&value) {
                            Ok(()) => {
                                report.add_success(&var.name, &var.description, Some(&value));
                            }
                            Err(e) => {
                                report.add_error(&var.name, &var.description, &e.to_string());
                                has_errors = true;
                            }
                        }
                    } else {
                        report.add_success(&var.name, &var.description, Some(&value));
                    }
                }
                Err(_) => {
                    report.add_error(&var.name, &var.description, "Variable not set");
                    has_errors = true;
                }
            }
        }

        // Validate optional variables
        for var in &self.optional_vars {
            match env::var(&var.name) {
                Ok(value) => {
                    if let Some(validator) = var.validation_fn {
                        match validator(&value) {
                            Ok(()) => {
                                report.add_success(&var.name, &var.description, Some(&value));
                            }
                            Err(e) => {
                                report.add_error(&var.name, &var.description, &e.to_string());
                                has_errors = true;
                            }
                        }
                    } else {
                        report.add_success(&var.name, &var.description, Some(&value));
                    }
                }
                Err(_) => {
                    let status = if let Some(default) = &var.default_value {
                        format!("Not set (using default: {})", default)
                    } else {
                        "Not set (optional)".to_string()
                    };
                    report.add_info(&var.name, &var.description, &status);
                }
            }
        }

        // Check for at least one account
        let has_single_account = env::var("LAZABOT_USERNAME").is_ok() && env::var("LAZABOT_PASSWORD").is_ok();
        let has_numbered_account = env::var("LAZABOT_ACCOUNT_1_USERNAME").is_ok() && env::var("LAZABOT_ACCOUNT_1_PASSWORD").is_ok();
        
        if !has_single_account && !has_numbered_account {
            report.add_error(
                "ACCOUNT_CONFIG",
                "At least one Lazada account must be configured",
                "No valid account configuration found. Set LAZABOT_USERNAME/LAZABOT_PASSWORD or LAZABOT_ACCOUNT_1_USERNAME/LAZABOT_ACCOUNT_1_PASSWORD"
            );
            has_errors = true;
        }

        if has_errors {
            Err(ValidationError::ConfigValidationFailed(
                "Environment validation failed. See report for details.".to_string()
            ))
        } else {
            Ok(report)
        }
    }

    /// Validate credentials using the credential manager
    pub fn validate_credentials(&self, vault_path: &str) -> ValidationResult<()> {
        match CredentialManager::new(vault_path) {
            Ok(mut manager) => {
                manager.load_from_env()
                    .map_err(|e| ValidationError::CredentialValidationFailed(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(ValidationError::CredentialValidationFailed(e.to_string()))
        }
    }
}

/// Validation report containing results of environment validation
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub successes: Vec<ValidationItem>,
    pub errors: Vec<ValidationItem>,
    pub infos: Vec<ValidationItem>,
}

#[derive(Debug, Clone)]
pub struct ValidationItem {
    pub variable: String,
    pub description: String,
    pub status: String,
    pub value: Option<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            errors: Vec::new(),
            infos: Vec::new(),
        }
    }

    pub fn add_success(&mut self, variable: &str, description: &str, value: Option<&str>) {
        self.successes.push(ValidationItem {
            variable: variable.to_string(),
            description: description.to_string(),
            status: "✓ Valid".to_string(),
            value: value.map(|v| v.to_string()),
        });
    }

    pub fn add_error(&mut self, variable: &str, description: &str, error: &str) {
        self.errors.push(ValidationItem {
            variable: variable.to_string(),
            description: description.to_string(),
            status: format!("✗ Error: {}", error),
            value: None,
        });
    }

    pub fn add_info(&mut self, variable: &str, description: &str, status: &str) {
        self.infos.push(ValidationItem {
            variable: variable.to_string(),
            description: description.to_string(),
            status: status.to_string(),
            value: None,
        });
    }

    pub fn print_report(&self) {
        println!("\n=== Environment Validation Report ===\n");

        if !self.successes.is_empty() {
            println!("✓ SUCCESSFUL VALIDATIONS:");
            for item in &self.successes {
                let value_display = item.value.as_ref()
                    .map(|v| format!(" = {}", v))
                    .unwrap_or_default();
                println!("  {} {}: {}{}", 
                    item.status, item.variable, item.description, value_display);
            }
            println!();
        }

        if !self.errors.is_empty() {
            println!("✗ ERRORS:");
            for item in &self.errors {
                println!("  {} {}: {}", 
                    item.status, item.variable, item.description);
            }
            println!();
        }

        if !self.infos.is_empty() {
            println!("ℹ INFO:");
            for item in &self.infos {
                println!("  {} {}: {}", 
                    item.status, item.variable, item.description);
            }
            println!();
        }

        let total = self.successes.len() + self.errors.len() + self.infos.len();
        println!("Total variables checked: {}", total);
        println!("Successful: {}", self.successes.len());
        println!("Errors: {}", self.errors.len());
        println!("Info: {}", self.infos.len());
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

// Validation functions

fn validate_master_key(value: &str) -> ValidationResult<()> {
    if value.len() != 64 {
        return Err(ValidationError::InvalidFormat(
            "Master key must be 64 hex characters (32 bytes)".to_string()
        ));
    }

    if !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidFormat(
            "Master key must contain only hex characters".to_string()
        ));
    }

    Ok(())
}

fn validate_api_key(value: &str) -> ValidationResult<()> {
    if value.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "API key cannot be empty".to_string()
        ));
    }

    if value.len() < 10 {
        return Err(ValidationError::InvalidFormat(
            "API key appears to be too short".to_string()
        ));
    }

    Ok(())
}

fn validate_email(value: &str) -> ValidationResult<()> {
    if value.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "Email cannot be empty".to_string()
        ));
    }

    if !value.contains('@') {
        return Err(ValidationError::InvalidFormat(
            "Email must contain @ symbol".to_string()
        ));
    }

    Ok(())
}

fn validate_password(value: &str) -> ValidationResult<()> {
    if value.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "Password cannot be empty".to_string()
        ));
    }

    if value.len() < 6 {
        return Err(ValidationError::InvalidFormat(
            "Password must be at least 6 characters long".to_string()
        ));
    }

    Ok(())
}

fn validate_log_level(value: &str) -> ValidationResult<()> {
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&value.to_lowercase().as_str()) {
        return Err(ValidationError::InvalidFormat(
            format!("Log level must be one of: {}", valid_levels.join(", "))
        ));
    }

    Ok(())
}

fn validate_directory_path(value: &str) -> ValidationResult<()> {
    if value.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "Directory path cannot be empty".to_string()
        ));
    }

    // Check if path is valid (basic check)
    if value.contains("..") {
        return Err(ValidationError::InvalidFormat(
            "Directory path cannot contain '..'".to_string()
        ));
    }

    Ok(())
}

fn validate_hostname(value: &str) -> ValidationResult<()> {
    if value.is_empty() {
        return Err(ValidationError::InvalidFormat(
            "Hostname cannot be empty".to_string()
        ));
    }

    // Basic hostname validation
    if value.len() > 253 {
        return Err(ValidationError::InvalidFormat(
            "Hostname too long (max 253 characters)".to_string()
        ));
    }

    Ok(())
}

fn validate_port(value: &str) -> ValidationResult<()> {
    match value.parse::<u16>() {
        Ok(port) => {
            if port == 0 {
                return Err(ValidationError::InvalidFormat(
                    "Port cannot be 0".to_string()
                ));
            }
            Ok(())
        }
        Err(_) => Err(ValidationError::InvalidFormat(
            "Port must be a valid number between 1 and 65535".to_string()
        ))
    }
}

fn validate_proxy_type(value: &str) -> ValidationResult<()> {
    let valid_types = ["http", "https", "socks4", "socks5"];
    if !valid_types.contains(&value.to_lowercase().as_str()) {
        return Err(ValidationError::InvalidFormat(
            format!("Proxy type must be one of: {}", valid_types.join(", "))
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_master_key_validation() {
        assert!(validate_master_key("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef").is_ok());
        assert!(validate_master_key("invalid").is_err());
        assert!(validate_master_key("").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("12345").is_err());
        assert!(validate_password("").is_err());
    }

    #[test]
    fn test_log_level_validation() {
        assert!(validate_log_level("info").is_ok());
        assert!(validate_log_level("DEBUG").is_ok());
        assert!(validate_log_level("invalid").is_err());
    }

    #[test]
    fn test_port_validation() {
        assert!(validate_port("8080").is_ok());
        assert!(validate_port("0").is_err());
        assert!(validate_port("invalid").is_err());
        assert!(validate_port("70000").is_err());
    }

    #[test]
    fn test_proxy_type_validation() {
        assert!(validate_proxy_type("http").is_ok());
        assert!(validate_proxy_type("SOCKS5").is_ok());
        assert!(validate_proxy_type("invalid").is_err());
    }

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        report.add_success("TEST_VAR", "Test variable", Some("test_value"));
        report.add_error("ERROR_VAR", "Error variable", "Test error");
        report.add_info("INFO_VAR", "Info variable", "Not set");

        assert_eq!(report.successes.len(), 1);
        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.infos.len(), 1);
        assert!(report.has_errors());
    }
}
