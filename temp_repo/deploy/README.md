# Lazabot Deployment Guide

This guide covers deploying Lazabot on Ubuntu servers using both traditional systemd services and Docker containers.

## Prerequisites

- Ubuntu 20.04+ server
- User with sudo privileges
- Internet connectivity

## Method 1: Automated Setup Script

### Quick Deployment

```bash
# Download and run the setup script
curl -fsSL https://raw.githubusercontent.com/your-repo/lazabot/main/scripts/setup.sh | bash
```

### Manual Setup

```bash
# Clone the repository
git clone https://github.com/your-repo/lazabot.git
cd lazabot

# Make setup script executable
chmod +x scripts/setup.sh

# Run setup script
./scripts/setup.sh
```

### What the Setup Script Does

1. **System Updates**: Updates all system packages
2. **Dependencies**: Installs Rust, Node.js, and build tools
3. **User Creation**: Creates dedicated `lazabot` user
4. **Directory Structure**: Sets up `/opt/lazabot/` with proper permissions
5. **Configuration**: Creates environment file with safe defaults
6. **Services**: Creates and enables systemd services
7. **Health Monitoring**: Sets up health checks and log rotation

## Method 2: Docker Deployment

### Build Docker Image

```bash
# Build the multi-stage Docker image
docker build -t lazabot:latest .

# Or build with specific tag
docker build -t lazabot:v1.0.0 .
```

### Run with Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  lazabot:
    image: lazabot:latest
    container_name: lazabot
    restart: unless-stopped
    ports:
      - "8081:8081"
    environment:
      - LAZADA_API_KEY=${LAZADA_API_KEY}
      - LAZADA_API_SECRET=${LAZADA_API_SECRET}
      - LAZABOT_PORT=8081
    volumes:
      - lazabot_data:/opt/lazabot/data
      - lazabot_logs:/opt/lazabot/logs
    env_file:
      - .env

volumes:
  lazabot_data:
  lazabot_logs:
```

Run with Docker Compose:

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Run with Docker Run

```bash
# Create data directories
sudo mkdir -p /opt/lazabot/{data,logs}
sudo chown -R 1000:1000 /opt/lazabot

# Run container
docker run -d \
  --name lazabot \
  --restart unless-stopped \
  -p 8081:8081 \
  -v /opt/lazabot/data:/opt/lazabot/data \
  -v /opt/lazabot/logs:/opt/lazabot/logs \
  -e LAZADA_API_KEY="your_api_key" \
  -e LAZADA_API_SECRET="your_api_secret" \
  lazabot:latest
```

## Configuration

### Environment Variables

Create `.env` file with your settings:

```bash
# API Configuration
LAZADA_API_KEY=your_actual_api_key
LAZADA_API_SECRET=your_actual_api_secret
LAZADA_API_ENDPOINT=https://api.lazada.com.my/rest

# Application Settings
LAZABOT_PORT=8081
LAZABOT_LOG_LEVEL=info
LAZABOT_DATA_DIR=/opt/lazabot/data
LAZABOT_LOG_DIR=/opt/lazabot/logs

# Security Settings (CHANGE THESE!)
LAZABOT_SECRET_KEY=your_very_secure_secret_key_here
LAZABOT_JWT_SECRET=your_very_secure_jwt_secret_here

# Database
DATABASE_URL=sqlite:////opt/lazabot/data/lazabot.db

# Proxy Settings (optional)
# HTTP_PROXY=http://proxy.example.com:8080
# HTTPS_PROXY=http://proxy.example.com:8080
# NO_PROXY=localhost,127.0.0.1
```

### Security Notes

⚠️ **IMPORTANT SECURITY CONSIDERATIONS:**

1. **Change Default Secrets**: Always change `LAZABOT_SECRET_KEY` and `LAZABOT_JWT_SECRET`
2. **File Permissions**: Environment file should be readable only by the lazabot user:
   ```bash
   sudo chmod 600 /opt/lazabot/config/.env
   sudo chown lazabot:lazabot /opt/lazabot/config/.env
   ```
3. **Firewall**: Configure firewall to restrict access to port 8081
4. **SSL/TLS**: Use reverse proxy (nginx/apache) with SSL certificates for production
5. **Updates**: Keep system and dependencies updated regularly

## Service Management

### Systemd Services

```bash
# Check service status
sudo systemctl status lazabot.service
sudo systemctl status lazabot-playwright.service

# Start services
sudo systemctl start lazabot.service lazabot-playwright.service

# Stop services
sudo systemctl stop lazabot.service lazabot-playwright.service

# Restart services
sudo systemctl restart lazabot.service lazabot-playwright.service

# Enable auto-start
sudo systemctl enable lazabot.service lazabot-playwright.service

# Disable auto-start
sudo systemctl disable lazabot.service lazabot-playwright.service
```

### Using Management Script

```bash
# Check status
/opt/lazabot/bin/manage status

# View logs
/opt/lazabot/bin/manage logs

# Health check
/opt/lazabot/bin/manage health

# Restart services
/opt/lazabot/bin/manage restart
```

### Docker Management

```bash
# Check container status
docker ps

# View logs
docker logs lazabot

# Restart container
docker restart lazabot

# Stop container
docker stop lazabot

# Remove container
docker rm lazabot
```

## Monitoring and Logs

### Health Checks

```bash
# Check health endpoint
curl http://localhost:8081/health

# Expected response:
{
  "status": "ok",
  "timestamp": "2025-09-30T15:31:51.528Z",
  "message": "Lazabot Playwright server is running"
}
```

### Log Locations

- **Systemd logs**: `sudo journalctl -u lazabot.service`
- **Application logs**: `/opt/lazabot/logs/`
- **Docker logs**: `docker logs lazabot`

### Log Rotation

Log rotation is automatically configured for systemd deployment:

```bash
# Check logrotate configuration
sudo cat /etc/logrotate.d/lazabot

# Test logrotate
sudo logrotate -d /etc/logrotate.d/lazabot
```

## Troubleshooting

### Common Issues

1. **Service won't start**:
   ```bash
   sudo journalctl -u lazabot.service -f
   ```

2. **Port already in use**:
   ```bash
   sudo netstat -tlnp | grep :8081
   sudo lsof -i :8081
   ```

3. **Permission issues**:
   ```bash
   sudo chown -R lazabot:lazabot /opt/lazabot
   ```

4. **Docker build fails**:
   ```bash
   docker system prune -a
   docker build --no-cache -t lazabot:latest .
   ```

### Debug Mode

Run in debug mode for troubleshooting:

```bash
# Systemd
sudo systemctl stop lazabot.service
sudo -u lazabot /opt/lazabot/bin/lazabot

# Docker
docker run -it --rm lazabot:latest /bin/bash
```

## Production Deployment

### Reverse Proxy Setup (Nginx)

```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://localhost:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### SSL with Let's Encrypt

```bash
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo crontab -e
# Add: 0 12 * * * /usr/bin/certbot renew --quiet
```

### Backup Strategy

```bash
# Create backup script
cat > /opt/lazabot/scripts/backup.sh << 'BACKUPEOF'
#!/bin/bash
BACKUP_DIR="/opt/backups/lazabot"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"
tar -czf "$BACKUP_DIR/lazabot_$DATE.tar.gz" \
    /opt/lazabot/data \
    /opt/lazabot/config \
    /opt/lazabot/logs

# Keep only last 7 days
find "$BACKUP_DIR" -name "lazabot_*.tar.gz" -mtime +7 -delete
BACKUPEOF

chmod +x /opt/lazabot/scripts/backup.sh

# Schedule daily backups
echo "0 2 * * * /opt/lazabot/scripts/backup.sh" | sudo crontab -
```

## Updates and Maintenance

### Updating Lazabot

```bash
# Stop services
sudo systemctl stop lazabot.service lazabot-playwright.service

# Backup current version
sudo cp /opt/lazabot/bin/lazabot /opt/lazabot/bin/lazabot.backup

# Build new version
cargo build --release

# Install new binary
sudo cp target/release/lazabot /opt/lazabot/bin/

# Start services
sudo systemctl start lazabot.service lazabot-playwright.service
```

### System Maintenance

```bash
# Update system packages
sudo apt update && sudo apt upgrade

# Clean up old logs
sudo find /opt/lazabot/logs -name "*.log" -mtime +30 -delete

# Check disk space
df -h /opt/lazabot

# Monitor resource usage
htop
```

## Support

For issues and support:

1. Check logs: `/opt/lazabot/logs/` and `sudo journalctl -u lazabot.service`
2. Verify configuration: `/opt/lazabot/config/.env`
3. Test health endpoint: `curl http://localhost:8081/health`
4. Check service status: `sudo systemctl status lazabot.service`

## Security Checklist

- [ ] Changed default secrets in `.env`
- [ ] Set proper file permissions (600 for .env)
- [ ] Configured firewall rules
- [ ] Set up SSL/TLS certificates
- [ ] Enabled log rotation
- [ ] Set up monitoring and alerts
- [ ] Created backup strategy
- [ ] Updated system packages
- [ ] Disabled unnecessary services
- [ ] Set up fail2ban or similar protection
