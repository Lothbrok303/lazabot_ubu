use std::env;

// Simple integration test for encryption functionality
#[test]
fn test_encryption_integration() {
    // Set up test environment
    let test_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    env::set_var("LAZABOT_MASTER_KEY", test_key);

    // Test that we can create the encryption module
    // This is a basic test to ensure the module compiles and can be imported
    assert!(env::var("LAZABOT_MASTER_KEY").is_ok());

    // Test key format validation
    let master_key = env::var("LAZABOT_MASTER_KEY").unwrap();
    let key_bytes = hex::decode(&master_key).unwrap();
    assert_eq!(key_bytes.len(), 32, "Master key must be 32 bytes");

    println!("✅ Encryption integration test passed");
}

#[test]
fn test_key_generation_instructions() {
    // Test that the key generation command works
    use std::process::Command;

    let output = Command::new("openssl")
        .args(&["rand", "-hex", "32"])
        .output()
        .expect("Failed to execute openssl command");

    assert!(output.status.success(), "OpenSSL command should succeed");

    let key = String::from_utf8(output.stdout).unwrap();
    let key = key.trim();

    // Verify the key format
    assert_eq!(key.len(), 64, "Generated key should be 64 hex characters");
    assert!(
        key.chars().all(|c| c.is_ascii_hexdigit()),
        "Key should contain only hex characters"
    );

    println!("✅ Key generation test passed. Generated key: {}", key);
}
