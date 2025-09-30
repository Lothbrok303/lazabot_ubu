#!/bin/bash

# Production Rollback Script for Lazabot
# This script rolls back to the previous deployment

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SERVICE_NAME="lazabot"
INSTALL_DIR="/opt/lazabot"
BINARY_DIR="/usr/local/bin"
SERVICE_DIR="/etc/systemd/system"
BACKUP_DIR="/opt/backups/lazabot"
BINARY_NAME="lazabot"

echo -e "${RED}🔄 Starting Lazabot Rollback Process...${NC}"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}❌ This script must be run as root (use sudo)${NC}"
    exit 1
fi

# List available backups
echo -e "${YELLOW}📋 Available backups:${NC}"
if [ ! -d "${BACKUP_DIR}" ]; then
    echo -e "${RED}❌ Backup directory not found: ${BACKUP_DIR}${NC}"
    exit 1
fi

ls -la "${BACKUP_DIR}"/lazabot_backup_*.tar.gz 2>/dev/null || {
    echo -e "${RED}❌ No backups found in ${BACKUP_DIR}${NC}"
    exit 1
}

# Get the latest backup
LATEST_BACKUP=$(ls -t "${BACKUP_DIR}"/lazabot_backup_*.tar.gz 2>/dev/null | head -1)

if [ -z "$LATEST_BACKUP" ]; then
    echo -e "${RED}❌ No backups found for rollback${NC}"
    exit 1
fi

echo -e "${YELLOW}📦 Latest backup: $(basename $LATEST_BACKUP)${NC}"

# Show backup details
echo -e "${BLUE}📊 Backup Details:${NC}"
ls -lh "$LATEST_BACKUP"
echo -e "   Created: $(stat -c %y "$LATEST_BACKUP")"

# Confirm rollback
echo -e "${YELLOW}⚠️  WARNING: This will replace the current installation with the backup!${NC}"
read -p "Are you sure you want to rollback to this backup? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}❌ Rollback cancelled${NC}"
    exit 0
fi

# Stop the service
echo -e "${YELLOW}🛑 Stopping service...${NC}"
systemctl stop "${SERVICE_NAME}" 2>/dev/null || echo "Service not running"

# Create current state backup before rollback
echo -e "${YELLOW}💾 Creating backup of current state before rollback...${NC}"
CURRENT_BACKUP="${BACKUP_DIR}/lazabot_pre_rollback_$(date +%Y%m%d_%H%M%S).tar.gz"
if [ -d "${INSTALL_DIR}" ]; then
    tar -czf "$CURRENT_BACKUP" -C /opt lazabot
    echo -e "${GREEN}✅ Current state backed up: $(basename $CURRENT_BACKUP)${NC}"
fi

# Execute rollback
echo -e "${YELLOW}🔄 Executing rollback...${NC}"

# Extract backup
echo "📁 Extracting backup..."
cd /opt
tar -xzf "$LATEST_BACKUP"

# Set proper permissions
echo "🔐 Setting permissions..."
chown -R lazabot:lazabot "${INSTALL_DIR}" 2>/dev/null || {
    echo -e "${YELLOW}⚠️  User lazabot not found, setting root ownership${NC}"
    chown -R root:root "${INSTALL_DIR}"
}

# Restart service
echo "🔄 Restarting service..."
systemctl daemon-reload
systemctl start "${SERVICE_NAME}"

# Wait for service to start
echo "⏳ Waiting for service to start..."
sleep 10

# Check service status
echo -e "${YELLOW}🏥 Checking service status...${NC}"
if systemctl is-active --quiet "${SERVICE_NAME}"; then
    echo -e "${GREEN}✅ Service is running successfully after rollback!${NC}"
else
    echo -e "${RED}❌ Service failed to start after rollback${NC}"
    echo -e "${YELLOW}�� Service status:${NC}"
    systemctl status "${SERVICE_NAME}" --no-pager
    echo -e "${YELLOW}📋 Service logs:${NC}"
    journalctl -u "${SERVICE_NAME}" --no-pager -n 20
    
    # Offer to restore from pre-rollback backup
    echo -e "${YELLOW}🔄 Would you like to restore from the pre-rollback backup?${NC}"
    read -p "Restore from pre-rollback backup? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}🔄 Restoring from pre-rollback backup...${NC}"
        systemctl stop "${SERVICE_NAME}" 2>/dev/null || true
        cd /opt
        tar -xzf "$CURRENT_BACKUP"
        chown -R lazabot:lazabot "${INSTALL_DIR}" 2>/dev/null || chown -R root:root "${INSTALL_DIR}"
        systemctl start "${SERVICE_NAME}"
        sleep 5
        if systemctl is-active --quiet "${SERVICE_NAME}"; then
            echo -e "${GREEN}✅ Pre-rollback state restored successfully!${NC}"
        else
            echo -e "${RED}❌ Failed to restore pre-rollback state${NC}"
            exit 1
        fi
    else
        exit 1
    fi
fi

# Show rollback information
echo -e "${GREEN}🎉 Rollback completed successfully!${NC}"
echo -e "${BLUE}📊 Rollback Information:${NC}"
echo -e "   Restored from: $(basename $LATEST_BACKUP)"
echo -e "   Service: ${SERVICE_NAME}"
echo -e "   Status: $(systemctl is-active ${SERVICE_NAME})"
echo -e "   Pre-rollback backup: $(basename $CURRENT_BACKUP)"

echo -e "${BLUE}🔧 Management Commands:${NC}"
echo -e "   Status:  sudo systemctl status ${SERVICE_NAME}"
echo -e "   Logs:    sudo journalctl -u ${SERVICE_NAME} -f"
echo -e "   Restart: sudo systemctl restart ${SERVICE_NAME}"

echo -e "${BLUE}📝 Available Backups:${NC}"
ls -la "${BACKUP_DIR}"/lazabot_*.tar.gz | tail -5

