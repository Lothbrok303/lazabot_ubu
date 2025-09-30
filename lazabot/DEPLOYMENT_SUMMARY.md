# Lazabot Production Deployment - Complete Setup

## 🎉 Deployment Setup Complete!

Your Lazabot project is now fully configured for production deployment as a systemd service on Ubuntu.

## 📁 Files Created

### Core Deployment Files
- **`lazabot.service`** - Systemd service configuration
- **`scripts/deploy.sh`** - Production deployment script
- **`scripts/rollback.sh`** - Rollback script for failed deployments
- **`scripts/test-deployment.sh`** - Pre-deployment testing script
- **`DEPLOYMENT.md`** - Comprehensive deployment documentation

## 🚀 Quick Start

### 1. Test Deployment (Safe)
```bash
cd lazabot
./scripts/test-deployment.sh
```

### 2. Deploy to Production
```bash
sudo ./scripts/deploy.sh
```

### 3. Configure Environment
```bash
sudo nano /etc/lazabot/env
```

### 4. Check Service Status
```bash
sudo systemctl status lazabot
```

## 🔧 Service Management

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
```

### Rollback if Needed
```bash
sudo ./scripts/rollback.sh
```

## 📊 What Gets Deployed

### System Configuration
- **User:** `lazabot` (system user, no shell)
- **Binary:** `/usr/local/bin/lazabot`
- **Service:** `lazabot.service` (systemd)
- **Auto-start:** Enabled on boot

### Directory Structure
```
/opt/lazabot/                 # Main application directory
├── data/                     # Data storage
├── logs/                     # Log files
├── config/                   # Configuration files
└── examples/                 # Example files

/etc/lazabot/
└── env                       # Environment configuration

/opt/backups/lazabot/         # Backup storage
└── lazabot_backup_*.tar.gz   # Automatic backups

/usr/local/bin/
└── lazabot                   # Binary executable

/etc/systemd/system/
└── lazabot.service           # Systemd service file
```

## 🛡️ Security Features

- **Non-privileged user:** Runs as `lazabot` system user
- **Restricted filesystem:** `ProtectSystem=strict`
- **No new privileges:** `NoNewPrivileges=true`
- **Resource limits:** 2GB memory, 200% CPU quota
- **Secure permissions:** Proper file ownership and permissions

## 📈 Monitoring & Logs

### Service Health
```bash
# Check if running
sudo systemctl is-active lazabot

# Check for failures
sudo systemctl is-failed lazabot

# View recent logs
sudo journalctl -u lazabot --since "1 hour ago"
```

### Log Locations
- **Systemd logs:** `journalctl -u lazabot`
- **Application logs:** `/opt/lazabot/logs/` (if configured)
- **Service logs:** `/var/log/syslog` (grep for lazabot)

## 🔄 Backup & Recovery

### Automatic Backups
- Created before each deployment
- Stored in `/opt/backups/lazabot/`
- Format: `lazabot_backup_YYYYMMDD_HHMMSS.tar.gz`

### Manual Backup
```bash
sudo tar -czf /opt/backups/lazabot/manual_backup_$(date +%Y%m%d_%H%M%S).tar.gz -C /opt lazabot
```

### Recovery
```bash
sudo ./scripts/rollback.sh
```

## ⚙️ Configuration

### Environment File
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

### Service Configuration
The service includes:
- **Auto-restart:** Restarts on failure (max 5 times per 5 minutes)
- **Resource limits:** Memory and CPU limits enforced
- **Security:** Restricted permissions and filesystem access
- **Logging:** Integrated with systemd journal

## 🧪 Testing

### Pre-deployment Test
```bash
./scripts/test-deployment.sh
```

### Post-deployment Verification
```bash
# Check service status
sudo systemctl status lazabot

# Check logs
sudo journalctl -u lazabot --no-pager -n 20

# Test binary execution
/usr/local/bin/lazabot --help
```

## 📚 Documentation

- **`DEPLOYMENT.md`** - Complete deployment guide
- **`README.md`** - Project documentation
- **Service logs** - Runtime information

## 🆘 Troubleshooting

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

## ✅ Deployment Checklist

- [x] Binary builds successfully
- [x] Service file syntax valid
- [x] Deployment script ready
- [x] Rollback script ready
- [x] Test script working
- [x] Documentation complete
- [x] Security configured
- [x] Backup system ready

## 🎯 Next Steps

1. **Configure environment:** Edit `/etc/lazabot/env`
2. **Deploy:** Run `sudo ./scripts/deploy.sh`
3. **Monitor:** Check service status and logs
4. **Test:** Verify functionality
5. **Maintain:** Regular monitoring and updates

---

**Ready for production deployment!** 🚀

For detailed information, see `DEPLOYMENT.md`.
