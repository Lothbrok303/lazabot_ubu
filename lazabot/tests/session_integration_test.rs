//! Integration test for session management
//!
//! This test demonstrates:
//! 1. Creating a session with mock cookies
//! 2. Persisting session to disk
//! 3. Restoring session from disk
//! 4. Verifying cookie persistence

use anyhow::Result;
use lazabot::core::session::{Credentials, Session};


#[tokio::test]
async fn test_session_manual_persistence() -> Result<()> {
    // Create temporary session storage
    let temp_dir = std::env::temp_dir().join(format!("lazabot_test_{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir_all(&temp_dir).await?;

    // 1. CREATE: Manually create a session
    let credentials = Credentials::new("test_user".to_string(), "test_password".to_string());

    let mut session = Session::new("test_session_123".to_string(), credentials);

    // Add some test cookies
    session.add_cookie("session_id".to_string(), "abc123xyz789".to_string());
    session.add_cookie("auth_token".to_string(), "bearer_token_456".to_string());
    session.add_cookie("user_id".to_string(), "user_789".to_string());

    // Add metadata
    session.add_metadata(
        "test_key".to_string(),
        serde_json::Value::String("test_value".to_string()),
    );

    println!("✓ Created session with {} cookies", session.cookies.len());

    // 2. PERSIST: Save to file (unencrypted for testing)
    let session_file = temp_dir.join(format!("{}.json", session.id));
    let session_json = serde_json::to_string_pretty(&session)?;
    tokio::fs::write(&session_file, session_json).await?;

    println!("✓ Persisted session to {:?}", session_file);

    // 3. RESTORE: Load from file
    let restored_json = tokio::fs::read_to_string(&session_file).await?;
    let restored_session: Session = serde_json::from_str(&restored_json)?;

    println!(
        "✓ Restored session with {} cookies",
        restored_session.cookies.len()
    );

    // 4. VERIFY: Check integrity
    assert_eq!(session.id, restored_session.id);
    assert_eq!(
        session.credentials.username,
        restored_session.credentials.username
    );
    assert_eq!(session.cookies.len(), restored_session.cookies.len());
    assert_eq!(
        session.cookies.get("session_id"),
        restored_session.cookies.get("session_id")
    );
    assert_eq!(
        session.cookies.get("auth_token"),
        restored_session.cookies.get("auth_token")
    );
    assert_eq!(
        session.cookies.get("user_id"),
        restored_session.cookies.get("user_id")
    );

    println!("✓ Cookies integrity verified!");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await?;
    println!("✓ Test complete and cleaned up");

    Ok(())
}
