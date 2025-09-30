use lazabot::config::encryption::{EncryptionManager, init_encryption, encrypt, decrypt};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a test key
    let test_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    env::set_var("LAZABOT_MASTER_KEY", test_key);
    
    println!("🔐 Testing Lazada CLI Bot Encryption Module");
    println!("==========================================");
    
    // Test 1: Direct manager usage
    println!("\n1. Testing EncryptionManager directly:");
    let manager = EncryptionManager::new()?;
    
    let test_strings = vec![
        "Hello, World!",
        "Sensitive password data",
        "Unicode test: 世界 🌍",
        "", // Empty string
    ];
    
    for (i, plaintext) in test_strings.iter().enumerate() {
        println!("  Test {}.1: Encrypting '{}'", i + 1, plaintext);
        let encrypted = manager.encrypt(plaintext)?;
        println!("  Test {}.2: Encrypted: {}", i + 1, encrypted);
        
        let decrypted = manager.decrypt(&encrypted)?;
        println!("  Test {}.3: Decrypted: '{}'", i + 1, decrypted);
        
        assert_eq!(plaintext, &decrypted);
        println!("  Test {}.4: ✅ Roundtrip successful", i + 1);
    }
    
    // Test 2: Global functions
    println!("\n2. Testing global encryption functions:");
    init_encryption()?;
    
    let global_test = "Global encryption test";
    let encrypted_global = encrypt(global_test)?;
    let decrypted_global = decrypt(&encrypted_global)?;
    
    println!("  Original: {}", global_test);
    println!("  Encrypted: {}", encrypted_global);
    println!("  Decrypted: {}", decrypted_global);
    assert_eq!(global_test, decrypted_global);
    println!("  ✅ Global functions work correctly");
    
    // Test 3: Field encryption
    println!("\n3. Testing field encryption:");
    let sensitive_data = vec![
        ("username", "user@example.com"),
        ("password", "super_secret_password"),
        ("api_key", "sk-1234567890abcdef"),
        ("empty_field", ""),
    ];
    
    for (field_name, field_value) in sensitive_data {
        let encrypted_field = manager.encrypt_field(field_value)?;
        let decrypted_field = manager.decrypt_field(&encrypted_field)?;
        
        println!("  {}: '{}' -> encrypted -> '{}'", field_name, field_value, decrypted_field);
        assert_eq!(field_value, decrypted_field);
    }
    println!("  ✅ Field encryption works correctly");
    
    // Test 4: Error handling
    println!("\n4. Testing error handling:");
    
    // Test missing key
    env::remove_var("LAZABOT_MASTER_KEY");
    let result = EncryptionManager::new();
    match result {
        Err(e) => println!("  ✅ Correctly caught missing key error: {}", e),
        Ok(_) => println!("  ❌ Should have failed with missing key"),
    }
    
    // Restore key for remaining tests
    env::set_var("LAZABOT_MASTER_KEY", test_key);
    
    // Test invalid key format
    env::set_var("LAZABOT_MASTER_KEY", "invalid_key");
    let result = EncryptionManager::new();
    match result {
        Err(e) => println!("  ✅ Correctly caught invalid key error: {}", e),
        Ok(_) => println!("  ❌ Should have failed with invalid key"),
    }
    
    // Restore valid key
    env::set_var("LAZABOT_MASTER_KEY", test_key);
    
    println!("\n🎉 All encryption tests passed successfully!");
    println!("\n📋 Key Management Instructions:");
    println!("1. Generate a new master key: openssl rand -hex 32");
    println!("2. Set environment variable: export LAZABOT_MASTER_KEY=<generated_key>");
    println!("3. Store the key securely (password manager, key vault, etc.)");
    println!("4. For production, use a secure key management system");
    println!("5. Rotate keys regularly following your security policy");
    
    Ok(())
}
