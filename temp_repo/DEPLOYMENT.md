# Lazabot Deployment Guide

This guide provides comprehensive instructions for deploying the Lazabot CLI application on Ubuntu servers.

## Ì∫Ä Quick Start

### Option 1: Automated Setup (Recommended)

```bash
# 1. Run setup script on Ubuntu server
curl -fsSL https://raw.githubusercontent.com/your-repo/lazabot/main/scripts/setup.sh | bash

# 2. Deploy application
REMOTE_HOST=192.168.1.56 ./scripts/deploy_remote.sh

# 3. Test deployment
REMOTE_HOST=192.168.1.56 ./scripts/test_deployment.sh
```

### Option 2: Manual Setup

```bash
# 1. SSH to your Ubuntu server
ssh ubuntu@192.168.1.56

# 2. Run setup script
curl -fsSL https://raw.githubusercontent.com/your-repo/lazabot/main/scripts/setup.sh | bash

# 3. Copy source code
sudo cp -r . /tmp/lazabot-source/

# 4. Deploy
sudo /opt/lazabot/scripts/deploy.sh

# 5. Configure environment
sudo nano /opt/lazabot/config/.env

# 6. Start services
sudo systemctl start lazabot lazabot-playwright
sudo systemctl enable lazabot lazabot-playwright
```

## Ì≥ã Prerequisites

- Ubuntu 20.04+ (tested on 22.04 LTS)
- Root or sudo access
- Internet connection
- At least 2GB RAM
- At least 10GB free disk space

## Ì¥ß What the Setup Script Does

The `scripts/setup.sh` script automatically:

1. **Updates system packages**
2. **Installs dependencies:**
   - Build tools (gcc, make, pkg-config)
   - Rust toolchain
   - Node.js LTS
   - Playwright system dependencies
   - Security tools

3. **Creates application structure:**
   ```
   /opt/lazabot/
   ‚îú‚îÄ‚îÄ bin/           # Rust binary
   ‚îú‚îÄ‚îÄ config/        # Configuration files
   ‚îú‚îÄ‚îÄ data/          # Database and data files
   ‚îú‚îÄ‚îÄ logs/          # Application logs
   ‚îú‚îÄ‚îÄ scripts/       # Utility scripts
   ‚îî‚îÄ‚îÄ backups/       # Backup files
   ```

4. **Sets up systemd services:**
   - `lazabot.service` - Main application
   - `lazabot-playwright.service` - Playwright server

5. **Configures security:**
   - Creates dedicated `lazabot` user
   - Sets proper file permissions
   - Configures firewall rules
   - Sets up log rotation

6. **Creates utility scripts:**
   - Health monitoring
   - Automated backups
   - Deployment automation

## Ì∞≥ Docker Deployment

### Build and Run

```bash
# Build the image
docker build -t lazabot:latest .

# Run with Docker Compose
docker-compose up -d

# Or run directly
docker run -d \
  --name lazabot \
  --restart unless-stopped \
  -p 8081:8081 \
  -v $(pwd)/data:/opt/lazabot/data \
  -v $(pwd)/logs:/opt/lazabot/logs \
  -v $(pwd)/config:/opt/lazabot/config \
  lazabot:latest
```

### Docker Compose

The included `docker-compose.yml` provides:

- Multi-stage build optimization
- Volume persistence
- Health checks
- Optional nginx reverse proxy
- Network isolation

## Ì¥ê Security Configuration

### 1. Generate Secure Keys

```bash
# Generate encryption key
openssl rand -hex 32

# Generate session secret
openssl rand -hex 32

# Update /opt/lazabot/config/.env
ENCRYPTION_KEY=your-generated-key-here
SESSION_SECRET=your-generated-secret-here
```

### 2. Firewall Setup

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 8081/tcp  # Playwright server
sudo ufw enable
```

### 3. SSL/TLS (Production)

For production deployments, use a reverse proxy:

```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Ì≥ä Monitoring and Maintenance

### Health Checks

```bash
# Manual health check
sudo -u lazabot /opt/lazabot/scripts/health_check.sh

# Check service status
sudo systemctl status lazabot lazabot-playwright

# Check health endpoint
curl http://localhost:8081/health
```

### Log Management

```bash
# View real-time logs
sudo journalctl -u lazabot -f

# View Playwright logs
sudo journalctl -u lazabot-playwright -f

# Check log files
tail -f /opt/lazabot/logs/*.log
```

### Backup and Recovery

```bash
# Create backup
sudo -u lazabot /opt/lazabot/scripts/backup.sh

# List backups
ls -la /opt/lazabot/backups/

# Restore from backup
sudo -u lazabot tar -xzf /opt/lazabot/backups/lazabot_backup_YYYYMMDD_HHMMSS.tar.gz -C /opt/lazabot/
```

## Ì¥Ñ Updates and Maintenance

### Update Application

```bash
# Stop services
sudo systemctl stop lazabot lazabot-playwright

# Deploy new version
sudo /opt/lazabot/scripts/deploy.sh

# Start services
sudo systemctl start lazabot lazabot-playwright
```

### System Updates

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Restart services if needed
sudo systemctl restart lazabot lazabot-playwright
```

## Ì∞õ Troubleshooting

### Common Issues

1. **Service won't start**
   ```bash
   sudo journalctl -u lazabot -n 50
   sudo systemctl status lazabot
   ```

2. **Permission denied**
   ```bash
   sudo chown -R lazabot:lazabot /opt/lazabot
   sudo chmod +x /opt/lazabot/bin/lazabot
   ```

3. **Port already in use**
   ```bash
   sudo netstat -tlnp | grep :8081
   sudo lsof -i :8081
   ```

4. **Playwright browser issues**
   ```bash
   sudo -u lazabot npx playwright install chromium
   ```

### Debug Mode

```bash
# Enable debug logging
echo "LAZABOT_LOG_LEVEL=debug" | sudo tee -a /opt/lazabot/config/.env
sudo systemctl restart lazabot

# Run in debug mode
sudo -u lazabot RUST_LOG=debug /opt/lazabot/bin/lazabot
```

## Ì≥à Performance Tuning

### System Limits

```bash
# Increase file descriptor limits
echo "lazabot soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "lazabot hard nofile 65536" | sudo tee -a /etc/security/limits.conf
```

### Memory Optimization

```bash
# Monitor memory usage
htop
free -h

# Check for memory leaks
sudo journalctl -u lazabot | grep -i memory
```

## Ì¥ç Testing Deployment

### Automated Testing

```bash
# Test remote deployment
REMOTE_HOST=192.168.1.56 ./scripts/test_deployment.sh

# Test health endpoint
curl http://192.168.1.56:8081/health

# Test Playwright functionality
curl -X POST http://192.168.1.56:8081/api/browser/launch
```

### Manual Testing

```bash
# SSH to server
ssh ubuntu@192.168.1.56

# Check services
sudo systemctl status lazabot lazabot-playwright

# Check logs
sudo journalctl -u lazabot -f

# Test health
curl http://localhost:8081/health
```

## Ì≥ö Additional Resources

- [Systemd Service Management](https://www.freedesktop.org/software/systemd/man/systemd.service.html)
- [Docker Compose Reference](https://docs.docker.com/compose/compose-file/)
- [Playwright Documentation](https://playwright.dev/)
- [Rust Deployment Guide](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)

## Ì∂ò Support

If you encounter issues:

1. Check the logs: `sudo journalctl -u lazabot -f`
2. Run health check: `sudo -u lazabot /opt/lazabot/scripts/health_check.sh`
3. Check service status: `sudo systemctl status lazabot lazabot-playwright`
4. Review this documentation
5. Check GitHub issues

## Ì≥ù Environment Variables Reference

### Required Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `LAZABOT_LOG_LEVEL` | Logging level | `info` | `debug`, `info`, `warn`, `error` |
| `LAZABOT_DATA_DIR` | Data directory | `/opt/lazabot/data` | `/opt/lazabot/data` |
| `LAZABOT_LOG_DIR` | Log directory | `/opt/lazabot/logs` | `/opt/lazabot/logs` |
| `PLAYWRIGHT_SERVER_PORT` | Playwright server port | `8081` | `8081` |
| `PLAYWRIGHT_HEADLESS` | Run browser headless | `true` | `true`, `false` |
| `DATABASE_URL` | Database connection string | `sqlite:///opt/lazabot/data/lazabot.db` | `sqlite:///opt/lazabot/data/lazabot.db` |
| `ENCRYPTION_KEY` | 32-character encryption key | **REQUIRED** | `a1b2c3d4e5f6...` |
| `SESSION_SECRET` | Session secret key | **REQUIRED** | `x1y2z3a4b5c6...` |

### Optional Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `PROXY_HOST` | Proxy server host | - | `proxy.example.com` |
| `PROXY_PORT` | Proxy server port | - | `8080` |
| `PROXY_USERNAME` | Proxy username | - | `username` |
| `PROXY_PASSWORD` | Proxy password | - | `password` |
| `API_TIMEOUT` | API request timeout (seconds) | `30` | `30` |
| `API_RETRY_ATTEMPTS` | Number of retry attempts | `3` | `3` |
| `API_RETRY_DELAY` | Delay between retries (ms) | `1000` | `1000` |
| `RATE_LIMIT_REQUESTS_PER_MINUTE` | Rate limit per minute | `60` | `60` |
| `RATE_LIMIT_BURST` | Burst rate limit | `10` | `10` |
