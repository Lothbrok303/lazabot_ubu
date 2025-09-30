use rand::RngCore;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use std::env;
use thiserror::Error;

/// Encryption errors
#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Failed to get master key from environment: {0}")]
    MissingMasterKey(String),
    #[error("Invalid master key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Base64 encoding/decoding failed: {0}")]
    Base64Error(String),
}

/// Result type for encryption operations
pub type EncryptionResult<T> = Result<T, EncryptionError>;

/// AES-GCM encryption manager
pub struct EncryptionManager {
    cipher: Aes256Gcm,
}

impl EncryptionManager {
    /// Create a new encryption manager using the master key from environment
    pub fn new() -> EncryptionResult<Self> {
        let master_key = env::var("LAZABOT_MASTER_KEY").map_err(|_| {
            EncryptionError::MissingMasterKey(
                "LAZABOT_MASTER_KEY environment variable not set".to_string(),
            )
        })?;

        // Decode the hex-encoded master key
        let key_bytes = hex::decode(&master_key)
            .map_err(|e| EncryptionError::InvalidKeyFormat(format!("Invalid hex format: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(EncryptionError::InvalidKeyFormat(
                "Master key must be 32 bytes (64 hex characters)".to_string(),
            ));
        }

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        Ok(Self { cipher })
    }

    /// Encrypt a plaintext string
    pub fn encrypt(&self, plaintext: &str) -> EncryptionResult<String> {
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the plaintext
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

        // Combine nonce and ciphertext
        let mut encrypted_data = Vec::with_capacity(12 + ciphertext.len());
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);

        // Encode as base64
        let encoded = general_purpose::STANDARD.encode(&encrypted_data);
        Ok(encoded)
    }

    /// Decrypt a base64-encoded encrypted string
    pub fn decrypt(&self, encrypted_data: &str) -> EncryptionResult<String> {
        // Decode from base64
        let encrypted_bytes = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|e| EncryptionError::Base64Error(e.to_string()))?;

        if encrypted_bytes.len() < 12 {
            return Err(EncryptionError::DecryptionFailed(
                "Invalid encrypted data: too short".to_string(),
            ));
        }

        // Split nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

        // Convert to string
        String::from_utf8(plaintext)
            .map_err(|e| EncryptionError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
    }

    /// Encrypt a sensitive field and return the encrypted value
    pub fn encrypt_field(&self, field: &str) -> EncryptionResult<String> {
        if field.is_empty() {
            return Ok(String::new());
        }
        self.encrypt(field)
    }

    /// Decrypt a sensitive field and return the plaintext value
    pub fn decrypt_field(&self, encrypted_field: &str) -> EncryptionResult<String> {
        if encrypted_field.is_empty() {
            return Ok(String::new());
        }
        self.decrypt(encrypted_field)
    }
}

/// Convenience functions for global encryption operations
/// These use a lazy static to avoid recreating the cipher repeatedly
use std::sync::OnceLock;

static ENCRYPTION_MANAGER: OnceLock<EncryptionManager> = OnceLock::new();

/// Initialize the global encryption manager
pub fn init_encryption() -> EncryptionResult<()> {
    let manager = EncryptionManager::new()?;
    ENCRYPTION_MANAGER.set(manager).map_err(|_| {
        EncryptionError::EncryptionFailed("Failed to initialize encryption manager".to_string())
    })?;
    Ok(())
}

/// Get the global encryption manager
fn get_manager() -> EncryptionResult<&'static EncryptionManager> {
    ENCRYPTION_MANAGER.get().ok_or_else(|| {
        EncryptionError::EncryptionFailed("Encryption manager not initialized".to_string())
    })
}

/// Encrypt a string using the global encryption manager
pub fn encrypt(plaintext: &str) -> EncryptionResult<String> {
    get_manager()?.encrypt(plaintext)
}

/// Decrypt a string using the global encryption manager
pub fn decrypt(encrypted_data: &str) -> EncryptionResult<String> {
    get_manager()?.decrypt(encrypted_data)
}

/// Encrypt a sensitive field using the global encryption manager
pub fn encrypt_field(field: &str) -> EncryptionResult<String> {
    get_manager()?.encrypt_field(field)
}

/// Decrypt a sensitive field using the global encryption manager
pub fn decrypt_field(encrypted_field: &str) -> EncryptionResult<String> {
    get_manager()?.decrypt_field(encrypted_field)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_test_env() {
        // Use a test key for unit tests
        env::set_var(
            "LAZABOT_MASTER_KEY",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
    }

    #[test]
    fn test_encryption_manager_creation() {
        setup_test_env();
        let manager = EncryptionManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        setup_test_env();
        let manager = EncryptionManager::new().unwrap();

        let plaintext = "Hello, World! This is a test message.";
        let encrypted = manager.encrypt(plaintext).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_empty_string() {
        setup_test_env();
        let manager = EncryptionManager::new().unwrap();

        let plaintext = "";
        let encrypted = manager.encrypt(plaintext).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_unicode() {
        setup_test_env();
        let manager = EncryptionManager::new().unwrap();

        let plaintext = "Hello 世界! 🌍 This is a unicode test.";
        let encrypted = manager.encrypt(plaintext).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_field_empty() {
        setup_test_env();
        let manager = EncryptionManager::new().unwrap();

        let result = manager.encrypt_field("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_decrypt_field_empty() {
        setup_test_env();
        let manager = EncryptionManager::new().unwrap();

        let result = manager.decrypt_field("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_global_encryption_functions() {
        setup_test_env();
        init_encryption().unwrap();

        let plaintext = "Test global encryption";
        let encrypted = encrypt(plaintext).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_missing_master_key() {
        env::remove_var("LAZABOT_MASTER_KEY");
        let manager = EncryptionManager::new();
        assert!(matches!(manager, Err(EncryptionError::MissingMasterKey(_))));
    }

    #[test]
    fn test_invalid_key_format() {
        env::set_var("LAZABOT_MASTER_KEY", "invalid_key");
        let manager = EncryptionManager::new();
        assert!(matches!(manager, Err(EncryptionError::InvalidKeyFormat(_))));
    }

    #[test]
    fn test_invalid_key_length() {
        env::set_var("LAZABOT_MASTER_KEY", "0123456789abcdef"); // Too short
        let manager = EncryptionManager::new();
        assert!(matches!(manager, Err(EncryptionError::InvalidKeyFormat(_))));
    }
}
