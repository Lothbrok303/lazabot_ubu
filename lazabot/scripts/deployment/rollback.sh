#!/bin/bash

# Rollback Script
# This script rolls back to the previous deployment

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${RED}🔄 Starting Rollback Process...${NC}"

# Configuration
PROD_HOST="${PRODUCTION_SERVER_HOST:-production.example.com}"
PROD_USER="${PRODUCTION_SERVER_USER:-deploy}"
PROD_PATH="${PRODUCTION_DEPLOY_PATH:-/opt/lazabot}"
BACKUP_PATH="/opt/backups/lazabot"

# List available backups
echo -e "${YELLOW}📋 Available backups:${NC}"
ssh "${PROD_USER}@${PROD_HOST}" "ls -la ${BACKUP_PATH}/lazabot_backup_*.tar.gz 2>/dev/null || echo 'No backups found'"

# Get the latest backup
LATEST_BACKUP=$(ssh "${PROD_USER}@${PROD_HOST}" "ls -t ${BACKUP_PATH}/lazabot_backup_*.tar.gz 2>/dev/null | head -1")

if [ -z "$LATEST_BACKUP" ]; then
    echo -e "${RED}❌ No backups found for rollback${NC}"
    exit 1
fi

echo -e "${YELLOW}📦 Latest backup: $(basename $LATEST_BACKUP)${NC}"

# Confirm rollback
read -p "Are you sure you want to rollback to this backup? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}❌ Rollback cancelled${NC}"
    exit 1
fi

# Execute rollback
echo -e "${YELLOW}🔄 Executing rollback...${NC}"
ssh "${PROD_USER}@${PROD_HOST}" << ROLLBACK_EOF
    set -e
    echo "📁 Extracting backup..."
    cd /opt
    tar -xzf "$LATEST_BACKUP"
    
    echo "🔄 Restarting services..."
    sudo systemctl restart lazabot || echo "Service not found, starting manually"
    
    echo "🏥 Running health checks..."
    sleep 10
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        echo "✅ Health check passed - rollback successful"
    else
        echo "❌ Health check failed - rollback may have issues"
        exit 1
    fi
ROLLBACK_EOF

echo -e "${GREEN}✅ Rollback completed successfully!${NC}"
echo -e "${YELLOW}🔗 Production URL: https://${PROD_HOST}${NC}"
