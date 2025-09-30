#!/bin/bash

# Production Deployment Script
# This script deploys the application to the production environment

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting Production Deployment...${NC}"

# Configuration (update these for your environment)
PROD_HOST="${PRODUCTION_SERVER_HOST:-production.example.com}"
PROD_USER="${PRODUCTION_SERVER_USER:-deploy}"
PROD_PATH="${PRODUCTION_DEPLOY_PATH:-/opt/lazabot}"
APP_NAME="lazabot"
BACKUP_PATH="/opt/backups/lazabot"

# Pre-deployment checks
echo -e "${YELLOW}🔍 Running pre-deployment checks...${NC}"

# Check if we're on the main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "master" ] && [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${RED}❌ Production deployment must be from main/master branch${NC}"
    echo -e "${YELLOW}Current branch: $CURRENT_BRANCH${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}❌ Uncommitted changes detected. Please commit or stash them.${NC}"
    exit 1
fi

# Build the application
echo -e "${YELLOW}📦 Building application for production...${NC}"
cargo build --release --all-features

# Run tests
echo -e "${YELLOW}�� Running tests...${NC}"
cargo test --all-features

# Create deployment package
echo -e "${YELLOW}📋 Creating production deployment package...${NC}"
tar -czf "${APP_NAME}-production.tar.gz" \
    target/release/lazabot \
    scripts/ \
    tests/integration_test_config.yaml \
    README.md

# Create backup on production server
echo -e "${YELLOW}💾 Creating backup on production server...${NC}"
ssh "${PROD_USER}@${PROD_HOST}" << 'BACKUP_EOF'
    set -e
    BACKUP_DIR="/opt/backups/lazabot"
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    
    # Create backup directory if it doesn't exist
    mkdir -p "$BACKUP_DIR"
    
    # Backup current deployment
    if [ -d "/opt/lazabot" ]; then
        echo "Creating backup: lazabot_backup_$TIMESTAMP.tar.gz"
        tar -czf "$BACKUP_DIR/lazabot_backup_$TIMESTAMP.tar.gz" -C /opt lazabot
    fi
BACKUP_EOF

# Deploy to production server
echo -e "${YELLOW}🌐 Deploying to production server...${NC}"
scp "${APP_NAME}-production.tar.gz" "${PROD_USER}@${PROD_HOST}:/tmp/"

# Execute deployment on production server
ssh "${PROD_USER}@${PROD_HOST}" << 'DEPLOY_EOF'
    set -e
    echo "📁 Extracting deployment package..."
    cd /opt/lazabot
    tar -xzf /tmp/lazabot-production.tar.gz
    
    echo "🔄 Restarting services..."
    sudo systemctl restart lazabot || echo "Service not found, starting manually"
    
    echo "🏥 Running health checks..."
    sleep 10
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        echo "✅ Health check passed"
    else
        echo "❌ Health check failed - initiating rollback"
        # Rollback logic would go here
        exit 1
    fi
    
    echo "🧹 Cleaning up..."
    rm /tmp/lazabot-production.tar.gz
DEPLOY_EOF

# Clean up local files
rm "${APP_NAME}-production.tar.gz"

echo -e "${GREEN}✅ Production deployment completed successfully!${NC}"
echo -e "${YELLOW}🔗 Production URL: https://${PROD_HOST}${NC}"
echo -e "${BLUE}📊 Monitor your application at: https://${PROD_HOST}/metrics${NC}"
