# Lazabot Production Deployment Guide

This guide covers deploying Lazabot as a production systemd service on Ubuntu.

## Prerequisites

- Ubuntu 18.04+ (tested on Ubuntu 20.04+)
- Rust toolchain installed
- Root/sudo access
- Network connectivity

## Quick Start

1. **Build the project:**
   ```bash
   cargo build --release
   ```

2. **Deploy to production:**
   ```bash
   sudo ./scripts/deploy.sh
   ```

3. **Configure environment:**
   ```bash
   sudo nano /etc/lazabot/env
   ```

4. **Check service status:**
   ```bash
   sudo systemctl status lazabot
   ```

## Deployment Details

### What the deployment script does:

1. **Creates system user:** `lazabot` (system user, no shell)
2. **Installs binary:** `/usr/local/bin/lazabot`
3. **Creates directories:**
   - `/opt/lazabot/` - Main application directory
   - `/opt/lazabot/data/` - Data storage
   - `/opt/lazabot/logs/` - Log files
   - `/opt/lazabot/config/` - Configuration files
4. **Installs systemd service:** `/etc/systemd/system/lazabot.service`
5. **Creates environment file:** `/etc/lazabot/env`
6. **Enables auto-start:** Service starts on boot
7. **Creates backup:** Previous installation backed up

### Service Configuration

The systemd service includes:

- **Auto-restart:** Restarts on failure (max 5 times per 5 minutes)
- **Security:** Runs as non-privileged user with restricted permissions
- **Resource limits:** 2GB memory, 200% CPU quota
- **Logging:** Integrated with systemd journal
- **Environment:** Loads from `/etc/lazabot/env`

### Environment Configuration

Edit `/etc/lazabot/env` to configure:

```bash
# Application settings
LAZABOT_CONFIG_PATH=/opt/lazabot/config
LAZABOT_DATA_PATH=/opt/lazabot/data
LAZABOT_LOG_PATH=/opt/lazabot/logs

# Add your specific configuration here
# API_KEY=your_api_key_here
# DATABASE_URL=sqlite:///opt/lazabot/data/lazabot.db
```

## Service Management

### Basic Commands

```bash
# Start service
sudo systemctl start lazabot

# Stop service
sudo systemctl stop lazabot

# Restart service
sudo systemctl restart lazabot

# Check status
sudo systemctl status lazabot

# View logs
sudo journalctl -u lazabot -f

# View recent logs
sudo journalctl -u lazabot --since "1 hour ago"
```

### Service Information

- **Service name:** `lazabot`
- **User:** `lazabot` (system user)
- **Working directory:** `/opt/lazabot`
- **Binary:** `/usr/local/bin/lazabot`
- **Configuration:** `/opt/lazabot/config/`
- **Data:** `/opt/lazabot/data/`
- **Logs:** `/opt/lazabot/logs/` + systemd journal
- **Environment:** `/etc/lazabot/env`

## Rollback Process

### Automatic Rollback

```bash
sudo ./scripts/rollback.sh
```

The rollback script:
1. Lists available backups
2. Shows backup details
3. Asks for confirmation
4. Stops current service
5. Creates pre-rollback backup
6. Restores from selected backup
7. Restarts service
8. Verifies service health

### Manual Rollback

1. **Stop service:**
   ```bash
   sudo systemctl stop lazabot
   ```

2. **Restore from backup:**
   ```bash
   cd /opt
   sudo tar -xzf /opt/backups/lazabot/lazabot_backup_YYYYMMDD_HHMMSS.tar.gz
   ```

3. **Restart service:**
   ```bash
   sudo systemctl start lazabot
   ```

### Backup Management

Backups are stored in `/opt/backups/lazabot/`:

- **Format:** `lazabot_backup_YYYYMMDD_HHMMSS.tar.gz`
- **Contents:** Complete `/opt/lazabot` directory
- **Retention:** Manual cleanup required
- **Size:** Typically 50-200MB per backup

## Monitoring and Troubleshooting

### Health Checks

```bash
# Check service status
sudo systemctl is-active lazabot

# Check service health
sudo systemctl is-failed lazabot

# View service logs
sudo journalctl -u lazabot --since "1 hour ago" --no-pager
```

### Common Issues

1. **Service won't start:**
   ```bash
   sudo journalctl -u lazabot --no-pager -n 50
   ```

2. **Permission issues:**
   ```bash
   sudo chown -R lazabot:lazabot /opt/lazabot
   ```

3. **Configuration errors:**
   ```bash
   sudo nano /etc/lazabot/env
   sudo systemctl restart lazabot
   ```

4. **Resource limits:**
   ```bash
   sudo systemctl show lazabot --property=MemoryCurrent,CPUUsageNSec
   ```

### Log Locations

- **Systemd logs:** `journalctl -u lazabot`
- **Application logs:** `/opt/lazabot/logs/` (if configured)
- **Service logs:** `/var/log/syslog` (grep for lazabot)

## Security Considerations

### File Permissions

- **Binary:** `root:root` (755)
- **Service file:** `root:root` (644)
- **Environment file:** `root:root` (600)
- **Data directory:** `lazabot:lazabot` (755)
- **Config directory:** `lazabot:lazabot` (755)

### Security Features

- **Non-privileged user:** Runs as `lazabot` system user
- **Restricted filesystem:** `ProtectSystem=strict`
- **No new privileges:** `NoNewPrivileges=true`
- **Private temp:** `PrivateTmp=true`
- **Resource limits:** Memory and CPU limits enforced

## Updating the Service

### Update Process

1. **Build new version:**
   ```bash
   cargo build --release
   ```

2. **Deploy update:**
   ```bash
   sudo ./scripts/deploy.sh
   ```

3. **Verify deployment:**
   ```bash
   sudo systemctl status lazabot
   ```

### Rollback if Needed

```bash
sudo ./scripts/rollback.sh
```

## Configuration Examples

### Basic Configuration

```bash
# /etc/lazabot/env
LAZABOT_CONFIG_PATH=/opt/lazabot/config
LAZABOT_DATA_PATH=/opt/lazabot/data
LAZABOT_LOG_PATH=/opt/lazabot/logs
RUST_LOG=info
RUST_ENV=production
```

### Advanced Configuration

```bash
# /etc/lazabot/env
LAZABOT_CONFIG_PATH=/opt/lazabot/config
LAZABOT_DATA_PATH=/opt/lazabot/data
LAZABOT_LOG_PATH=/opt/lazabot/logs
RUST_LOG=info
RUST_ENV=production

# API Configuration
API_KEY=your_production_api_key
API_SECRET=your_production_api_secret
API_BASE_URL=https://api.lazada.com

# Database Configuration
DATABASE_URL=sqlite:///opt/lazabot/data/lazabot.db

# Proxy Configuration
PROXY_LIST=/opt/lazabot/config/proxies.txt
PROXY_ROTATION=true

# Monitoring Configuration
METRICS_ENABLED=true
METRICS_PORT=8080
HEALTH_CHECK_INTERVAL=30

# Security Configuration
ENCRYPTION_KEY=your_32_byte_encryption_key
SESSION_SECRET=your_session_secret
```

## Maintenance

### Regular Maintenance

1. **Monitor logs:** Check for errors and warnings
2. **Check disk space:** Ensure adequate space for data and logs
3. **Update configuration:** As needed for your environment
4. **Clean old backups:** Remove old backup files periodically
5. **Monitor resources:** Check memory and CPU usage

### Backup Strategy

- **Automatic backups:** Created before each deployment
- **Manual backups:** Create before major changes
- **Backup retention:** Implement retention policy
- **Backup testing:** Periodically test restore process

## Support

For issues or questions:

1. Check service logs: `sudo journalctl -u lazabot -f`
2. Check service status: `sudo systemctl status lazabot`
3. Verify configuration: `sudo cat /etc/lazabot/env`
4. Check file permissions: `ls -la /opt/lazabot/`

## File Structure

```
/opt/lazabot/                 # Main application directory
├── data/                     # Data storage
├── logs/                     # Log files
├── config/                   # Configuration files
└── examples/                 # Example files

/etc/lazabot/
└── env                       # Environment configuration

/opt/backups/lazabot/         # Backup storage
└── lazabot_backup_*.tar.gz   # Backup files

/usr/local/bin/
└── lazabot                   # Binary executable

/etc/systemd/system/
└── lazabot.service           # Systemd service file
```

