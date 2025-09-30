# Lazabot - Advanced Lazada CLI Bot

A comprehensive, production-ready Rust-based CLI bot for Lazada with advanced features including browser automation, proxy management, task orchestration, captcha solving, session management, automated testing, Docker deployment, Ubuntu server setup, and horizontal scaling capabilities.

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
- [Docker Deployment](#docker-deployment)
- [Ubuntu Server Deployment](#ubuntu-server-deployment)
- [Configuration](#configuration)
- [Horizontal Scaling](#horizontal-scaling)
- [Development](#development)
- [Contributing](#contributing)

## Overview

Lazabot is a high-performance CLI bot built with Rust and Tokio for automating Lazada operations. It features controlled concurrency, proxy management, comprehensive testing, browser automation, captcha solving, session management, automated deployment workflows, Docker support, Ubuntu server deployment, and horizontal scaling capabilities.

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

### Deployment Features
- ✅ **Docker Support**: Multi-stage build optimization with health checks
- ✅ **Ubuntu Server**: Automated setup scripts and systemd services
- ✅ **Horizontal Scaling**: Redis-based distributed task queues
- ✅ **Metrics & Monitoring**: Prometheus-compatible metrics server
- ✅ **Security**: Encrypted data storage and secure communication
- ✅ **Production Ready**: Comprehensive deployment documentation


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

### Systemd Services
- `lazabot.service`: Main Rust CLI application
- `lazabot-playwright.service`: Playwright RPC server


## Installation

### Prerequisites
- Rust 1.75+ with Cargo
- Git
- Node.js (v18+) for Playwright integration
- Network connectivity for external API calls
- 2Captcha API key (for captcha solving)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/Lothbrok303/lazabot.git
cd lazabot

# Install Rust dependencies
cargo build --release

# Install Node.js dependencies
npm install

# Install Playwright browsers
npx playwright install chromium
```

### Quick Start

```bash
# Run the CLI
./target/release/lazabot --help

# Start Playwright server
npm start
```


## 🐳 Docker Deployment

### Quick Start with Docker

```bash
# Build and run with Docker Compose
docker-compose up -d

# Or build and run manually
docker build -t lazabot:latest .
docker run -d --name lazabot -p 8081:8081 lazabot:latest
```

### Docker Features

- Multi-stage build optimization
- Rust binary + Node.js + Playwright
- Health checks and monitoring
- Volume persistence
- Network isolation

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


## 🖥️ Ubuntu Server Deployment

### Automated Setup

```bash
# Run setup script on Ubuntu server
curl -fsSL https://raw.githubusercontent.com/Lothbrok303/lazabot/main/scripts/setup.sh | bash

# Deploy application
REMOTE_HOST=192.168.1.56 ./scripts/deploy_remote.sh

# Test deployment
REMOTE_HOST=192.168.1.56 ./scripts/test_deployment.sh
```

### Manual Setup

```bash
# SSH to your Ubuntu server
ssh ubuntu@192.168.1.56

# Run setup script
curl -fsSL https://raw.githubusercontent.com/Lothbrok303/lazabot/main/scripts/setup.sh | bash

# Copy source code and deploy
sudo cp -r . /tmp/lazabot-source/
sudo /opt/lazabot/scripts/deploy.sh

# Configure environment
sudo nano /opt/lazabot/config/.env

# Start services
sudo systemctl start lazabot lazabot-playwright
sudo systemctl enable lazabot lazabot-playwright
```


## ⚙️ Configuration

### Environment Variables

Create a `.env` file with the following variables:

```bash
# Application settings
LAZABOT_LOG_LEVEL=info
LAZABOT_DATA_DIR=/opt/lazabot/data
LAZABOT_LOG_DIR=/opt/lazabot/logs

# Playwright settings
PLAYWRIGHT_SERVER_PORT=8081
PLAYWRIGHT_HEADLESS=true
PLAYWRIGHT_TIMEOUT=30000

# Database settings
DATABASE_URL=sqlite:///opt/lazabot/data/lazabot.db

# Security settings (REQUIRED - Generate new keys!)
ENCRYPTION_KEY=your-32-character-encryption-key-here
SESSION_SECRET=your-session-secret-key-here

# Proxy settings (optional)
PROXY_HOST=proxy.example.com
PROXY_PORT=8080
PROXY_USERNAME=username
PROXY_PASSWORD=password

# API settings
API_TIMEOUT=30
API_RETRY_ATTEMPTS=3
API_RETRY_DELAY=1000

# Rate limiting
RATE_LIMIT_REQUESTS_PER_MINUTE=60
RATE_LIMIT_BURST=10
```

### Generate Secure Keys

```bash
# Generate encryption key
openssl rand -hex 32

# Generate session secret
openssl rand -hex 32
```

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

## Checkout Engine

A comprehensive instant checkout engine for automated product purchasing.

### Features
- **Instant Checkout Flow**: Complete checkout process from add-to-cart to order submission
- **Retry Logic with Exponential Backoff**: Automatic retry for transient failures
- **Captcha Handling**: Automatic detection and solving of captchas (reCAPTCHA v2 supported)
- **Session Management**: Integration with session management for authenticated requests
- **Configurable Retry Policy**: Customizable retry attempts, delays, and backoff multipliers
- **Comprehensive Error Handling**: Clear error types with detailed messages

### Checkout Flow Steps
1. **Session Validation**: Verify that the session is valid
2. **Add to Cart**: Add the product to cart with retry logic (default: 3 retries)
3. **Get Checkout URL**: Retrieve the checkout URL from the cart
4. **Fill Shipping Info**: Update shipping address from account settings
5. **Select Payment Method**: Select payment method from account settings
6. **Captcha Handling**: Detect and solve captcha if present
7. **Submit Order**: Submit the order with retry logic (default: 3 retries)


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

## 📊 Monitoring

### Health Checks

```bash
# Check service status
sudo systemctl status lazabot lazabot-playwright

# View logs
sudo journalctl -u lazabot -f

# Health endpoint
curl http://localhost:8081/health
```

### Automated Monitoring

- Health checks every 5 minutes
- Service restart on failure
- Log rotation (7 days retention)
- Daily backups at 2 AM
- Resource usage monitoring

### Metrics and Monitoring

Lazabot includes a lightweight metrics server that exposes operational metrics in Prometheus format.

#### Features
- **Request Counters**: Total, success, and failed request tracking
- **Request Rate**: Real-time requests per second calculation
- **Active Tasks**: Current number of active tasks
- **Uptime Tracking**: System uptime in seconds
- **Prometheus Format**: Standard Prometheus text format for easy integration

#### Querying Metrics
```bash
# Get metrics in Prometheus format
curl http://localhost:9091/metrics

# Health check
curl http://localhost:9091/health
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

## 🔒 Security

### Security Features

- Dedicated `lazabot` user
- Proper file permissions
- Firewall configuration
- Encrypted data storage
- Secure session management
- Rate limiting
- Input validation

### Production Security

- SSL/TLS with reverse proxy
- Regular security updates
- Access logging
- Backup encryption
- Network isolation


## 🛠️ Troubleshooting

### Common Issues

1. **Service won't start**
   ```bash
   sudo journalctl -u lazabot -n 50
   ```

2. **Permission denied**
   ```bash
   sudo chown -R lazabot:lazabot /opt/lazabot
   ```

3. **Port already in use**
   ```bash
   sudo netstat -tlnp | grep :8081
   ```

4. **Playwright browser issues**
   ```bash
   npx playwright install chromium
   ```

### Debug Mode

```bash
# Enable debug logging
echo "LAZABOT_LOG_LEVEL=debug" | sudo tee -a /opt/lazabot/config/.env
sudo systemctl restart lazabot
```

## 🚀 Performance

### System Requirements

- **Minimum**: 2GB RAM, 10GB disk
- **Recommended**: 4GB RAM, 20GB disk
- **CPU**: 2+ cores

### Optimization

- File descriptor limits
- Memory monitoring
- Disk space management
- Resource usage tracking

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

### v0.1.3
- Added Docker deployment support
- Implemented Ubuntu server deployment
- Added horizontal scaling capabilities
- Created comprehensive monitoring system
- Enhanced security features
- Added production-ready deployment documentation

---

**Status**: ✅ Production Ready  
**Tests**: 83/83 PASSED  
**Last Updated**: October 1, 2025

## Deployment Options

### 1. Docker (Recommended for Development)

```bash
docker-compose up -d
```

### 2. Ubuntu Server (Recommended for Production)

```bash
# Automated setup
curl -fsSL https://raw.githubusercontent.com/Lothbrok303/lazabot/main/scripts/setup.sh | bash
```

### 3. Manual Installation

```bash
# Install dependencies
cargo build --release
npm install
npx playwright install chromium
```

---

**Ready to deploy!** 🚀

Choose your deployment method and follow the setup instructions. For production deployments, use the Ubuntu server setup with proper security configuration.

