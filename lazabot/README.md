# Lazabot - Lazada CLI Bot

A powerful Rust-based CLI bot for automating Lazada operations with Playwright integration.

## ��� Features

- **Rust CLI**: High-performance command-line interface
- **Playwright Integration**: Browser automation with stealth capabilities
- **Proxy Support**: Rotating proxy management with health checks
- **Session Management**: Persistent session handling with encryption
- **Task Management**: Concurrent task execution with monitoring
- **Storage**: SQLite database with caching
- **Security**: Encrypted data storage and secure communication
- **Monitoring**: Real-time performance and health monitoring

## ��� Quick Start

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Playwright browsers

### Installation

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

### Basic Usage

```bash
# Run the CLI
./target/release/lazabot --help

# Start Playwright server
npm start
```

## ��� Docker Deployment

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

## ���️ Ubuntu Server Deployment

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

## ��� Configuration

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

## ���️ Architecture

### Core Modules

- **API Client**: HTTP client with retry logic and proxy support
- **Session Management**: Encrypted session storage and validation
- **Task Manager**: Concurrent task execution with monitoring
- **Proxy Manager**: Rotating proxy management with health checks
- **Storage**: SQLite database with in-memory caching
- **Stealth**: Browser fingerprinting and behavior simulation
- **Playwright Integration**: Browser automation with RPC server

### Systemd Services

- `lazabot.service`: Main Rust CLI application
- `lazabot-playwright.service`: Playwright RPC server

## ��� Monitoring

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

## ��� Security

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

## ��� Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test api_client
cargo test --test integration_tests

# Run with coverage
cargo test --lib
```

### Smoke Testing

The project includes comprehensive smoke tests that validate the complete pipeline:

```bash
# Run the complete smoke test
bash scripts/smoke_test.sh

# Verify smoke test results
bash scripts/verify_results.sh
```

#### Smoke Test Features

- **Mock Lazada API Server**: Simulates real Lazada endpoints
- **Product Monitoring**: Tests product availability detection
- **Flash Sale Simulation**: Triggers mock flash sales
- **Checkout Process**: Simulates order creation
- **Database Storage**: Verifies order persistence
- **End-to-End Validation**: Complete pipeline testing

#### Expected Output

```
╔══════════════════════════════════════════════════════════════╗
║                    LAZABOT SMOKE TEST                        ║
║                                                              ║
║  Monitor → Flash Sale Detection → Checkout → Database        ║
╚══════════════════════════════════════════════════════════════╝

[12:34:56] Checking prerequisites...
✓ Prerequisites check passed
[12:34:56] Creating test products configuration...
✓ Test products configuration created
[12:34:57] Starting mock Lazada API server...
✓ Mock server started (PID: 12345)
[12:34:58] Building lazabot...
✓ Lazabot built successfully
[12:34:59] Simulating monitoring and checkout process...
✓ Flash sale triggered successfully
✓ Checkout attempt successful
[12:35:05] Verifying database storage...
✓ Order found in mock server (count: 1)

🎉 ALL TESTS PASSED! 🎉
The core pipeline is working correctly.
```

#### Verification Commands

```bash
# Check database for order rows
sqlite3 smoke_test.db "SELECT * FROM orders;"

# Check logs for checkout triggered
grep -i "checkout triggered" lazabot.log

# Verify mock server orders
curl -s http://localhost:3001/api/orders | jq
```

### Test Coverage

- Unit tests for all modules
- Integration tests with mock servers
- API client tests (11/11 passing)
- Deployment configuration tests
- Security configuration tests

## �� Documentation

- [Deployment Guide](DEPLOYMENT.md) - Complete deployment documentation
- [Docker Setup](docker-compose.yml) - Docker configuration
- [API Reference](docs/api.md) - API documentation
- [Configuration](docs/config.md) - Configuration options

## ��� Deployment Options

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

## ��� Troubleshooting

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

## ��� Performance

### System Requirements

- **Minimum**: 2GB RAM, 10GB disk
- **Recommended**: 4GB RAM, 20GB disk
- **CPU**: 2+ cores

### Optimization

- File descriptor limits
- Memory monitoring
- Disk space management
- Resource usage tracking

## ��� Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## ��� License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## �� Support

For issues and questions:

1. Check the [troubleshooting guide](DEPLOYMENT.md#troubleshooting)
2. Review the [documentation](DEPLOYMENT.md)
3. Check [GitHub issues](https://github.com/Lothbrok303/lazabot/issues)
4. Create a new issue with detailed information

## ��� Roadmap

- [ ] Web dashboard for monitoring
- [ ] Advanced proxy rotation strategies
- [ ] Machine learning for captcha solving
- [ ] Multi-platform support
- [ ] API rate limiting improvements
- [ ] Enhanced security features

---

**Ready to deploy!** ���

Choose your deployment method and follow the setup instructions. For production deployments, use the Ubuntu server setup with proper security configuration.

## Encryption and Security

### Master Key Management

The Lazabot uses AES-GCM encryption to protect sensitive data like passwords, API keys, and other credentials. All sensitive fields in the configuration are automatically encrypted using a master key.

#### Generating a Master Key

Generate a secure 256-bit (32-byte) master key using OpenSSL:

```bash
# Generate a new master key (64 hex characters)
openssl rand -hex 32

# Example output:
# a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
```

#### Setting the Master Key

Set the master key as an environment variable:

```bash
# For current session
export LAZABOT_MASTER_KEY="a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"

# For permanent storage (add to ~/.bashrc or ~/.zshrc)
echo 'export LAZABOT_MASTER_KEY="a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456"' >> ~/.bashrc
```

#### Secure Key Storage

**⚠️ IMPORTANT: Store your master key securely!**

1. **Password Managers**: Store the key in a secure password manager (1Password, Bitwarden, etc.)
2. **Key Vaults**: Use cloud key management services (AWS KMS, Azure Key Vault, HashiCorp Vault)
3. **Hardware Security Modules (HSM)**: For enterprise deployments
4. **Environment Files**: Use `.env` files (never commit to version control)

#### Key Rotation Procedure

Regular key rotation is essential for security:

1. **Generate New Key**:
   ```bash
   openssl rand -hex 32
   ```

2. **Backup Current Data**: Export all encrypted data before rotation

3. **Decrypt with Old Key**: Use the old key to decrypt all sensitive data

4. **Encrypt with New Key**: Re-encrypt all data with the new key

5. **Update Environment**: Set the new `LAZABOT_MASTER_KEY`

6. **Verify**: Test that all encrypted data can be decrypted with the new key

7. **Secure Disposal**: Securely delete the old key

### Encrypted Fields

The following configuration fields are automatically encrypted:

- Account passwords
- Proxy authentication credentials
- API keys (captcha services, etc.)
- Webhook URLs with sensitive tokens
- Any field marked as `encrypted: true` in the configuration

### Encryption Usage in Code

```rust
use lazabot::config::encryption::{init_encryption, encrypt, decrypt, encrypt_field, decrypt_field};

// Initialize encryption (call once at startup)
init_encryption()?;

// Encrypt sensitive data
let encrypted_password = encrypt("my_secret_password")?;

// Decrypt sensitive data
let decrypted_password = decrypt(&encrypted_password)?;

// Encrypt/decrypt configuration fields
let encrypted_field = encrypt_field("sensitive_value")?;
let decrypted_field = decrypt_field(&encrypted_field)?;
```

### Security Best Practices

1. **Never commit master keys** to version control
2. **Use different keys** for different environments (dev, staging, prod)
3. **Rotate keys regularly** (every 90-180 days)
4. **Monitor key usage** and access patterns
5. **Use secure key distribution** mechanisms
6. **Implement key escrow** for disaster recovery
7. **Audit encryption/decryption** operations
8. **Use hardware security modules** for production environments

### Troubleshooting Encryption

**Error: "LAZABOT_MASTER_KEY environment variable not set"**
- Solution: Set the environment variable with a valid 64-character hex key

**Error: "Invalid master key format"**
- Solution: Ensure the key is exactly 64 hex characters (32 bytes)

**Error: "Decryption failed"**
- Solution: Verify the master key is correct and the encrypted data is valid

**Error: "Encryption manager not initialized"**
- Solution: Call `init_encryption()` before using encryption functions

## 🚀 CI/CD Pipeline

This project includes a comprehensive CI/CD pipeline with automated testing, security scanning, and deployment controls.

### Pipeline Overview

The CI/CD pipeline includes the following stages:

1. **Rust Build & Test** - Compiles and tests Rust code across multiple toolchains
2. **Node.js Playwright Lint & Test** - Lints and tests JavaScript/Node.js code
3. **Security Scan** - Runs vulnerability scanning and secret detection
4. **Docker Build & Test** - Builds and tests Docker containers
5. **Integration Tests** - Runs comprehensive integration tests
6. **Deploy to Staging** - Automatic deployment to staging environment
7. **Deploy to Production** - Manual approval required for production deployment

### Running Tests Locally

#### Rust Tests
```bash
# Run all Rust tests
cargo test --all-features

# Run with verbose output
cargo test --verbose --all-features

# Run specific test
cargo test test_name --verbose

# Run integration tests
cargo test --test '*' --verbose
```

#### Node.js Tests
```bash
# Install dependencies
npm ci

# Run Playwright tests
npm test

# Run integration tests
node scripts/test_full_integration.js

# Lint JavaScript code
npx eslint scripts/*.js
```

#### Docker Tests
```bash
# Build Docker image
docker build -t lazabot:test .

# Test Docker container
docker run --rm -d --name lazabot-test lazabot:test
docker logs lazabot-test
docker stop lazabot-test
```

### Security Scanning

The pipeline includes multiple security scanning tools:

- **Trivy** - Vulnerability scanner for containers and dependencies
- **CodeQL** - Static analysis for security vulnerabilities
- **TruffleHog** - Secret detection in code and commits
- **npm audit** - Node.js dependency vulnerability scanning

### Deployment Process

#### Staging Deployment
- Triggered automatically on pushes to `develop` branch
- Runs after all tests and security scans pass
- Includes smoke tests for basic functionality

#### Production Deployment
- Triggered automatically on pushes to `main` branch
- Requires manual approval from DevOps team
- Includes comprehensive health checks
- Monitored for performance and error rates

### Pull Request Process

All pull requests must pass the following checks:

1. **Security Review** - No secrets, proper input validation, secure coding practices
2. **Code Quality** - Rust clippy, ESLint, proper formatting
3. **Testing** - Unit tests, integration tests, manual testing
4. **Documentation** - Updated README, code comments, API documentation
5. **Performance** - No performance regressions, resource usage within limits

### Environment Configuration

The pipeline uses GitHub Environments for deployment control:

- **staging** - Automatic deployment with basic approval
- **production** - Manual approval required from authorized users

### Monitoring and Alerting

- **Build Status** - Real-time build and test status
- **Security Alerts** - Immediate notification of security issues
- **Deployment Status** - Track deployment success/failure
- **Performance Metrics** - Monitor application performance post-deployment

### Troubleshooting

#### Common Issues

1. **Rust Build Failures**
   - Check Rust version compatibility
   - Verify all dependencies are available
   - Run `cargo clean` and rebuild

2. **Node.js Test Failures**
   - Ensure Playwright browsers are installed
   - Check Node.js version compatibility
   - Verify test environment configuration

3. **Security Scan Failures**
   - Review and fix security vulnerabilities
   - Remove any hardcoded secrets
   - Update dependencies with known issues

4. **Docker Build Failures**
   - Check Dockerfile syntax
   - Verify base image availability
   - Review resource constraints

#### Getting Help

- Check the [Security Policy](.github/SECURITY.md) for security-related issues
- Review the [Pull Request Template](.github/pull_request_template.md) for PR guidelines
- Open an issue for bugs or feature requests
- Contact the DevOps team for deployment issues

