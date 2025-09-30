# Lazabot - Lazada CLI Bot

A powerful Rust-based CLI bot for automating Lazada operations with Playwright integration.

## Ì∫Ä Features

- **Rust CLI**: High-performance command-line interface
- **Playwright Integration**: Browser automation with stealth capabilities
- **Proxy Support**: Rotating proxy management with health checks
- **Session Management**: Persistent session handling with encryption
- **Task Management**: Concurrent task execution with monitoring
- **Storage**: SQLite database with caching
- **Security**: Encrypted data storage and secure communication
- **Monitoring**: Real-time performance and health monitoring

## Ì≥¶ Quick Start

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

## Ì∞≥ Docker Deployment

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

## Ì∂•Ô∏è Ubuntu Server Deployment

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

## Ì¥ß Configuration

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

## ÌøóÔ∏è Architecture

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

## Ì≥ä Monitoring

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

## Ì¥í Security

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

## Ì∑™ Testing

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

### Test Coverage

- Unit tests for all modules
- Integration tests with mock servers
- API client tests (11/11 passing)
- Deployment configuration tests
- Security configuration tests

## ÔøΩÔøΩ Documentation

- [Deployment Guide](DEPLOYMENT.md) - Complete deployment documentation
- [Docker Setup](docker-compose.yml) - Docker configuration
- [API Reference](docs/api.md) - API documentation
- [Configuration](docs/config.md) - Configuration options

## Ì∫Ä Deployment Options

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

## Ì¥ß Troubleshooting

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

## Ì≥à Performance

### System Requirements

- **Minimum**: 2GB RAM, 10GB disk
- **Recommended**: 4GB RAM, 20GB disk
- **CPU**: 2+ cores

### Optimization

- File descriptor limits
- Memory monitoring
- Disk space management
- Resource usage tracking

## Ì¥ù Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Ì≥Ñ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ÔøΩÔøΩ Support

For issues and questions:

1. Check the [troubleshooting guide](DEPLOYMENT.md#troubleshooting)
2. Review the [documentation](DEPLOYMENT.md)
3. Check [GitHub issues](https://github.com/Lothbrok303/lazabot/issues)
4. Create a new issue with detailed information

## ÌæØ Roadmap

- [ ] Web dashboard for monitoring
- [ ] Advanced proxy rotation strategies
- [ ] Machine learning for captcha solving
- [ ] Multi-platform support
- [ ] API rate limiting improvements
- [ ] Enhanced security features

---

**Ready to deploy!** Ì∫Ä

Choose your deployment method and follow the setup instructions. For production deployments, use the Ubuntu server setup with proper security configuration.
