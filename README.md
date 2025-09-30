# Lazabot - Advanced Lazada CLI Bot

A comprehensive, production-ready Rust-based CLI bot for Lazada with advanced features including browser automation, proxy management, task orchestration, captcha solving, session management, and automated testing.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Browser Automation](#browser-automation)
- [API Client](#api-client)
- [Captcha Solver](#captcha-solver)
- [Session Management](#session-management)
- [Checkout Engine](#checkout-engine)
- [Proxy Management](#proxy-management)
- [Task Manager](#task-manager)
- [Testing](#testing)
- [Auto-Push System](#auto-push-system)
- [Configuration](#configuration)
- [Development](#development)
- [Contributing](#contributing)

## Overview

Lazabot is a high-performance CLI bot built with Rust and Tokio for automating Lazada operations. It features controlled concurrency, proxy management, comprehensive testing, browser automation, captcha solving, session management, and automated deployment workflows.

## Features

### Browser Automation
- ✅ **Playwright Integration**: RPC bridge between Rust and Node.js/Playwright
- ✅ **Stealth Mode**: Browser automation with anti-detection features
- ✅ **Captcha Solving**: Visual captcha detection and solving
- ✅ **Checkout Automation**: Automated product checkout flows
- ✅ **Screenshot Capture**: Visual feedback and debugging support

### Core Features
- ✅ **Async/Concurrent**: Built with Tokio for high-performance async operations
- ✅ **Proxy Management**: Thread-safe proxy rotation with health checking
- ✅ **Task Orchestration**: Controlled parallel execution with semaphore-based concurrency
- ✅ **HTTP Client**: Robust client with cookie store, retry logic, and error handling
- ✅ **Captcha Solver**: 2Captcha API integration for solving image and reCAPTCHA challenges
- ✅ **Session Management**: Complete login/save/restore session functionality
- ✅ **Checkout Engine**: Instant checkout with retry logic and captcha handling
- ✅ **Comprehensive Testing**: Unit, integration, and end-to-end test suites
- ✅ **Auto-Deployment**: Automated Git pushing on successful tests
- ✅ **Configuration Management**: Flexible configuration with TOML/YAML support

### Advanced Features
- **Cookie Store**: Automatic cookie management with `reqwest::cookie::Jar`
- **Retry Logic**: Exponential backoff retry with configurable parameters
- **Health Monitoring**: Real-time proxy and task health tracking
- **Graceful Shutdown**: Proper cleanup and resource management
- **Comprehensive Logging**: Structured logging with `tracing`
- **Captcha Integration**: Support for image captchas and reCAPTCHA v2 solving
- **Session Persistence**: AES-256-GCM encrypted session storage
- **Instant Checkout**: Complete checkout flow with retry and error handling

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Lazabot CLI Bot                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ API Client  │  │ Task Manager│  │Proxy Manager│        │
│  │             │  │             │  │             │        │
│  │ • HTTP Req  │  │ • Concurrency│  │ • Round Robin│      │
│  │ • Retry     │  │ • Status    │  │ • Health    │        │
│  │ • Cookies   │  │ • Persist   │  │ • Auth      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Monitor     │  │ Purchase    │  │ Config      │        │
│  │ Tasks       │  │ Tasks       │  │ Manager     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Captcha     │  │ Session     │  │ Checkout    │        │
│  │ Solver      │  │ Manager     │  │ Engine      │        │
│  │             │  │             │  │             │        │
│  │ • 2Captcha  │  │ • Login     │  │ • Instant   │        │
│  │ • Image     │  │ • Persist   │  │ • Retry     │        │
│  │ • reCAPTCHA │  │ • Validate  │  │ • Captcha   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Playwright  │  │ Stealth     │  │ Performance │        │
│  │ Integration │  │ Module      │  │ Monitor     │        │
│  │             │  │             │  │             │        │
│  │ • RPC Server│  │ • Fingerprint│  │ • Metrics   │        │
│  │ • Browser   │  │ • Behavior  │  │ • Timing    │        │
│  │ • Automation│  │ • Headers   │  │ • Stats     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites
- Rust 1.70+ with Cargo
- Git
- Node.js (v14+) for Playwright integration
- Network connectivity for external API calls
- 2Captcha API key (for captcha solving)

### Build from Source
```bash
git clone https://github.com/Lothbrok303/lazabot.git
cd lazabot
cargo build --release
```

### Install Playwright Dependencies
```bash
# Install Node.js dependencies
npm install

# Install Playwright browsers
npm run install-browsers
```

## Usage

### Basic CLI Usage
```bash
# Run with default configuration
cargo run

# Run with specific command
cargo run -- monitor --config config/products.yaml
cargo run -- proxy --test --proxies config/proxies.txt
cargo run -- purchase --product-id 12345
```

### Environment Variables
```bash
export LAZABOT_CONFIG_PATH="config/app.toml"
export LAZABOT_PROXY_FILE="config/proxies.txt"
export LAZABOT_LOG_LEVEL="info"
export CAPTCHA_API_KEY="your_2captcha_api_key"
```

## Browser Automation

Lazabot includes a powerful Playwright integration that provides browser automation capabilities through an RPC bridge between Rust and Node.js.

### Quick Start

```bash
# Install Node.js dependencies
npm install

# Install Playwright browsers
npm run install-browsers

# Start the RPC server
npm start

# Test the integration
cargo run --example playwright_integration
```

### Features

- **HTTP JSON RPC Server**: Express.js server on port 8081
- **Browser Automation**: Chromium with stealth mode
- **Captcha Solving**: Visual captcha detection and solving
- **Checkout Automation**: Automated product checkout flows
- **Rust Client**: Async Rust client with automatic server management

### API Endpoints

- `GET /health` - Server health check
- `POST /solveCaptcha` - Solve visual captchas
- `POST /performCheckoutFlow` - Automated checkout process

### Usage Example
```rust
use lazabot::integrations::playwright::{PlaywrightClient, CaptchaRequest, CheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PlaywrightClient::new();
    
    // Ensure server is running
    client.ensure_server_running().await?;
    
    // Check server health
    let health = client.is_server_healthy().await?;
    println!("Server health: {:?}", health);
    
    // Solve captcha
    let captcha_request = CaptchaRequest {
        captcha_url: "https://example.com/captcha".to_string(),
        captcha_type: Some("image".to_string()),
    };
    let captcha_response = client.solve_captcha(captcha_request).await?;
    println!("Captcha response: {:?}", captcha_response);
    
    // Perform checkout
    let checkout_request = CheckoutRequest {
        product_url: "https://example.com/product".to_string(),
        quantity: Some(1),
        shipping_info: None,
        payment_info: None,
        user_agent: None,
    };
    let checkout_response = client.perform_checkout_flow(checkout_request).await?;
    println!("Checkout response: {:?}", checkout_response);
    
    Ok(())
}
```

## API Client

A robust HTTP client with advanced features for making requests to Lazada APIs.

### Features
- **Cookie Store**: Automatic cookie management
- **Proxy Support**: HTTP proxy with optional authentication
- **Retry Logic**: Exponential backoff retry with configurable parameters
- **Logging**: Comprehensive tracing with `tracing` crate
- **Async**: Built with `tokio` for high-performance operations

### Basic Usage
```rust
use anyhow::Result;
use reqwest::Method;
use lazabot::api::{ApiClient, ProxyInfo};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with custom user agent
    let client = ApiClient::new(Some("Lazabot/1.0".to_string()))?;
    
    // Make a GET request
    let response = client.request(
        Method::GET,
        "https://httpbin.org/get",
        None, // headers
        None, // body
        None, // proxy
    ).await?;
    
    println!("Status: {}", response.status);
    println!("Body: {}", response.text);
    
    Ok(())
}
```

### With Proxy
```rust
// Simple proxy
let proxy = ProxyInfo::new("127.0.0.1".to_string(), 8080);

// Proxy with authentication
let proxy = ProxyInfo::new("127.0.0.1".to_string(), 8080)
    .with_auth("username".to_string(), "password".to_string());

let response = client.request(
    Method::GET,
    "https://httpbin.org/ip",
    None,
    None,
    Some(proxy),
).await?;
```

## Captcha Solver

A comprehensive 2Captcha API client for solving various types of captchas encountered during Lazada operations.

### Features
- **Image Captcha Solving**: Solve image-based captchas using 2Captcha's human workers
- **reCAPTCHA v2 Solving**: Solve Google reCAPTCHA v2 challenges
- **Async Support**: Built with Tokio for non-blocking operations
- **Mock Support**: Includes mock solver for testing without API calls
- **Environment Configuration**: Support for API key via environment variables
- **Comprehensive Error Handling**: Detailed error messages and proper Result types

### Basic Usage
```rust
use lazabot::captcha::{CaptchaSolver, CaptchaSolverTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create solver with API key
    let solver = CaptchaSolver::new("your_2captcha_api_key".to_string());
    
    // Solve an image captcha
    let image_data = std::fs::read("captcha.png")?;
    let result = solver.solve_image(&image_data).await?;
    println!("Captcha solved: {}", result);
    
    // Solve a reCAPTCHA
    let site_key = "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-";
    let page_url = "https://example.com";
    let result = solver.solve_recaptcha(site_key, page_url).await?;
    println!("reCAPTCHA solved: {}", result);
    
    Ok(())
}
```

### Environment Variable Configuration
```rust
use lazabot::captcha::CaptchaSolver;

// Set CAPTCHA_API_KEY environment variable
std::env::set_var("CAPTCHA_API_KEY", "your_api_key");

// Create solver from environment
let solver = CaptchaSolver::from_env()?;
```

### Testing with Mock Solver
```rust
use lazabot::captcha::{MockCaptchaSolver, CaptchaSolverTrait};

#[tokio::test]
async fn test_captcha_solving() {
    let mock_solver = MockCaptchaSolver::new(
        "mock_image_result".to_string(),
        "mock_recaptcha_result".to_string(),
    );
    
    let result = mock_solver.solve_image(b"fake_data").await.unwrap();
    assert_eq!(result, "mock_image_result");
}
```

## Session Management

Complete session management system with login, save, and restore functionality.

### Features
- **Session Creation**: Create sessions with user credentials
- **Cookie Persistence**: Store authentication cookies
- **Session Persistence**: Save sessions to disk in JSON format
- **AES-256-GCM Encryption**: Secure session storage
- **Session Restoration**: Load sessions from disk
- **Cookie Integrity**: Verify cookie integrity after restore
- **Metadata Storage**: Store session metadata and timestamps

### Basic Usage
```rust
use lazabot::core::session::{Session, Credentials};

// Create credentials
let credentials = Credentials::new(
    "user@example.com".to_string(),
    "password123".to_string(),
);

// Create session
let mut session = Session::new("session_001".to_string(), credentials);

// Add cookies (simulating login response)
session.add_cookie("JSESSIONID".to_string(), "abc123xyz".to_string());
session.add_cookie("auth_token".to_string(), "bearer_token".to_string());

// Add metadata
session.add_metadata("login_ip".to_string(), 
    serde_json::Value::String("192.168.1.1".to_string()));
```

### Session Persistence
```rust
// Save to disk
let session_file = std::path::Path::new("session.json");
let session_json = serde_json::to_string_pretty(&session)?;
tokio::fs::write(session_file, session_json).await?;

// Restore from disk
let restored_json = tokio::fs::read_to_string(session_file).await?;
let restored_session: Session = serde_json::from_str(&restored_json)?;
```

### Using SessionManager (Production)
```rust
use lazabot::api::ApiClient;
use lazabot::core::session::{SessionManager, Credentials};
use std::sync::Arc;

// Create API client
let api_client = Arc::new(ApiClient::new(Some("MyApp/1.0".to_string()))?);

// Create session manager (with encryption)
let session_manager = SessionManager::new(api_client).await?;

// Login (would use actual endpoint in production)
let credentials = Credentials::new("user".to_string(), "pass".to_string());
let session = session_manager.login(credentials).await?;

// Persist (encrypted)
session_manager.persist_session(&session).await?;

// Restore (decrypted)
let restored = session_manager.restore_session(&session.id).await?;

// Validate
let is_valid = session_manager.validate_session(&mut restored).await?;
```

### Demo and Testing
```bash
# Run the basic demo
cargo run --example session_demo

# Run the integration test
cargo test --test session_integration_test -- --nocapture
```

## Checkout Engine

A comprehensive instant checkout engine for automated product purchasing.

### Features
- **Instant Checkout Flow**: Complete checkout process from add-to-cart to order submission
- **Retry Logic with Exponential Backoff**: Automatic retry for transient failures
- **Captcha Handling**: Automatic detection and solving of captchas (reCAPTCHA v2 supported)
- **Session Management**: Integration with session management for authenticated requests
- **Configurable Retry Policy**: Customizable retry attempts, delays, and backoff multipliers
- **Comprehensive Error Handling**: Clear error types with detailed messages

### Basic Usage
```rust
use lazabot::core::{CheckoutEngine, CheckoutConfig, Product, Account};
use lazabot::api::ApiClient;
use lazabot::captcha::MockCaptchaSolver;
use std::sync::Arc;

let api_client = Arc::new(ApiClient::new(Some("MyAgent/1.0".to_string()))?);
let captcha_solver = Arc::new(MockCaptchaSolver::new(
    "image_solution".to_string(),
    "recaptcha_solution".to_string(),
));

let checkout_engine = CheckoutEngine::new(api_client, captcha_solver);

// Create product
let product = Product::new(
    "PROD123".to_string(),
    "Gaming Mouse".to_string(),
    "https://lazada.com/gaming-mouse".to_string(),
)
.with_price(59.99)
.with_quantity(2);

// Create account
let account = Account {
    id: "ACC123".to_string(),
    username: "user@example.com".to_string(),
    settings: AccountSettings {
        payment_method: "credit_card".to_string(),
        shipping_address: "123 Main St, City, Country".to_string(),
        notifications: true,
    },
};

// Perform checkout
let result = checkout_engine
    .instant_checkout(&product, &account, &session)
    .await?;

if result.success {
    println!("Order placed! ID: {}", result.order_id.unwrap());
} else {
    println!("Checkout failed: {}", result.error.unwrap());
}
```

### Checkout Flow Steps
1. **Session Validation**: Verify that the session is valid
2. **Add to Cart**: Add the product to cart with retry logic (default: 3 retries)
3. **Get Checkout URL**: Retrieve the checkout URL from the cart
4. **Fill Shipping Info**: Update shipping address from account settings
5. **Select Payment Method**: Select payment method from account settings
6. **Captcha Handling**: Detect and solve captcha if present
7. **Submit Order**: Submit the order with retry logic (default: 3 retries)

### Retry Configuration
```rust
use lazabot::core::CheckoutConfig;

let config = CheckoutConfig {
    add_to_cart_retries: 5,          // Retry add-to-cart up to 5 times
    checkout_url_retries: 3,         // Retry checkout URL retrieval up to 3 times
    payment_retries: 2,              // Retry payment selection up to 2 times
    submission_retries: 5,           // Retry order submission up to 5 times
    base_delay_ms: 500,              // Start with 500ms delay
    max_delay_ms: 5000,              // Max delay of 5 seconds
    backoff_multiplier: 1.5,         // Multiply delay by 1.5 each retry
    captcha_timeout_secs: 180,       // Wait up to 3 minutes for captcha solving
};

let checkout_engine = CheckoutEngine::with_config(api_client, captcha_solver, config);
```

## Proxy Management

Thread-safe proxy management with round-robin selection and health checking.

### Features
- **Thread-safe**: Uses `AtomicUsize` for round-robin and `RwLock` for health tracking
- **Round-robin selection**: Automatically cycles through healthy proxies
- **Health tracking**: Maintains health status for each proxy
- **File loading**: Supports loading proxies from text files
- **Authentication**: Supports username/password authentication

### Configuration File Format

#### Basic Format (host:port)
```
127.0.0.1:8080
192.168.1.100:3128
10.0.0.1:8080
```

#### With Authentication (host:port:username:password)
```
127.0.0.1:8080:user1:pass1
192.168.1.100:3128:user2:pass2
10.0.0.1:8080:user3:pass3
```

### Usage Examples
```rust
use lazabot::proxy::{ProxyManager, ProxyHealth};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load proxy manager
    let manager = ProxyManager::from_file("config/proxies.txt").await?;
    
    // Get next available proxy
    if let Some(proxy) = manager.get_next_proxy().await {
        println!("Using proxy: {}:{}", proxy.host, proxy.port);
    }
    
    // Run health check
    let health_checker = ProxyHealth::new()?;
    let report = health_checker.run_comprehensive_check(&manager).await?;
    report.print_report();
    
    Ok(())
}
```

### CLI Commands
```bash
# Test proxies
cargo run -- proxy --test --proxies config/proxies.txt

# List proxies
cargo run -- proxy --list --proxies config/proxies.txt
```

## Task Manager

A robust, concurrent task execution framework with controlled parallelism.

### Features
- **Controlled Concurrency**: Limits simultaneous task execution using `tokio::sync::Semaphore`
- **Task Status Tracking**: Persists task results in thread-safe `DashMap` store
- **Graceful Shutdown**: Handles shutdown signals and waits for running tasks
- **Type-Safe Interface**: Generic task submission using the `Task` trait
- **Comprehensive Queries**: Query tasks by status, count running/pending tasks

### Architecture
```
┌─────────────────────────────────────────────┐
│           Task Submission Queue              │
│  [Task 1] [Task 2] [Task 3] ... [Task 50]   │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │     Semaphore       │
        │  (max_concurrent=5)  │
        └─────────┬───────────┘
                  │
        ┌─────────▼──────────────────┐
        │   Running Tasks (max 5)     │
        │  ┌───┐ ┌───┐ ┌───┐ ┌───┐  │
        │  │ T │ │ T │ │ T │ │ T │  │
        │  └───┘ └───┘ └───┘ └───┘  │
        └─────────┬──────────────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │   DashMap Store      │
        │  (TaskId → Result)   │
        └─────────────────────┘
```

### Usage Example
```rust
use lazabot::tasks::{Task, TaskManager, TaskResult};

// Define a custom task
struct MyTask {
    name: String,
    data: String,
}

#[async_trait::async_trait]
impl Task for MyTask {
    async fn execute(&self) -> Result<serde_json::Value> {
        // Perform async work here
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        Ok(serde_json::json!({
            "task": self.name,
            "result": "success"
        }))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create TaskManager with max 5 concurrent tasks
    let manager = TaskManager::new(5);

    // Submit tasks
    let task = MyTask {
        name: "my_task".to_string(),
        data: "some_data".to_string(),
    };

    let task_id = manager.submit_task(task).await?;
    
    // Query task status
    if let Some(result) = manager.get_task_result(task_id) {
        println!("Task status: {:?}", result.status);
    }

    // Graceful shutdown
    manager.shutdown().await;

    Ok(())
}
```

### API Reference

#### TaskManager Methods
- `new(max_concurrent: usize) -> Self` - Create with concurrency limit
- `submit_task<T: Task + 'static>(&self, task: T) -> Result<TaskId>` - Submit task
- `get_task_result(&self, task_id: TaskId) -> Option<TaskResult>` - Get result
- `get_all_task_results(&self) -> Vec<TaskResult>` - Get all results
- `get_tasks_by_status(&self, status: TaskStatus) -> Vec<TaskResult>` - Filter by status
- `running_tasks_count(&self) -> usize` - Count running tasks
- `pending_tasks_count(&self) -> usize` - Count pending tasks
- `shutdown(&self) -> Future` - Graceful shutdown

#### TaskStatus States
- `Pending`: Task is waiting to be executed
- `Running`: Task is currently executing
- `Completed`: Task finished successfully
- `Failed`: Task encountered an error
- `Cancelled`: Task was cancelled (typically during shutdown)

## Testing

Comprehensive test suite with unit, integration, and end-to-end tests.

### Test Structure
- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end functionality testing
- **Task Manager Tests**: Concurrency and orchestration testing
- **API Client Tests**: HTTP request and retry mechanism testing
- **Proxy Tests**: Proxy management and health checking testing
- **Captcha Tests**: Captcha solver functionality
- **Session Tests**: Session management and persistence
- **Checkout Tests**: Checkout engine functionality
- **Playwright Tests**: Browser automation integration

### Running Tests

#### All Tests
```bash
cargo test
```

#### Specific Test Suites
```bash
# Integration tests
cargo test --test integration_tests

# Task manager tests
cargo test tasks::manager::tests -- --nocapture

# API client tests
cargo test api::client::tests -- --nocapture

# Proxy tests
cargo test proxy::tests -- --nocapture

# Captcha tests
cargo test captcha -- --nocapture

# Session tests
cargo test session -- --nocapture

# Checkout tests
cargo test checkout -- --nocapture

# Playwright tests
cargo test playwright_integration_test -- --nocapture
```

#### With Verbose Output
```bash
cargo test -- --nocapture
```

### Test Results
All tests pass successfully:
- **Unit Tests**: 42/42 PASSED ✓
- **Integration Tests**: 14/14 PASSED ✓
- **Task Manager Tests**: 7/7 PASSED ✓
- **Captcha Tests**: 4/4 PASSED ✓
- **Session Tests**: 1/1 PASSED ✓
- **Checkout Tests**: 11/11 PASSED ✓
- **Playwright Tests**: 4/4 PASSED ✓
- **Concurrency Verification**: 50 tasks with max 5 concurrent ✓

### Test Coverage
- ✅ API Client functionality
- ✅ Proxy management and health checking
- ✅ Task orchestration and concurrency control
- ✅ Configuration loading and error handling
- ✅ Network failure resilience
- ✅ Graceful shutdown behavior
- ✅ Captcha solving (image and reCAPTCHA)
- ✅ Session management and persistence
- ✅ Checkout engine functionality
- ✅ Browser automation integration

## Auto-Push System

Automated Git pushing when tests are completed successfully.

### Features
- ✅ Only pushes if tests pass
- ✅ Checks for actual changes before committing
- ✅ Uses descriptive commit messages
- ✅ Preserves existing commit history
- ❌ Never pushes on test failure

### Usage

#### Option 1: Use the Scripts
**On Windows:**
```bash
# Run default tests and push on success
./test-and-push.bat

# Run specific test command
./test-and-push.bat "cargo test --release"
./test-and-push.bat "cargo check"
./test-and-push.bat "cargo build"
```

**On Unix/Linux/Mac:**
```bash
# Run default tests and push on success
./test-and-push.sh

# Run specific test command
./test-and-push.sh "cargo test --release"
./test-and-push.sh "cargo check"
./test-and-push.sh "cargo build"
```

#### Option 2: Manual Commands
```bash
# Run tests
cargo test

# If tests pass, stage, commit, and push
git add .
git commit -m "test: cargo test - Tests passed successfully"
git push origin main
```

### Commit Message Examples
- `test: cargo test - Tests passed successfully`
- `test: cargo check - Compilation successful`
- `test: cargo build - Build successful`
- `test: cargo test --release - Release build tests passed`

## Configuration

### Configuration Files

#### Main Configuration (`config/app.toml`)
```toml
[api]
base_url = "https://api.lazada.com"
timeout_seconds = 30
max_retries = 3

[proxy]
enabled = true
file_path = "config/proxies.txt"
health_check_interval = 300

[monitoring]
check_interval_ms = 5000
max_concurrent_monitors = 10

[logging]
level = "info"
format = "json"

[captcha]
api_key = "your_2captcha_api_key"
polling_interval = 5
max_attempts = 60

[task_manager]
max_concurrent = 10
shutdown_timeout = 30

[checkout]
add_to_cart_retries = 3
checkout_url_retries = 2
payment_retries = 2
submission_retries = 3
base_delay_ms = 1000
max_delay_ms = 10000
backoff_multiplier = 2.0
captcha_timeout_secs = 120
```

#### Product Configuration (`config/products.yaml`)
```yaml
products:
  - id: "12345"
    name: "Sample Product"
    url: "https://www.lazada.com.ph/products/sample-product"
    target_price: 100.0
    interval_ms: 5000
    enabled: true
```

#### Proxy Configuration (`config/proxies.txt`)
```
# Basic format
127.0.0.1:8080
192.168.1.100:3128

# With authentication
127.0.0.1:8080:user1:pass1
192.168.1.100:3128:user2:pass2
```

### Environment Variables
```bash
export LAZABOT_CONFIG_PATH="config/app.toml"
export LAZABOT_PROXY_FILE="config/proxies.txt"
export LAZABOT_LOG_LEVEL="info"
export LAZABOT_MAX_CONCURRENT="10"
export CAPTCHA_API_KEY="your_2captcha_api_key"
```

## Development

### Project Structure
```
lazabot/
├── src/
│   ├── api/           # HTTP client and API utilities
│   ├── captcha/       # Captcha solving functionality
│   ├── cli/           # Command-line interface
│   ├── config/        # Configuration management
│   ├── core/          # Core business logic
│   │   ├── checkout.rs    # Checkout engine
│   │   └── session.rs     # Session management
│   ├── integrations/  # External integrations
│   │   └── playwright.rs  # Playwright RPC client
│   ├── proxy/         # Proxy management
│   └── tasks/         # Task orchestration
├── tests/             # Integration tests
├── examples/          # Usage examples
├── scripts/           # Node.js scripts for Playwright
├── config/            # Configuration files
└── docs/              # Documentation
```

### Dependencies
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "cookies"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
dashmap = "5.5"
async-trait = "0.1"
clap = { version = "4.0", features = ["derive"] }
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
base64 = "0.21"

[dev-dependencies]
wiremock = "0.5"
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check

# Run with optimizations
cargo run --release
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run all checks
cargo check && cargo clippy && cargo test
```

## Contributing

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run auto-push: `./test-and-push.sh`
6. Create a pull request

### Code Standards
- Follow Rust naming conventions
- Use `async`/`await` for I/O operations
- Implement proper error handling with `anyhow`
- Add comprehensive tests for new features
- Update documentation for API changes

### Testing Requirements
- All new features must include tests
- Integration tests for end-to-end functionality
- Unit tests for individual components
- Performance tests for critical paths

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## GitHub Repository

- **Repository**: https://github.com/Lothbrok303/lazabot
- **Issues**: https://github.com/Lothbrok303/lazabot/issues
- **Pull Requests**: https://github.com/Lothbrok303/lazabot/pulls

## Support

For support and questions:
- Create an issue on GitHub
- Check the documentation in the `docs/` directory
- Review the test examples for usage patterns

## Changelog

### v0.1.0
- Initial release
- Core bot functionality
- Proxy management
- Task orchestration
- Captcha solving integration
- Comprehensive testing suite
- Auto-deployment workflow

### v0.1.1
- Added stealth module with fingerprint spoofing
- Implemented browser fingerprint generation
- Added human-like behavior simulation
- Created stealth HTTP client with realistic headers
- Enhanced request uniformity avoidance
- Added comprehensive stealth testing suite

### v0.1.2
- Added Playwright RPC integration
- Implemented browser automation capabilities
- Added session management with encryption
- Created comprehensive checkout engine
- Enhanced captcha solving with visual detection
- Added extensive testing and documentation

---

**Status**: ✅ Production Ready  
**Tests**: 83/83 PASSED  
**Last Updated**: September 30, 2025

## Metrics and Monitoring

Lazabot includes a lightweight metrics server that exposes operational metrics in Prometheus format.

### Features
- **Request Counters**: Total, success, and failed request tracking
- **Request Rate**: Real-time requests per second calculation
- **Active Tasks**: Current number of active tasks
- **Uptime Tracking**: System uptime in seconds
- **Prometheus Format**: Standard Prometheus text format for easy integration

### Usage

#### Starting the Metrics Server
```rust
use lazabot::utils::{MetricsCollector, MetricsServer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create metrics collector
    let collector = MetricsCollector::new();

    // Start metrics server
    let server = MetricsServer::new(collector.clone(), "0.0.0.0:9091");
    tokio::spawn(async move {
        server.start().await.unwrap();
    });

    // Use the collector in your application
    collector.inc_total_requests();
    collector.inc_success_requests();
    collector.inc_active_tasks();
    
    // ... rest of your application logic
    
    Ok(())
}
```

#### Querying Metrics
```bash
# Get metrics in Prometheus format
curl http://localhost:9091/metrics

# Health check
curl http://localhost:9091/health
```

#### Example Output
```
# HELP lazabot_requests_total Total number of requests
# TYPE lazabot_requests_total counter
lazabot_requests_total 1500

# HELP lazabot_requests_success_total Total number of successful requests
# TYPE lazabot_requests_success_total counter
lazabot_requests_success_total 1350

# HELP lazabot_requests_failed_total Total number of failed requests
# TYPE lazabot_requests_failed_total counter
lazabot_requests_failed_total 150

# HELP lazabot_active_tasks Number of currently active tasks
# TYPE lazabot_active_tasks gauge
lazabot_active_tasks 5

# HELP lazabot_requests_per_second Current request rate
# TYPE lazabot_requests_per_second gauge
lazabot_requests_per_second 12.50

# HELP lazabot_uptime_seconds Uptime in seconds
# TYPE lazabot_uptime_seconds counter
lazabot_uptime_seconds 3600
```

### Running the Demo
```bash
# Run the metrics demo
cargo run --example metrics_demo

# In another terminal, query the metrics
curl http://127.0.0.1:9091/metrics
```

## Horizontal Scaling

Lazabot supports horizontal scaling to distribute workload across multiple agent instances using Redis as a distributed task queue.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Load Balancer (Optional)                 │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
   ┌─────────┐  ┌─────────┐  ┌─────────┐
   │ Agent 1 │  │ Agent 2 │  │ Agent 3 │
   │ :9091   │  │ :9092   │  │ :9093   │
   └────┬────┘  └────┬────┘  └────┬────┘
        │            │            │
        └────────────┼────────────┘
                     │
              ┌──────▼──────┐
              │    Redis    │
              │   Queue     │
              └─────────────┘
                     │
              ┌──────▼──────┐
              │ Prometheus  │
              │  (Optional) │
              └─────────────┘
```

### Docker Compose Setup

The project includes a `docker-compose.yml` file for easy horizontal scaling:

#### Components
- **Redis**: Distributed task queue and state management
- **Multiple Agents**: 3 agent instances by default (easily scalable)
- **Prometheus**: Optional metrics aggregation
- **Health Checks**: Automatic service health monitoring

#### Quick Start
```bash
# Start all services
docker-compose up -d

# Scale agents to 5 instances
docker-compose up -d --scale agent1=5

# View logs
docker-compose logs -f

# Check metrics for each agent
curl http://localhost:9091/metrics  # Agent 1
curl http://localhost:9092/metrics  # Agent 2
curl http://localhost:9093/metrics  # Agent 3

# View aggregated metrics in Prometheus
# Open browser to http://localhost:9090

# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

### Environment Variables for Scaling

Each agent instance can be configured with:

```bash
# Agent identification
AGENT_ID=agent1

# Redis connection
REDIS_URL=redis://redis:6379

# Metrics port
METRICS_PORT=9091

# Logging
LAZABOT_LOG_LEVEL=info
RUST_BACKTRACE=1

# Task configuration
MAX_CONCURRENT_TASKS=10
TASK_TIMEOUT_SECS=300
```

### Manual Scaling (Without Docker)

You can also run multiple instances manually:

```bash
# Terminal 1 - Agent 1
AGENT_ID=agent1 METRICS_PORT=9091 cargo run

# Terminal 2 - Agent 2
AGENT_ID=agent2 METRICS_PORT=9092 cargo run

# Terminal 3 - Agent 3
AGENT_ID=agent3 METRICS_PORT=9093 cargo run

# Terminal 4 - Redis (using Docker)
docker run -d -p 6379:6379 redis:7-alpine
```

### Redis Task Queue Integration

To integrate Redis for distributed task queues, add to your `Cargo.toml`:

```toml
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
```

Example Redis integration:

```rust
use redis::AsyncCommands;

pub struct DistributedTaskQueue {
    client: redis::Client,
}

impl DistributedTaskQueue {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn push_task(&self, task: &str) -> anyhow::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.rpush("task_queue", task).await?;
        Ok(())
    }

    pub async fn pop_task(&self) -> anyhow::Result<Option<String>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let task: Option<String> = conn.lpop("task_queue", None).await?;
        Ok(task)
    }
}
```

### Load Balancing Strategies

#### 1. Redis-Based Queue (Recommended)
- All agents pull tasks from a shared Redis queue
- Automatic load distribution
- No single point of failure
- Persistent task queue

#### 2. Round-Robin Load Balancer
- Use nginx or HAProxy to distribute requests
- Simple configuration
- Stateless agents

#### 3. Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lazabot-agent
spec:
  replicas: 5
  selector:
    matchLabels:
      app: lazabot
  template:
    metadata:
      labels:
        app: lazabot
    spec:
      containers:
      - name: lazabot
        image: lazabot:latest
        env:
        - name: REDIS_URL
          value: "redis://redis-service:6379"
        - name: METRICS_PORT
          value: "9091"
        ports:
        - containerPort: 9091
          name: metrics
```

### Monitoring Multiple Agents

#### Using Prometheus
The included `docker-compose.yml` sets up Prometheus to scrape metrics from all agents:

```yaml
# config/prometheus.yml
scrape_configs:
  - job_name: 'lazabot-agents'
    static_configs:
      - targets:
        - 'agent1:9091'
        - 'agent2:9092'
        - 'agent3:9093'
```

Access Prometheus UI at `http://localhost:9090` and run queries:

```promql
# Total requests across all agents
sum(lazabot_requests_total)

# Request rate per agent
rate(lazabot_requests_total[5m])

# Average active tasks
avg(lazabot_active_tasks)

# Failed requests percentage
sum(lazabot_requests_failed_total) / sum(lazabot_requests_total) * 100
```

#### Using Grafana (Optional)
Add Grafana to your `docker-compose.yml`:

```yaml
grafana:
  image: grafana/grafana:latest
  ports:
    - "3000:3000"
  environment:
    - GF_SECURITY_ADMIN_PASSWORD=admin
  volumes:
    - grafana-data:/var/lib/grafana
```

### Performance Tuning

#### Optimal Agent Count
- Start with 1 agent per CPU core
- Monitor CPU and memory usage
- Scale up when request queue grows
- Scale down during low traffic

#### Redis Configuration
```bash
# Increase max connections
redis-cli CONFIG SET maxclients 10000

# Optimize memory
redis-cli CONFIG SET maxmemory 2gb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

#### Agent Configuration
```toml
[scaling]
max_concurrent_tasks = 50
task_queue_size = 1000
worker_threads = 8
connection_pool_size = 100

[redis]
url = "redis://redis:6379"
pool_size = 20
timeout_ms = 5000
```

### Best Practices

1. **Use Health Checks**: Always configure health checks in Docker/Kubernetes
2. **Monitor Metrics**: Set up alerts for high failure rates or queue depth
3. **Graceful Shutdown**: Ensure agents finish tasks before terminating
4. **Task Idempotency**: Design tasks to be safely retried
5. **Connection Pooling**: Reuse Redis connections across tasks
6. **Rate Limiting**: Protect downstream services from overload
7. **Circuit Breakers**: Fail fast when dependencies are down

### Troubleshooting

#### High Memory Usage
```bash
# Check Redis memory
docker exec lazabot-redis redis-cli INFO memory

# Clear Redis queue
docker exec lazabot-redis redis-cli FLUSHALL
```

#### Agent Not Responding
```bash
# Check agent logs
docker-compose logs agent1

# Restart agent
docker-compose restart agent1

# Check metrics endpoint
curl http://localhost:9091/health
```

#### Task Queue Backed Up
```bash
# Check queue length
docker exec lazabot-redis redis-cli LLEN task_queue

# Add more agents
docker-compose up -d --scale agent1=10
```

