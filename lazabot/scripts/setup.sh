#!/bin/bash

# Lazabot Ubuntu Setup Script
# This script sets up Lazabot on a fresh Ubuntu server

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
LAZABOT_USER="lazabot"
LAZABOT_HOME="/opt/lazabot"
LAZABOT_PORT="8081"

log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
    error "This script should not be run as root. Please run as a regular user with sudo privileges."
fi

# Check sudo access
if ! sudo -n true 2>/dev/null; then
    error "This script requires sudo privileges. Please ensure your user can run sudo commands."
fi

log "Starting Lazabot Ubuntu Setup..."

# Update system packages
log "Updating system packages..."
sudo apt update && sudo apt upgrade -y
success "System packages updated"

# Install essential build tools
log "Installing build tools and dependencies..."
sudo apt install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    gnupg \
    lsb-release \
    software-properties-common \
    apt-transport-https \
    unzip \
    jq \
    net-tools \
    procps
success "Build tools installed"

# Install Rust
log "Installing Rust toolchain..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    rustup default stable
    success "Rust installed: $(rustc --version)"
else
    success "Rust already installed: $(rustc --version)"
fi

# Install Node.js LTS
log "Installing Node.js LTS..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
    sudo apt install -y nodejs
    success "Node.js installed: $(node --version)"
else
    success "Node.js already installed: $(node --version)"
fi

# Install Playwright system dependencies
log "Installing Playwright system dependencies..."
sudo apt install -y \
    libnss3 \
    libatk-bridge2.0-0 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libxss1 \
    libasound2
success "Playwright dependencies installed"

# Create lazabot user
log "Creating lazabot user..."
if ! id "$LAZABOT_USER" &>/dev/null; then
    sudo useradd -r -s /bin/bash -d "$LAZABOT_HOME" -m "$LAZABOT_USER"
    success "User $LAZABOT_USER created"
else
    success "User $LAZABOT_USER already exists"
fi

# Create application directory structure
log "Creating application directory structure..."
sudo mkdir -p "$LAZABOT_HOME"/{bin,config,data,logs,scripts}
sudo chown -R "$LAZABOT_USER:$LAZABOT_USER" "$LAZABOT_HOME"
success "Directory structure created"

# Create environment configuration
log "Creating environment configuration..."
sudo tee "$LAZABOT_HOME/config/.env" > /dev/null <<ENVEOF
# Lazabot Configuration
# Update these values with your actual settings

# API Configuration
LAZADA_API_KEY=your_api_key_here
LAZADA_API_SECRET=your_api_secret_here
LAZADA_API_ENDPOINT=https://api.lazada.com.my/rest

# Application Settings
LAZABOT_PORT=$LAZABOT_PORT
LAZABOT_LOG_LEVEL=info
LAZABOT_DATA_DIR=$LAZABOT_HOME/data
LAZABOT_LOG_DIR=$LAZABOT_HOME/logs

# Security Settings
LAZABOT_SECRET_KEY=your_secret_key_here_change_this
LAZABOT_JWT_SECRET=your_jwt_secret_here_change_this

# Database (if using)
DATABASE_URL=sqlite:///$LAZABOT_HOME/data/lazabot.db

# Proxy Settings (optional)
# HTTP_PROXY=http://proxy.example.com:8080
# HTTPS_PROXY=http://proxy.example.com:8080
# NO_PROXY=localhost,127.0.0.1
ENVEOF

sudo chown "$LAZABOT_USER:$LAZABOT_USER" "$LAZABOT_HOME/config/.env"
sudo chmod 600 "$LAZABOT_HOME/config/.env"
success "Environment configuration created"

# Create placeholder binary (will be replaced by actual Rust binary)
log "Creating placeholder binary..."
sudo tee "$LAZABOT_HOME/bin/lazabot" > /dev/null <<'BINEOF'
#!/bin/bash
# Placeholder binary - replace with actual Rust binary after compilation

echo "Lazabot placeholder binary"
echo "To build the actual binary, run: cargo build --release"
echo "Then copy: sudo cp target/release/lazabot /opt/lazabot/bin/"

# Keep the service running
while true; do
    sleep 30
done
BINEOF

sudo chmod +x "$LAZABOT_HOME/bin/lazabot"
sudo chown "$LAZABOT_USER:$LAZABOT_USER" "$LAZABOT_HOME/bin/lazabot"
success "Placeholder binary created"

# Create Playwright server script
log "Creating Playwright server script..."
sudo tee "$LAZABOT_HOME/scripts/playwright_server.js" > /dev/null <<'JSEOF'
const express = require('express');
const cors = require('cors');
const path = require('path');

const app = express();
const PORT = process.env.LAZABOT_PORT || 8081;

// Middleware
app.use(cors());
app.use(express.json());

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'ok',
        timestamp: new Date().toISOString(),
        message: 'Lazabot Playwright server is running'
    });
});

// API endpoints (placeholder)
app.get('/api/status', (req, res) => {
    res.json({
        service: 'playwright',
        status: 'running',
        timestamp: new Date().toISOString()
    });
});

// Error handling
app.use((err, req, res, next) => {
    console.error('Error:', err);
    res.status(500).json({ error: 'Internal server error' });
});

// Start server
app.listen(PORT, '0.0.0.0', () => {
    console.log(`Playwright server running on port ${PORT}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('Received SIGTERM, shutting down gracefully');
    process.exit(0);
});

process.on('SIGINT', () => {
    console.log('Received SIGINT, shutting down gracefully');
    process.exit(0);
});
JSEOF

sudo chown "$LAZABOT_USER:$LAZABOT_USER" "$LAZABOT_HOME/scripts/playwright_server.js"
success "Playwright server script created"

# Install Node.js dependencies
log "Installing Node.js dependencies..."
sudo -u "$LAZABOT_USER" bash -c "cd '$LAZABOT_HOME' && npm init -y"
sudo -u "$LAZABOT_USER" bash -c "cd '$LAZABOT_HOME' && npm install express cors"
success "Node.js dependencies installed"

# Create systemd service files
log "Creating systemd service files..."

# Main lazabot service
sudo tee /etc/systemd/system/lazabot.service > /dev/null <<SERVICEEOF
[Unit]
Description=Lazabot Main Application
After=network.target
Wants=network.target

[Service]
Type=simple
User=$LAZABOT_USER
Group=$LAZABOT_USER
WorkingDirectory=$LAZABOT_HOME
ExecStart=$LAZABOT_HOME/bin/lazabot
Restart=always
RestartSec=10
EnvironmentFile=$LAZABOT_HOME/config/.env

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$LAZABOT_HOME/data $LAZABOT_HOME/logs

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=lazabot

[Install]
WantedBy=multi-user.target
SERVICEEOF

# Playwright service
sudo tee /etc/systemd/system/lazabot-playwright.service > /dev/null <<PLAYWRIGHTEOF
[Unit]
Description=Lazabot Playwright Server
After=network.target
Wants=network.target

[Service]
Type=simple
User=$LAZABOT_USER
Group=$LAZABOT_USER
WorkingDirectory=$LAZABOT_HOME
ExecStart=/usr/bin/node $LAZABOT_HOME/scripts/playwright_server.js
Restart=always
RestartSec=10
EnvironmentFile=$LAZABOT_HOME/config/.env

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$LAZABOT_HOME/data $LAZABOT_HOME/logs

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=lazabot-playwright

[Install]
WantedBy=multi-user.target
PLAYWRIGHTEOF

success "Systemd service files created"

# Reload systemd and enable services
log "Enabling systemd services..."
sudo systemctl daemon-reload
sudo systemctl enable lazabot.service
sudo systemctl enable lazabot-playwright.service
success "Services enabled"

# Start services
log "Starting services..."
sudo systemctl start lazabot.service
sudo systemctl start lazabot-playwright.service
success "Services started"

# Wait for services to start
sleep 5

# Verify services are running
log "Verifying services..."
if systemctl is-active --quiet lazabot.service; then
    success "Lazabot service is running"
else
    warning "Lazabot service failed to start"
fi

if systemctl is-active --quiet lazabot-playwright.service; then
    success "Playwright service is running"
else
    warning "Playwright service failed to start"
fi

# Test health endpoint
log "Testing health endpoint..."
if curl -s http://localhost:$LAZABOT_PORT/health > /dev/null; then
    success "Health endpoint is responding"
else
    warning "Health endpoint is not responding"
fi

# Create log rotation configuration
log "Setting up log rotation..."
sudo tee /etc/logrotate.d/lazabot > /dev/null <<LOGROTATEEOF
$LAZABOT_HOME/logs/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 $LAZABOT_USER $LAZABOT_USER
    postrotate
        systemctl reload lazabot.service lazabot-playwright.service
    endscript
}
LOGROTATEEOF
success "Log rotation configured"

# Create management script
log "Creating management script..."
sudo tee "$LAZABOT_HOME/bin/manage" > /dev/null <<'MANAGEEOF'
#!/bin/bash
# Lazabot Management Script

case "$1" in
    status)
        echo "=== Lazabot Service Status ==="
        systemctl status lazabot.service --no-pager
        echo
        echo "=== Playwright Service Status ==="
        systemctl status lazabot-playwright.service --no-pager
        ;;
    restart)
        echo "Restarting Lazabot services..."
        sudo systemctl restart lazabot.service
        sudo systemctl restart lazabot-playwright.service
        echo "Services restarted"
        ;;
    logs)
        echo "=== Lazabot Logs ==="
        sudo journalctl -u lazabot.service -f
        ;;
    logs-playwright)
        echo "=== Playwright Logs ==="
        sudo journalctl -u lazabot-playwright.service -f
        ;;
    health)
        curl -s http://localhost:8081/health | jq .
        ;;
    *)
        echo "Usage: $0 {status|restart|logs|logs-playwright|health}"
        exit 1
        ;;
esac
MANAGEEOF

sudo chmod +x "$LAZABOT_HOME/bin/manage"
sudo chown "$LAZABOT_USER:$LAZABOT_USER" "$LAZABOT_HOME/bin/manage"
success "Management script created"

# Final verification
log "Running final verification..."
echo
echo "=== Service Status ==="
sudo systemctl is-active lazabot.service lazabot-playwright.service
echo
echo "=== Health Check ==="
curl -s http://localhost:$LAZABOT_PORT/health | jq . || echo "Health check failed"
echo
echo "=== Port Status ==="
netstat -tlnp | grep ":$LAZABOT_PORT" || echo "Port $LAZABOT_PORT not listening"

echo
success "Lazabot setup completed successfully!"
echo
echo "Next steps:"
echo "1. Update configuration: sudo nano $LAZABOT_HOME/config/.env"
echo "2. Build Rust binary: cargo build --release"
echo "3. Install binary: sudo cp target/release/lazabot $LAZABOT_HOME/bin/"
echo "4. Restart services: sudo systemctl restart lazabot.service"
echo
echo "Management commands:"
echo "- Check status: $LAZABOT_HOME/bin/manage status"
echo "- View logs: $LAZABOT_HOME/bin/manage logs"
echo "- Health check: $LAZABOT_HOME/bin/manage health"
echo
echo "Service management:"
echo "- Start: sudo systemctl start lazabot.service lazabot-playwright.service"
echo "- Stop: sudo systemctl stop lazabot.service lazabot-playwright.service"
echo "- Restart: sudo systemctl restart lazabot.service lazabot-playwright.service"
echo "- Status: sudo systemctl status lazabot.service lazabot-playwright.service"
