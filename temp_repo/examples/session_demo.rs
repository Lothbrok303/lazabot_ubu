//! Session Management Demonstration
//! 
//! This example demonstrates the complete login/save/restore flow
//! with working cookie persistence.
//! 
//! Run with: cargo run --example session_demo

use anyhow::Result;
use lazabot::core::session::{Session, Credentials};

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n=== Lazabot Session Management Demo ===\n");
    
    // Create a temporary directory for this demo
    let demo_dir = std::env::temp_dir().join("lazabot_session_demo");
    tokio::fs::create_dir_all(&demo_dir).await?;
    println!("[*] Session storage: {:?}\n", demo_dir);
    
    // Step 1: CREATE SESSION (simulating login)
    println!("STEP 1: Login / Create Session");
    println!("----------------------------------");
    
    let credentials = Credentials::new(
        "demo_user@example.com".to_string(),
        "secure_password_123".to_string(),
    );
    
    let mut session = Session::new("demo_session_001".to_string(), credentials);
    
    // Simulate cookies received from a login endpoint
    session.add_cookie("JSESSIONID".to_string(), "A1B2C3D4E5F6G7H8I9J0".to_string());
    session.add_cookie("auth_token".to_string(), "bearer_xyz789abc456def123".to_string());
    session.add_cookie("user_pref".to_string(), "lang=en&currency=USD".to_string());
    session.add_cookie("cart_id".to_string(), "cart_12345_abc".to_string());
    
    // Add session metadata
    session.add_metadata("login_method".to_string(), serde_json::Value::String("email".to_string()));
    session.add_metadata("ip_address".to_string(), serde_json::Value::String("192.168.1.100".to_string()));
    
    println!("[+] Session created: {}", session.id);
    println!("[+] Username: {}", session.credentials.username);
    println!("[+] Cookies received: {}", session.cookies.len());
    println!("[+] Session created at: {}", session.created_at);
    
    println!("\nCookies:");
    for (name, value) in &session.cookies {
        let display_value = if value.len() > 40 {
            format!("{}...", &value[..40])
        } else {
            value.clone()
        };
        println!("  [COOKIE] {}: {}", name, display_value);
    }
    
    // Step 2: PERSIST SESSION (save to disk)
    println!("\n\nSTEP 2: Persist Session to Disk");
    println!("----------------------------------");
    
    let session_file = demo_dir.join(format!("{}.json", session.id));
    let session_json = serde_json::to_string_pretty(&session)?;
    tokio::fs::write(&session_file, &session_json).await?;
    
    println!("[+] Session persisted to file");
    println!("[+] File path: {:?}", session_file);
    println!("[+] File size: {} bytes", session_json.len());
    
    // Show a snippet of the saved data
    let snippet_lines: Vec<&str> = session_json.lines().take(10).collect();
    println!("\nSaved session data (first 10 lines):");
    for line in snippet_lines {
        println!("  {}", line);
    }
    println!("  ...");
    
    // Step 3: RESTORE SESSION (load from disk)
    println!("\n\nSTEP 3: Restore Session from Disk");
    println!("----------------------------------");
    
    println!("Reading session file...");
    let restored_json = tokio::fs::read_to_string(&session_file).await?;
    let restored_session: Session = serde_json::from_str(&restored_json)?;
    
    println!("[+] Session restored successfully!");
    println!("[+] Session ID: {}", restored_session.id);
    println!("[+] Username: {}", restored_session.credentials.username);
    println!("[+] Cookies restored: {}", restored_session.cookies.len());
    println!("[+] Last used: {}", restored_session.last_used);
    
    println!("\nRestored cookies:");
    for (name, value) in &restored_session.cookies {
        let display_value = if value.len() > 40 {
            format!("{}...", &value[..40])
        } else {
            value.clone()
        };
        println!("  [COOKIE] {}: {}", name, display_value);
    }
    
    // Step 4: VERIFY INTEGRITY
    println!("\n\nSTEP 4: Verify Cookie Persistence");
    println!("----------------------------------");
    
    let mut all_cookies_match = true;
    for (name, value) in &session.cookies {
        if let Some(restored_value) = restored_session.cookies.get(name) {
            if value == restored_value {
                println!("  [OK] Cookie '{}' matches", name);
            } else {
                println!("  [FAIL] Cookie '{}' MISMATCH!", name);
                all_cookies_match = false;
            }
        } else {
            println!("  [FAIL] Cookie '{}' NOT FOUND!", name);
            all_cookies_match = false;
        }
    }
    
    if all_cookies_match {
        println!("\n[SUCCESS] All cookies persisted and restored correctly!");
    } else {
        println!("\n[ERROR] Some cookies were not persisted correctly!");
    }
    
    // Step 5: SUMMARY
    println!("\n\n=== Demo Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("  [+] Session creation with credentials");
    println!("  [+] Cookie storage (simulating login response)");
    println!("  [+] Session persistence to disk (JSON format)");
    println!("  [+] Session restoration from disk");
    println!("  [+] Cookie integrity verification");
    println!("  [+] Metadata storage");
    
    println!("\nProduction Features Available:");
    println!("  * AES-256-GCM encryption for session files");
    println!("  * Automatic session validation");
    println!("  * Session expiration and cleanup");
    println!("  * Multiple session management");
    println!("  * Cookie jar integration with HTTP client");
    
    // Cleanup
    println!("\n[*] Cleaning up demo files...");
    tokio::fs::remove_dir_all(&demo_dir).await?;
    println!("[+] Cleanup complete");
    
    println!("\n=== Session Management Demo Finished! ===\n");
    
    Ok(())
}
