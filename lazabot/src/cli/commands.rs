use anyhow::Result;
use crate::cli::args::Commands;
use crate::config::loader::load_config;
use crate::config::validation::EnvValidator;
use crate::config::credentials::CredentialManager;
use crate::proxy::ProxyManager;

/// Handle monitor command
pub async fn handle_monitor(
    products: Option<String>,
    interval: u64,
    verbose: bool,
) -> Result<()> {
    println!("Monitor command executed");
    println!("Products file: {:?}", products);
    println!("Interval: {} seconds", interval);
    println!("Verbose: {}", verbose);
    Ok(())
}

/// Handle buy command
pub async fn handle_buy(
    product: Option<String>,
    quantity: u32,
    dry_run: bool,
) -> Result<()> {
    println!("Buy command executed");
    println!("Product: {:?}", product);
    println!("Quantity: {}", quantity);
    println!("Dry run: {}", dry_run);
    Ok(())
}

/// Handle proxy command
pub async fn handle_proxy(
    test: bool,
    add: Option<String>,
    list: bool,
    proxies: Option<String>,
) -> Result<()> {
    if test {
        let proxy_file = proxies.unwrap_or_else(|| "config/proxies.txt".to_string());
        println!("Testing proxies from: {}", proxy_file);

        let manager = ProxyManager::from_file(&proxy_file).await?;
        let healthy_proxies = manager.get_healthy_proxies().await;

        println!("Found {} healthy proxies", healthy_proxies.len());
        for (i, proxy) in healthy_proxies.iter().enumerate() {
            println!("  {}: {}:{}", i + 1, proxy.host, proxy.port);
        }
    } else if list {
        let proxy_file = proxies.unwrap_or_else(|| "config/proxies.txt".to_string());
        println!("Listing proxies from: {}", proxy_file);

        let manager = ProxyManager::from_file(&proxy_file).await?;
        let all_proxies = manager.get_all_proxies();
        let healthy_proxies = manager.get_healthy_proxies().await;

        println!("\nAll proxies ({}):", all_proxies.len());
        for (i, proxy) in all_proxies.iter().enumerate() {
            let is_healthy = manager.is_proxy_healthy(proxy).await;
            let status = if is_healthy { "✓" } else { "✗" };
            println!("  {} {}: {}:{}", status, i + 1, proxy.host, proxy.port);
        }

        println!("\nHealthy proxies ({}):", healthy_proxies.len());
        for (i, proxy) in healthy_proxies.iter().enumerate() {
            println!("  {}: {}:{}", i + 1, proxy.host, proxy.port);
        }
    } else if let Some(proxy_str) = add {
        println!("Adding proxy: {}", proxy_str);
        // TODO: Implement adding proxy to file
        println!("Proxy addition not yet implemented");
    } else {
        println!("Proxy command executed");
        println!("Use --test to test proxies, --list to list them, or --add to add new ones");
        println!("Use --proxies to specify a custom proxy file path");
    }

    Ok(())
}

/// Handle session command
pub async fn handle_session(login: bool, logout: bool, status: bool) -> Result<()> {
    println!("Session command executed");
    println!("Login: {}", login);
    println!("Logout: {}", logout);
    println!("Status: {}", status);
    Ok(())
}

/// Handle config command
pub async fn handle_config(
    file: Option<String>,
    show: bool,
    set: Option<String>,
    reset: bool,
) -> Result<()> {
    if reset {
        println!("Resetting to default configuration...");
        let default_config = crate::config::create_default_config();
        println!("Default configuration created successfully");
        println!("Bot name: {}", default_config.bot.name);
        println!("Default delay: {}ms", default_config.bot.default_delay);
        println!("Max retries: {}", default_config.bot.max_retries);
        return Ok(());
    }

    if let Some(file_path) = file {
        println!("Loading configuration from: {}", file_path);
        match load_config(&file_path) {
            Ok(config) => {
                println!("Configuration loaded successfully!");
                println!("Bot name: {}", config.bot.name);
                println!("Default delay: {}ms", config.bot.default_delay);
                println!("Max retries: {}", config.bot.max_retries);
                println!("Debug mode: {}", config.bot.debug);
                println!("Number of accounts: {}", config.accounts.len());
                println!("Number of proxies: {}", config.proxies.len());
                println!("Captcha service: {}", config.captcha.service);
                println!(
                    "Stealth mode: random_delays={}, proxy_rotation={}",
                    config.stealth.random_delays, config.stealth.proxy_rotation
                );
                println!(
                    "Monitoring: enabled={}, log_level={}",
                    config.monitoring.enable_logging, config.monitoring.log_level
                );

                if show {
                    println!("\n=== Full Configuration ===");
                    println!("{:#?}", config);
                }
            }
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                return Err(e);
            }
        }
    } else if show {
        println!("No configuration file specified. Use --file to specify a config file.");
        println!("Example: cargo run -- config --file config/config.toml.example");
    } else {
        println!("Config command executed");
        println!("Use --file to specify a configuration file");
        println!("Use --show to display the loaded configuration");
        println!("Use --reset to reset to default configuration");
    }

    if let Some(set_value) = set {
        println!("Set configuration value: {}", set_value);
        // TODO: Implement configuration value setting
    }

    Ok(())
}

/// Handle validate command
pub async fn handle_validate(
    verbose: bool,
    credentials: bool,
    vault_path: String,
    strict: bool,
) -> Result<()> {
    println!("🔍 Validating environment and configuration...\n");

    if credentials {
        // Validate credentials only
        println!("Validating credentials...");
        match CredentialManager::new(&vault_path) {
            Ok(mut manager) => {
                manager.load_from_env().map_err(|e| anyhow::anyhow!("Credential validation failed: {}", e))?;
                println!("✅ Credentials validation successful");
                
                if verbose {
                    let vault_info = manager.get_vault_info();
                    println!("\n📊 Credential Vault Info:");
                    println!("  Accounts: {}", vault_info.accounts.len());
                    println!("  Proxies: {}", vault_info.proxies.len());
                    println!("  Captcha configured: {}", vault_info.captcha.is_some());
                    println!("  Created: {}", vault_info.created_at);
                    println!("  Last updated: {}", vault_info.last_updated);
                }
            }
            Err(e) => {
                eprintln!("❌ Credentials validation failed: {}", e);
                if strict {
                    std::process::exit(1);
                }
                return Err(anyhow::anyhow!("Credential validation failed: {}", e));
            }
        }
    } else {
        // Full validation
        let validator = EnvValidator::new();
        
        match validator.validate_all() {
            Ok(report) => {
                println!("✅ Environment validation successful");
                
                if verbose {
                    report.print_report();
                } else {
                    println!("  Total variables checked: {}", 
                        report.successes.len() + report.errors.len() + report.infos.len());
                    println!("  Successful: {}", report.successes.len());
                    println!("  Errors: {}", report.errors.len());
                    println!("  Info: {}", report.infos.len());
                }

                // Also validate credentials
                println!("\n🔐 Validating credentials...");
                match CredentialManager::new(&vault_path) {
                    Ok(mut manager) => {
                        manager.load_from_env().map_err(|e| anyhow::anyhow!("Credential validation failed: {}", e))?;
                        println!("✅ Credentials validation successful");
                    }
                    Err(e) => {
                        eprintln!("❌ Credentials validation failed: {}", e);
                        if strict {
                            std::process::exit(1);
                        }
                        return Err(anyhow::anyhow!("Credential validation failed: {}", e));
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Environment validation failed: {}", e);
                if strict {
                    std::process::exit(1);
                }
                return Err(anyhow::anyhow!("Environment validation failed: {}", e));
            }
        }
    }

    println!("\n🎉 All validations passed!");
    Ok(())
}

/// Handle generate command
pub async fn handle_generate(
    master_key: bool,
    session_secret: bool,
    all: bool,
    format: String,
) -> Result<()> {
    println!("🔑 Generating secure keys...\n");

    if all || master_key {
        println!("Master Encryption Key:");
        let key = generate_master_key(&format)?;
        println!("  {}", key);
        println!("  Set this as LAZABOT_MASTER_KEY environment variable");
        println!();
    }

    if all || session_secret {
        println!("Session Secret:");
        let secret = generate_session_secret(&format)?;
        println!("  {}", secret);
        println!("  Set this as LAZABOT_SESSION_SECRET environment variable");
        println!();
    }

    if !all && !master_key && !session_secret {
        println!("No keys specified. Use --master-key, --session-secret, or --all");
        println!("\nAvailable options:");
        println!("  --master-key     Generate master encryption key");
        println!("  --session-secret Generate session secret");
        println!("  --all            Generate all keys");
        println!("  --format         Output format (hex, base64)");
    }

    Ok(())
}

/// Handle credentials command
pub async fn handle_credentials(
    list: bool,
    add: bool,
    remove: bool,
    vault_path: String,
    _account_id: Option<String>,
) -> Result<()> {
    if list {
        println!("📋 Listing stored credentials...\n");
        
        match CredentialManager::new(&vault_path) {
            Ok(mut manager) => {
                manager.load_from_env().map_err(|e| anyhow::anyhow!("Failed to load credentials: {}", e))?;
                let vault_info = manager.get_vault_info();
                
                println!("🔐 Credential Vault: {}", vault_path);
                println!("  Created: {}", vault_info.created_at);
                println!("  Last updated: {}", vault_info.last_updated);
                println!();
                
                // List accounts
                if !vault_info.accounts.is_empty() {
                    println!("👤 Accounts ({}):", vault_info.accounts.len());
                    for (id, account) in &vault_info.accounts {
                        println!("  {}: {} ({})", id, account.username, account.email.as_deref().unwrap_or("no email"));
                    }
                    println!();
                }
                
                // List proxies
                if !vault_info.proxies.is_empty() {
                    println!("🌐 Proxies ({}):", vault_info.proxies.len());
                    for (id, proxy) in &vault_info.proxies {
                        let auth = if proxy.username.is_some() { "with auth" } else { "no auth" };
                        println!("  {}: {}:{} ({})", id, proxy.host, proxy.port, auth);
                    }
                    println!();
                }
                
                // List captcha
                if let Some(captcha) = &vault_info.captcha {
                    println!("🤖 Captcha: {} (endpoint: {})", 
                        captcha.api_key.chars().take(8).collect::<String>() + "...",
                        captcha.endpoint.as_deref().unwrap_or("default")
                    );
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to load credentials: {}", e);
                return Err(anyhow::anyhow!("Failed to load credentials: {}", e));
            }
        }
    } else if add {
        println!("➕ Adding credentials...");
        println!("This feature will be implemented in a future version");
        println!("For now, set environment variables and run 'lazabot validate'");
    } else if remove {
        println!("➖ Removing credentials...");
        println!("This feature will be implemented in a future version");
        println!("For now, remove environment variables and run 'lazabot validate'");
    } else {
        println!("🔐 Credentials management");
        println!("\nAvailable commands:");
        println!("  --list           List all stored credentials");
        println!("  --add            Add new credentials (not implemented)");
        println!("  --remove         Remove credentials (not implemented)");
        println!("  --vault-path     Path to credential vault");
        println!("  --account-id     Account ID for operations");
    }

    Ok(())
}

/// Generate a master encryption key
fn generate_master_key(format: &str) -> Result<String> {
    use rand::RngCore;
    
    let mut key_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key_bytes);
    
    match format.to_lowercase().as_str() {
        "hex" => Ok(hex::encode(key_bytes)),
        "base64" => {
            use base64::{engine::general_purpose, Engine as _};
            Ok(general_purpose::STANDARD.encode(key_bytes))
        }
        _ => {
            eprintln!("Invalid format: {}. Using hex format.", format);
            Ok(hex::encode(key_bytes))
        }
    }
}

/// Generate a session secret
fn generate_session_secret(format: &str) -> Result<String> {
    use rand::RngCore;
    
    let mut secret_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret_bytes);
    
    match format.to_lowercase().as_str() {
        "hex" => Ok(hex::encode(secret_bytes)),
        "base64" => {
            use base64::{engine::general_purpose, Engine as _};
            Ok(general_purpose::STANDARD.encode(secret_bytes))
        }
        _ => {
            eprintln!("Invalid format: {}. Using hex format.", format);
            Ok(hex::encode(secret_bytes))
        }
    }
}

/// Main command dispatcher
pub async fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Monitor {
            products,
            interval,
            verbose,
        } => handle_monitor(products, interval, verbose).await,
        Commands::Buy {
            product,
            quantity,
            dry_run,
        } => handle_buy(product, quantity, dry_run).await,
        Commands::Proxy {
            test,
            add,
            list,
            proxies,
        } => handle_proxy(test, add, list, proxies).await,
        Commands::Session {
            login,
            logout,
            status,
        } => handle_session(login, logout, status).await,
        Commands::Config {
            file,
            show,
            set,
            reset,
        } => handle_config(file, show, set, reset).await,
        Commands::Validate {
            verbose,
            credentials,
            vault_path,
            strict,
        } => handle_validate(verbose, credentials, vault_path, strict).await,
        Commands::Generate {
            master_key,
            session_secret,
            all,
            format,
        } => handle_generate(master_key, session_secret, all, format).await,
        Commands::Credentials {
            list,
            add,
            remove,
            vault_path,
            account_id,
        } => handle_credentials(list, add, remove, vault_path, account_id).await,
    }
}
