#!/bin/bash

# Staging Deployment Script
# This script deploys the application to the staging environment

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${YELLOW}🚀 Starting Staging Deployment...${NC}"

# Configuration (update these for your environment)
STAGING_HOST="${STAGING_SERVER_HOST:-staging.example.com}"
STAGING_USER="${STAGING_SERVER_USER:-deploy}"
STAGING_PATH="${STAGING_DEPLOY_PATH:-/opt/lazabot}"
APP_NAME="lazabot"

# Build the application
echo -e "${YELLOW}📦 Building application...${NC}"
cargo build --release --all-features

# Create deployment package
echo -e "${YELLOW}📋 Creating deployment package...${NC}"
tar -czf "${APP_NAME}-staging.tar.gz" \
    target/release/lazabot \
    scripts/ \
    tests/integration_test_config.yaml \
    README.md

# Deploy to staging server
echo -e "${YELLOW}🌐 Deploying to staging server...${NC}"
scp "${APP_NAME}-staging.tar.gz" "${STAGING_USER}@${STAGING_HOST}:/tmp/"

# Execute deployment on staging server
ssh "${STAGING_USER}@${STAGING_HOST}" << 'DEPLOY_EOF'
    set -e
    echo "📁 Extracting deployment package..."
    cd /opt/lazabot
    tar -xzf /tmp/lazabot-staging.tar.gz
    
    echo "🔄 Restarting services..."
    sudo systemctl restart lazabot-staging || echo "Service not found, starting manually"
    
    echo "🏥 Running health checks..."
    sleep 5
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        echo "✅ Health check passed"
    else
        echo "❌ Health check failed"
        exit 1
    fi
    
    echo "🧹 Cleaning up..."
    rm /tmp/lazabot-staging.tar.gz
DEPLOY_EOF

# Clean up local files
rm "${APP_NAME}-staging.tar.gz"

echo -e "${GREEN}✅ Staging deployment completed successfully!${NC}"
echo -e "${YELLOW}🔗 Staging URL: http://${STAGING_HOST}${NC}"
