#!/bin/bash

# Production Deployment Script for Lazabot
# This script deploys the lazabot binary and systemd service to production

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
BINARY_NAME="lazabot"
SERVICE_NAME="lazabot"
INSTALL_DIR="/opt/lazabot"
BINARY_DIR="/usr/local/bin"
SERVICE_DIR="/etc/systemd/system"
ENV_FILE="/etc/lazabot/env"
USER_NAME="lazabot"
GROUP_NAME="lazabot"
BACKUP_DIR="/opt/backups/lazabot"

echo -e "${BLUE}🚀 Starting Lazabot Production Deployment...${NC}"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}❌ This script must be run as root (use sudo)${NC}"
    exit 1
fi

# Pre-deployment checks
echo -e "${YELLOW}🔍 Running pre-deployment checks...${NC}"

# Check if binary exists
if [ ! -f "target/release/${BINARY_NAME}" ]; then
    echo -e "${RED}❌ Binary not found. Please build the project first:${NC}"
    echo -e "${YELLOW}   cargo build --release${NC}"
    exit 1
fi

# Check if service file exists
if [ ! -f "${SERVICE_NAME}.service" ]; then
    echo -e "${RED}❌ Service file not found: ${SERVICE_NAME}.service${NC}"
    exit 1
fi

# Create backup of current installation
echo -e "${YELLOW}💾 Creating backup of current installation...${NC}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p "${BACKUP_DIR}"

if [ -d "${INSTALL_DIR}" ]; then
    echo "Creating backup: lazabot_backup_${TIMESTAMP}.tar.gz"
    tar -czf "${BACKUP_DIR}/lazabot_backup_${TIMESTAMP}.tar.gz" -C /opt lazabot
    echo -e "${GREEN}✅ Backup created: ${BACKUP_DIR}/lazabot_backup_${TIMESTAMP}.tar.gz${NC}"
fi

# Stop existing service if running
echo -e "${YELLOW}🛑 Stopping existing service...${NC}"
systemctl stop "${SERVICE_NAME}" 2>/dev/null || echo "Service not running"

# Create user and group if they don't exist
echo -e "${YELLOW}👤 Setting up user and group...${NC}"
if ! id "${USER_NAME}" &>/dev/null; then
    useradd --system --no-create-home --shell /bin/false "${USER_NAME}"
    echo -e "${GREEN}✅ Created user: ${USER_NAME}${NC}"
else
    echo -e "${YELLOW}ℹ️  User ${USER_NAME} already exists${NC}"
fi

# Create directories
echo -e "${YELLOW}📁 Creating directories...${NC}"
mkdir -p "${INSTALL_DIR}"/{data,logs,config}
mkdir -p "$(dirname "${ENV_FILE}")"
mkdir -p "${BACKUP_DIR}"

# Set permissions
chown -R "${USER_NAME}:${GROUP_NAME}" "${INSTALL_DIR}"
chmod 755 "${INSTALL_DIR}"
chmod 755 "${INSTALL_DIR}"/{data,logs,config}

# Install binary
echo -e "${YELLOW}📦 Installing binary...${NC}"
cp "target/release/${BINARY_NAME}" "${BINARY_DIR}/${BINARY_NAME}"
chmod +x "${BINARY_DIR}/${BINARY_NAME}"
chown root:root "${BINARY_DIR}/${BINARY_NAME}"

# Install service file
echo -e "${YELLOW}⚙️  Installing systemd service...${NC}"
cp "${SERVICE_NAME}.service" "${SERVICE_DIR}/${SERVICE_NAME}.service"
chmod 644 "${SERVICE_DIR}/${SERVICE_NAME}.service"

# Create environment file if it doesn't exist
if [ ! -f "${ENV_FILE}" ]; then
    echo -e "${YELLOW}📝 Creating environment file...${NC}"
    cat > "${ENV_FILE}" << 'ENV_EOF'
# Lazabot Environment Configuration
# Edit this file to configure your production environment

# Application settings
LAZABOT_CONFIG_PATH=/opt/lazabot/config
LAZABOT_DATA_PATH=/opt/lazabot/data
LAZABOT_LOG_PATH=/opt/lazabot/logs

# Database settings (if applicable)
# DATABASE_URL=sqlite:///opt/lazabot/data/lazabot.db

# API settings
# API_KEY=your_api_key_here
# API_SECRET=your_api_secret_here

# Proxy settings (if applicable)
# PROXY_LIST=/opt/lazabot/config/proxies.txt

# Monitoring settings
# METRICS_ENABLED=true
# METRICS_PORT=8080

# Security settings
# ENCRYPTION_KEY=your_encryption_key_here
# SESSION_SECRET=your_session_secret_here
ENV_EOF
    chmod 600 "${ENV_FILE}"
    chown root:root "${ENV_FILE}"
    echo -e "${GREEN}✅ Environment file created: ${ENV_FILE}${NC}"
    echo -e "${YELLOW}⚠️  Please edit ${ENV_FILE} with your production settings${NC}"
else
    echo -e "${YELLOW}ℹ️  Environment file already exists: ${ENV_FILE}${NC}"
fi

# Copy configuration files if they exist
echo -e "${YELLOW}📋 Copying configuration files...${NC}"
if [ -d "config" ]; then
    cp -r config/* "${INSTALL_DIR}/config/" 2>/dev/null || true
    chown -R "${USER_NAME}:${GROUP_NAME}" "${INSTALL_DIR}/config"
fi

# Copy example files if they exist
if [ -d "examples" ]; then
    cp -r examples "${INSTALL_DIR}/" 2>/dev/null || true
    chown -R "${USER_NAME}:${GROUP_NAME}" "${INSTALL_DIR}/examples"
fi

# Reload systemd and enable service
echo -e "${YELLOW}🔄 Reloading systemd and enabling service...${NC}"
systemctl daemon-reload
systemctl enable "${SERVICE_NAME}"

# Start service
echo -e "${YELLOW}▶️  Starting service...${NC}"
systemctl start "${SERVICE_NAME}"

# Wait a moment for service to start
sleep 5

# Check service status
echo -e "${YELLOW}🏥 Checking service status...${NC}"
if systemctl is-active --quiet "${SERVICE_NAME}"; then
    echo -e "${GREEN}✅ Service is running successfully!${NC}"
else
    echo -e "${RED}❌ Service failed to start${NC}"
    echo -e "${YELLOW}📋 Service status:${NC}"
    systemctl status "${SERVICE_NAME}" --no-pager
    echo -e "${YELLOW}📋 Service logs:${NC}"
    journalctl -u "${SERVICE_NAME}" --no-pager -n 20
    exit 1
fi

# Show service information
echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
echo -e "${BLUE}📊 Service Information:${NC}"
echo -e "   Service: ${SERVICE_NAME}"
echo -e "   Status: $(systemctl is-active ${SERVICE_NAME})"
echo -e "   Binary: ${BINARY_DIR}/${BINARY_NAME}"
echo -e "   Config: ${INSTALL_DIR}/config"
echo -e "   Data: ${INSTALL_DIR}/data"
echo -e "   Logs: ${INSTALL_DIR}/logs"
echo -e "   Environment: ${ENV_FILE}"
echo -e "   Backup: ${BACKUP_DIR}/lazabot_backup_${TIMESTAMP}.tar.gz"

echo -e "${BLUE}🔧 Management Commands:${NC}"
echo -e "   Start:   sudo systemctl start ${SERVICE_NAME}"
echo -e "   Stop:    sudo systemctl stop ${SERVICE_NAME}"
echo -e "   Restart: sudo systemctl restart ${SERVICE_NAME}"
echo -e "   Status:  sudo systemctl status ${SERVICE_NAME}"
echo -e "   Logs:    sudo journalctl -u ${SERVICE_NAME} -f"

echo -e "${BLUE}📝 Next Steps:${NC}"
echo -e "   1. Edit ${ENV_FILE} with your production settings"
echo -e "   2. Place your configuration files in ${INSTALL_DIR}/config"
echo -e "   3. Monitor logs: sudo journalctl -u ${SERVICE_NAME} -f"
echo -e "   4. Test the service: sudo systemctl status ${SERVICE_NAME}"

