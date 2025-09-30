use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lazabot")]
#[command(about = "A CLI bot for Lazada automation")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Monitor products and prices
    Monitor {
        /// Path to products YAML file
        #[arg(short, long, default_value = "examples/products.yaml")]
        products: Option<String>,
        /// Check interval in seconds (overrides product-specific intervals)
        #[arg(short = 'i', long, default_value = "0")]
        interval: u64,
        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },
    /// Buy products automatically
    Buy {
        /// Product URL or ID to buy
        #[arg(short, long)]
        product: Option<String>,
        /// Quantity to buy
        #[arg(short = 'q', long, default_value = "1")]
        quantity: u32,
        /// Dry run mode (don't actually buy)
        #[arg(long)]
        dry_run: bool,
    },
    /// Manage proxy settings
    Proxy {
        /// Test proxy connection
        #[arg(short, long)]
        test: bool,
        /// Add new proxy
        #[arg(short = 'a', long)]
        add: Option<String>,
        /// List all proxies
        #[arg(short = 'l', long)]
        list: bool,
        /// Path to proxy file
        #[arg(short = 'p', long)]
        proxies: Option<String>,
    },
    /// Manage session and authentication
    Session {
        /// Login with credentials
        #[arg(long)]
        login: bool,
        /// Logout and clear session
        #[arg(long)]
        logout: bool,
        /// Show current session status
        #[arg(short, long)]
        status: bool,
    },
    /// Manage configuration
    Config {
        /// Configuration file path
        #[arg(short, long)]
        file: Option<String>,
        /// Show configuration details
        #[arg(short, long)]
        show: bool,
        /// Set configuration value
        #[arg(short, long)]
        set: Option<String>,
        /// Reset to default configuration
        #[arg(long)]
        reset: bool,
    },
    /// Validate environment and configuration
    Validate {
        /// Show detailed validation report
        #[arg(short, long)]
        verbose: bool,
        /// Validate credentials only
        #[arg(long)]
        credentials: bool,
        /// Vault path for credential validation
        #[arg(long, default_value = "./data/credentials.vault")]
        vault_path: String,
        /// Exit with error code if validation fails
        #[arg(long)]
        strict: bool,
    },
    /// Generate secure keys and configuration
    Generate {
        /// Generate master encryption key
        #[arg(long)]
        master_key: bool,
        /// Generate session secret
        #[arg(long)]
        session_secret: bool,
        /// Generate all keys
        #[arg(long)]
        all: bool,
        /// Output format (hex, base64)
        #[arg(long, default_value = "hex")]
        format: String,
    },
    /// Manage credentials securely
    Credentials {
        /// List all stored credentials
        #[arg(short, long)]
        list: bool,
        /// Add new credentials
        #[arg(short, long)]
        add: bool,
        /// Remove credentials
        #[arg(short, long)]
        remove: bool,
        /// Vault path
        #[arg(long, default_value = "./data/credentials.vault")]
        vault_path: String,
        /// Account ID for operations
        #[arg(long)]
        account_id: Option<String>,
    },
}
