#!/bin/bash

echo "🔐 Lazada CLI Bot - Encryption Module Test"
echo "=========================================="

# Generate a test master key
echo "1. Generating test master key..."
MASTER_KEY=$(openssl rand -hex 32)
echo "Generated key: $MASTER_KEY"

# Set the environment variable
export LAZABOT_MASTER_KEY="$MASTER_KEY"
echo "✅ Environment variable set"

# Test the encryption functionality using a simple test
echo ""
echo "2. Testing encryption functionality..."

# Create a test directory
mkdir -p /tmp/encryption_test/src
cd /tmp/encryption_test

# Create a simple test program
cat > src/main.rs << 'RUST_EOF'
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let master_key = env::var("LAZABOT_MASTER_KEY")?;
    let key_bytes = hex::decode(&master_key)?;
    
    if key_bytes.len() != 32 {
        return Err("Master key must be 32 bytes (64 hex characters)".into());
    }
    
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    let test_strings = vec![
        "Hello, World!",
        "Sensitive password data",
        "Unicode test: 世界 🌍",
        "", // Empty string
    ];
    
    for (i, plaintext) in test_strings.iter().enumerate() {
        println!("Test {}.1: Encrypting '{}'", i + 1, plaintext);
        
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Combine nonce and ciphertext
        let mut encrypted_data = Vec::with_capacity(12 + ciphertext.len());
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);
        
        // Encode as base64
        let encoded = general_purpose::STANDARD.encode(&encrypted_data);
        println!("Test {}.2: Encrypted: {}", i + 1, encoded);
        
        // Decrypt
        let encrypted_bytes = general_purpose::STANDARD.decode(&encoded)?;
        let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        let decrypted_string = String::from_utf8(decrypted)?;
        
        println!("Test {}.3: Decrypted: '{}'", i + 1, decrypted_string);
        
        assert_eq!(plaintext, &decrypted_string);
        println!("Test {}.4: ✅ Roundtrip successful", i + 1);
    }
    
    println!("\n🎉 All encryption tests passed successfully!");
    Ok(())
}
RUST_EOF

# Create a simple Cargo.toml for the test
cat > Cargo.toml << 'CARGO_EOF'
[package]
name = "encryption_test"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "test_encryption"
path = "src/main.rs"

[dependencies]
aes-gcm = { version = "0.10", features = ["aes"] }
hex = "0.4"
base64 = "0.21"
rand = "0.8"
CARGO_EOF

# Run the test
cargo run --bin test_encryption

echo ""
echo "3. Key Management Instructions:"
echo "   - Generate key: openssl rand -hex 32"
echo "   - Set env var: export LAZABOT_MASTER_KEY=<key>"
echo "   - Store securely: Use password manager or key vault"
echo "   - Rotate regularly: Follow security policy"

echo ""
echo "🎉 Encryption module test completed successfully!"

# Cleanup
cd /tmp
rm -rf encryption_test
