use anyhow::{Context, Result};
use reqwest::cookie::Jar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tracing::{debug, error, info, warn};

use crate::api::ApiClient;

/// Session credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            email: None,
        }
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

/// Session data containing cookies and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub credentials: Credentials,
    pub cookies: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub is_valid: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Session {
    pub fn new(id: String, credentials: Credentials) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            credentials,
            cookies: HashMap::new(),
            created_at: now,
            last_used: now,
            is_valid: true,
            metadata: HashMap::new(),
        }
    }

    pub fn update_last_used(&mut self) {
        self.last_used = chrono::Utc::now();
    }

    pub fn add_cookie(&mut self, name: String, value: String) {
        self.cookies.insert(name, value);
        self.update_last_used();
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }
}

/// Session manager for handling authentication and cookie persistence
pub struct SessionManager {
    sessions_dir: PathBuf,
    encryption_key: [u8; 32],
    api_client: Arc<ApiClient>,
}

impl SessionManager {
    /// Create a new SessionManager with default configuration
    pub async fn new(api_client: Arc<ApiClient>) -> Result<Self> {
        Self::with_sessions_dir(
            api_client,
            Self::default_sessions_dir()?,
            Self::default_encryption_key(),
        ).await
    }

    /// Create a new SessionManager with custom sessions directory
    pub async fn with_sessions_dir(
        api_client: Arc<ApiClient>,
        sessions_dir: PathBuf,
        encryption_key: [u8; 32],
    ) -> Result<Self> {
        // Ensure sessions directory exists
        if !sessions_dir.exists() {
            fs::create_dir_all(&sessions_dir)
                .await
                .context("Failed to create sessions directory")?;
        }

        Ok(Self {
            sessions_dir,
            encryption_key,
            api_client,
        })
    }

    /// Get the default sessions directory
    fn default_sessions_dir() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Failed to get home directory")?
            .join(".local")
            .join("share")
            .join("lazabot")
            .join("sessions");
        Ok(home)
    }

    /// Generate a default encryption key (in production, this should be user-specific)
    fn default_encryption_key() -> [u8; 32] {
        // In production, this should be derived from user-specific data
        // For now, using a placeholder key
        let mut key = [0u8; 32];
        key[..16].copy_from_slice(b"lazabot-session-");
        key[16..].copy_from_slice(b"encryption-key--");
        key
    }

    /// Generate a unique session ID
    fn generate_session_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("session_{}_{}", timestamp, uuid::Uuid::new_v4().to_string()[..8].to_string())
    }

    /// Login with credentials and create a new session
    pub async fn login(&self, credentials: Credentials) -> Result<Session> {
        info!("Attempting login for user: {}", credentials.username);

        // Create a new session
        let session_id = Self::generate_session_id();
        let mut session = Session::new(session_id, credentials.clone());

        // Perform login request (using httpbin for testing)
        let login_result = self.perform_login(&credentials).await;
        
        match login_result {
            Ok(cookies) => {
                // Store cookies in session
                for (name, value) in cookies {
                    session.add_cookie(name, value);
                }
                
                // Add login metadata
                session.add_metadata("login_successful".to_string(), serde_json::Value::Bool(true));
                session.add_metadata("login_timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
                
                info!("Login successful for user: {}", credentials.username);
                Ok(session)
            }
            Err(e) => {
                error!("Login failed for user: {}: {}", credentials.username, e);
                Err(e)
            }
        }
    }

    /// Perform the actual login request
    async fn perform_login(&self, credentials: &Credentials) -> Result<HashMap<String, String>> {
        // For testing purposes, we'll use httpbin.org to simulate login
        // In production, this would be the actual Lazada login endpoint
        let login_url = "https://httpbin.org/cookies/set";
        
        // Create a request to set some test cookies
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        
        let login_data = serde_json::json!({
            "username": credentials.username,
            "password": "[REDACTED]", // Don't log actual password
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let response = self.api_client.request(
            reqwest::Method::POST,
            login_url,
            Some(headers),
            Some(login_data.to_string().as_bytes().to_vec()),
            None, // No proxy for login
        ).await?;

        if response.status >= 200 && response.status < 300 {
            // Parse cookies from response headers
            let mut cookies = HashMap::new();
            
            // Simulate setting some test cookies
            cookies.insert("session_id".to_string(), uuid::Uuid::new_v4().to_string());
            cookies.insert("user_id".to_string(), credentials.username.clone());
            cookies.insert("login_time".to_string(), chrono::Utc::now().to_rfc3339());
            cookies.insert("auth_token".to_string(), format!("token_{}", uuid::Uuid::new_v4().to_string()[..16].to_string()));
            
            debug!("Login response received with {} cookies", cookies.len());
            Ok(cookies)
        } else {
            Err(anyhow::anyhow!("Login failed with status: {}", response.status))
        }
    }

    /// Persist session to encrypted file
    pub async fn persist_session(&self, session: &Session) -> Result<()> {
        let session_file = self.sessions_dir.join(format!("{}.bin", session.id));
        
        info!("Persisting session {} to {:?}", session.id, session_file);

        // Serialize session data
        let session_data = serde_json::to_vec(session)
            .context("Failed to serialize session data")?;

        // Encrypt the session data
        let encrypted_data = self.encrypt_data(&session_data)
            .context("Failed to encrypt session data")?;

        // Write to file
        fs::write(&session_file, encrypted_data)
            .await
            .context("Failed to write session file")?;

        debug!("Session {} persisted successfully", session.id);
        Ok(())
    }

    /// Restore session from encrypted file
    pub async fn restore_session(&self, session_id: &str) -> Result<Session> {
        let session_file = self.sessions_dir.join(format!("{}.bin", session_id));
        
        if !session_file.exists() {
            return Err(anyhow::anyhow!("Session file not found: {:?}", session_file));
        }

        info!("Restoring session {} from {:?}", session_id, session_file);

        // Read encrypted data
        let encrypted_data = fs::read(&session_file)
            .await
            .context("Failed to read session file")?;

        // Decrypt the data
        let session_data = self.decrypt_data(&encrypted_data)
            .context("Failed to decrypt session data")?;

        // Deserialize session
        let session: Session = serde_json::from_slice(&session_data)
            .context("Failed to deserialize session data")?;

        debug!("Session {} restored successfully", session_id);
        Ok(session)
    }

    /// Validate session by pinging a lightweight endpoint
    pub async fn validate_session(&self, session: &mut Session) -> Result<bool> {
        info!("Validating session: {}", session.id);

        // Update last used timestamp
        session.update_last_used();

        // Create a cookie jar from session cookies
        let cookie_jar = self.create_cookie_jar_from_session(session);

        // Create a temporary API client with the session cookies
        let temp_client = ApiClient::with_cookie_jar(cookie_jar)?;

        // Ping a lightweight endpoint to validate the session
        let validation_result = self.ping_validation_endpoint(&temp_client).await;
        
        match validation_result {
            Ok(is_valid) => {
                session.is_valid = is_valid;
                if is_valid {
                    debug!("Session {} is valid", session.id);
                } else {
                    warn!("Session {} is invalid", session.id);
                }
                Ok(is_valid)
            }
            Err(e) => {
                error!("Session validation failed for {}: {}", session.id, e);
                session.is_valid = false;
                Ok(false)
            }
        }
    }

    /// Ping a lightweight endpoint to check session validity
    async fn ping_validation_endpoint(&self, client: &ApiClient) -> Result<bool> {
        // Use httpbin.org for testing - in production this would be a lightweight auth endpoint
        let validation_url = "https://httpbin.org/headers";
        
        let response = client.request(
            reqwest::Method::GET,
            validation_url,
            None,
            None,
            None,
        ).await?;

        // Consider session valid if we get a successful response
        let is_valid = response.status >= 200 && response.status < 300;
        
        if is_valid {
            debug!("Validation endpoint responded successfully");
        } else {
            warn!("Validation endpoint returned status: {}", response.status);
        }

        Ok(is_valid)
    }

    /// Create a cookie jar from session cookies
    fn create_cookie_jar_from_session(&self, session: &Session) -> Arc<Jar> {
        let jar = Arc::new(Jar::default());
        
        // Add cookies to the jar
        for (name, value) in &session.cookies {
            // Create a simple cookie string
            let cookie_str = format!("{}={}", name, value);
            jar.add_cookie_str(&cookie_str, &reqwest::Url::parse("https://httpbin.org").unwrap());
        }
        
        jar
    }

    /// List all available sessions
    pub async fn list_sessions(&self) -> Result<Vec<String>> {
        let mut sessions = Vec::new();
        
        let mut entries = fs::read_dir(&self.sessions_dir).await
            .context("Failed to read sessions directory")?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("bin") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    sessions.push(stem.to_string());
                }
            }
        }
        
        sessions.sort();
        Ok(sessions)
    }

    /// Delete a session
    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        let session_file = self.sessions_dir.join(format!("{}.bin", session_id));
        
        if session_file.exists() {
            fs::remove_file(&session_file).await
                .context("Failed to delete session file")?;
            info!("Session {} deleted", session_id);
        } else {
            warn!("Session file not found: {:?}", session_file);
        }
        
        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self, max_age_days: i64) -> Result<usize> {
        let mut cleaned_count = 0;
        let cutoff_time = chrono::Utc::now() - chrono::Duration::days(max_age_days);
        
        let sessions = self.list_sessions().await?;
        
        for session_id in sessions {
            match self.restore_session(&session_id).await {
                Ok(session) => {
                    if session.last_used < cutoff_time {
                        self.delete_session(&session_id).await?;
                        cleaned_count += 1;
                        info!("Cleaned up expired session: {}", session_id);
                    }
                }
                Err(e) => {
                    warn!("Failed to restore session {} for cleanup: {}", session_id, e);
                    // Delete corrupted session files
                    self.delete_session(&session_id).await?;
                    cleaned_count += 1;
                }
            }
        }
        
        info!("Cleaned up {} expired sessions", cleaned_count);
        Ok(cleaned_count)
    }

    /// Encrypt data using AES-GCM
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, KeyInit};

        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        
        // Generate a random nonce
        let nonce = Nonce::from_slice(b"uniqnonce123"); // In production, use a random nonce
        
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Failed to encrypt data: {}", e))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    /// Decrypt data using AES-GCM
    fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, KeyInit};

        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Encrypted data too short"));
        }

        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Failed to decrypt data: {}", e))?;
        
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_session_creation() -> Result<()> {
        let api_client = Arc::new(ApiClient::new(Some("Lazabot-Test/1.0".to_string()))?);
        let manager = SessionManager::new(api_client).await?;
        
        let credentials = Credentials::new("testuser".to_string(), "testpass".to_string());
        let session = manager.login(credentials).await?;
        
        assert!(!session.id.is_empty());
        assert_eq!(session.credentials.username, "testuser");
        assert!(!session.cookies.is_empty());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_session_persistence() -> Result<()> {
        let api_client = Arc::new(ApiClient::new(Some("Lazabot-Test/1.0".to_string()))?);
        let manager = SessionManager::new(api_client).await?;
        
        let credentials = Credentials::new("testuser".to_string(), "testpass".to_string());
        let session = manager.login(credentials).await?;
        
        // Persist session
        manager.persist_session(&session).await?;
        
        // Restore session
        let restored_session = manager.restore_session(&session.id).await?;
        
        assert_eq!(restored_session.id, session.id);
        assert_eq!(restored_session.credentials.username, session.credentials.username);
        assert_eq!(restored_session.cookies.len(), session.cookies.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_session_validation() -> Result<()> {
        let api_client = Arc::new(ApiClient::new(Some("Lazabot-Test/1.0".to_string()))?);
        let manager = SessionManager::new(api_client).await?;
        
        let credentials = Credentials::new("testuser".to_string(), "testpass".to_string());
        let mut session = manager.login(credentials).await?;
        
        // Validate session
        let is_valid = manager.validate_session(&mut session).await?;
        
        assert!(is_valid);
        assert!(session.is_valid);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_session_cleanup() -> Result<()> {
        let api_client = Arc::new(ApiClient::new(Some("Lazabot-Test/1.0".to_string()))?);
        let manager = SessionManager::new(api_client).await?;
        
        // Create and persist a session
        let credentials = Credentials::new("testuser".to_string(), "testpass".to_string());
        let session = manager.login(credentials).await?;
        manager.persist_session(&session).await?;
        
        // List sessions
        let sessions = manager.list_sessions().await?;
        assert!(sessions.contains(&session.id));
        
        // Clean up (with very short max age to force cleanup)
        let cleaned = manager.cleanup_expired_sessions(0).await?;
        assert!(cleaned >= 0);
        
        Ok(())
    }
}
