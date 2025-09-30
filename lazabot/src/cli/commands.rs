use anyhow::{Result, Context};
use std::sync::Arc;
use std::fs;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::cli::args::Commands;
use crate::config::{load_config, create_default_config};
use crate::proxy::{ProxyManager, ProxyHealth};
use crate::api::ApiClient;
use crate::core::{MonitorEngine, MonitorTask};

/// Product configuration loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub target_price: Option<f64>,
    pub min_stock: Option<u32>,
    pub monitor_interval_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductsConfig {
    pub products: Vec<ProductConfig>,
    #[serde(rename = "test_products")]
    pub test_products: Option<Vec<ProductConfig>>,
}

/// Load products from YAML file
pub fn load_products_from_yaml(file_path: &str) -> Result<Vec<ProductConfig>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read products file: {}", file_path))?;
    
    let config: ProductsConfig = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse YAML products file: {}", file_path))?;
    
    Ok(config.products)
}

/// Handle monitor command
pub async fn handle_monitor(products_file: Option<String>, interval: u64, verbose: bool) -> Result<()> {
    let products_file = products_file.unwrap_or_else(|| "examples/products.yaml".to_string());
    
    info!("Loading products from: {}", products_file);
    
    // Load products from YAML
    let products = load_products_from_yaml(&products_file)?;
    info!("Loaded {} products to monitor", products.len());
    
    // Create API client
    let api_client = Arc::new(ApiClient::new(Some("Lazabot-Monitor/1.0".to_string()))?);
    
    // Create proxy manager (empty for now, can be loaded from config)
    let proxy_manager = Arc::new(ProxyManager::new(vec![]));
    
    // Create monitor engine
    let mut engine = MonitorEngine::new();
    let mut event_receivers = Vec::new();
    
    // Create monitor tasks for each product
    for product in products {
        let monitor_interval = if interval > 0 { interval * 1000 } else { product.monitor_interval_ms };
        
        let monitor = MonitorTask::new(
            product.id.clone(),
            product.url.clone(),
            product.name.clone(),
            api_client.clone(),
            proxy_manager.clone(),
            monitor_interval,
        )
        .with_timeout(30000) // 30 second timeout
        .with_max_retries(3);
        
        // Apply target price and min stock if specified
        let monitor = if let Some(price) = product.target_price {
            monitor.with_target_price(price)
        } else {
            monitor
        };
        
        let monitor = if let Some(stock) = product.min_stock {
            monitor.with_min_stock(stock)
        } else {
            monitor
        };
        
        let receiver = engine.add_monitor(monitor);
        event_receivers.push((product.id.clone(), product.name.clone(), receiver));
        
        info!("Added monitor for: {} ({})", product.name, product.id);
    }
    
    // Start the engine
    engine.start().await?;
    info!("Monitor engine started with {} tasks", event_receivers.len());
    
    // Spawn event handler
    let event_handle = tokio::spawn(async move {
        for (_product_id, product_name, mut receiver) in event_receivers {
            while let Some(event) = receiver.recv().await {
                if event.is_available {
                    println!("✅ Product '{}' is now AVAILABLE!", product_name);
                    println!("   URL: {}", event.product_url);
                    println!("   Timestamp: {}", event.timestamp);
                } else {
                    if verbose {
                        println!("❌ Product '{}' is now UNAVAILABLE", product_name);
                    }
                }
            }
        }
    });
    
    // Run until interrupted
    info!("Monitoring products... Press Ctrl+C to stop");
    
    // Wait for Ctrl+C or event handler to finish
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, stopping monitor...");
        }
        _ = event_handle => {
            info!("Event handler finished");
        }
    }
    
    // Stop the engine
    engine.stop().await?;
    info!("Monitor stopped successfully");
    
    Ok(())
}

/// Handle buy command
pub async fn handle_buy(product: Option<String>, quantity: u32, dry_run: bool) -> Result<()> {
    println!("Buy command executed");
    println!("Product: {:?}", product);
    println!("Quantity: {}", quantity);
    println!("Dry run: {}", dry_run);
    Ok(())
}

/// Handle proxy command
pub async fn handle_proxy(test: bool, add: Option<String>, list: bool, proxies: Option<String>) -> Result<()> {
    if test {
        let proxy_file = proxies.unwrap_or_else(|| "config/proxies.txt".to_string());
        println!("Testing proxies from: {}", proxy_file);
        
        // Load proxy manager
        let manager = ProxyManager::from_file(&proxy_file).await?;
        println!("Loaded {} proxies", manager.total_proxies());
        
        // Create health checker
        let health_checker = ProxyHealth::new()?;
        
        // Run comprehensive health check
        let report = health_checker.run_comprehensive_check(&manager).await?;
        report.print_report();
        
        // Test round-robin selection
        println!("\n=== Testing Round-Robin Selection ===");
        for i in 0..5 {
            if let Some(proxy) = manager.get_next_proxy().await {
                println!("Selection {}: {}:{}", i + 1, proxy.host, proxy.port);
            } else {
                println!("Selection {}: No healthy proxies available", i + 1);
            }
        }
        
        // Show integration example
        println!("\n=== Integration with ApiClient ===");
        println!("To use proxies with ApiClient:");
        println!("1. Get a proxy: let proxy = manager.get_next_proxy().await;");
        println!("2. Use in request: client.request(method, url, headers, body, proxy).await");
        println!("3. Handle failures: if request fails, mark proxy as unhealthy");
        
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
pub async fn handle_config(file: Option<String>, show: bool, set: Option<String>, reset: bool) -> Result<()> {
    if reset {
        println!("Resetting to default configuration...");
        let default_config = create_default_config();
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
                println!("Stealth mode: random_delays={}, proxy_rotation={}", 
                    config.stealth.random_delays, config.stealth.proxy_rotation);
                println!("Monitoring: enabled={}, log_level={}", 
                    config.monitoring.enable_logging, config.monitoring.log_level);
                
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

/// Main command dispatcher
pub async fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Monitor { products, interval, verbose } => {
            handle_monitor(products, interval, verbose).await
        }
        Commands::Buy { product, quantity, dry_run } => {
            handle_buy(product, quantity, dry_run).await
        }
        Commands::Proxy { test, add, list, proxies } => {
            handle_proxy(test, add, list, proxies).await
        }
        Commands::Session { login, logout, status } => {
            handle_session(login, logout, status).await
        }
        Commands::Config { file, show, set, reset } => {
            handle_config(file, show, set, reset).await
        }
    }
}
