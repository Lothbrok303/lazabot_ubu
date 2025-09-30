#!/bin/bash

# Test Deployment Script for Lazabot
# This script tests the deployment without actually deploying

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🧪 Testing Lazabot Deployment...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Not in lazabot project directory${NC}"
    echo -e "${YELLOW}   Please run this script from the lazabot project root${NC}"
    exit 1
fi

# Check if binary exists
if [ ! -f "target/release/lazabot" ]; then
    echo -e "${YELLOW}🔨 Building project first...${NC}"
    cargo build --release
fi

# Test binary execution
echo -e "${YELLOW}🔍 Testing binary execution...${NC}"
if ./target/release/lazabot --help > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Binary executes successfully${NC}"
else
    echo -e "${RED}❌ Binary execution failed${NC}"
    exit 1
fi

# Test service file syntax (create temporary binary for validation)
echo -e "${YELLOW}🔍 Testing service file syntax...${NC}"
if [ -f "lazabot.service" ]; then
    # Create a temporary binary for systemd validation
    TEMP_BINARY="/tmp/lazabot_test"
    cp target/release/lazabot "$TEMP_BINARY"
    chmod +x "$TEMP_BINARY"
    
    # Create temporary service file with correct binary path
    TEMP_SERVICE="/tmp/lazabot_test.service"
    sed "s|/usr/local/bin/lazabot|$TEMP_BINARY|g" lazabot.service > "$TEMP_SERVICE"
    
    if systemd-analyze verify "$TEMP_SERVICE" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Service file syntax is valid${NC}"
    else
        echo -e "${RED}❌ Service file syntax is invalid${NC}"
        systemd-analyze verify "$TEMP_SERVICE"
        rm -f "$TEMP_BINARY" "$TEMP_SERVICE"
        exit 1
    fi
    
    # Cleanup
    rm -f "$TEMP_BINARY" "$TEMP_SERVICE"
else
    echo -e "${RED}❌ Service file not found${NC}"
    exit 1
fi

# Test deployment script syntax
echo -e "${YELLOW}🔍 Testing deployment script syntax...${NC}"
if [ -f "scripts/deploy.sh" ]; then
    if bash -n scripts/deploy.sh; then
        echo -e "${GREEN}✅ Deployment script syntax is valid${NC}"
    else
        echo -e "${RED}❌ Deployment script syntax is invalid${NC}"
        exit 1
    fi
else
    echo -e "${RED}❌ Deployment script not found${NC}"
    exit 1
fi

# Test rollback script syntax
echo -e "${YELLOW}🔍 Testing rollback script syntax...${NC}"
if [ -f "scripts/rollback.sh" ]; then
    if bash -n scripts/rollback.sh; then
        echo -e "${GREEN}✅ Rollback script syntax is valid${NC}"
    else
        echo -e "${RED}❌ Rollback script syntax is invalid${NC}"
        exit 1
    fi
else
    echo -e "${RED}❌ Rollback script not found${NC}"
    exit 1
fi

# Check required directories
echo -e "${YELLOW}🔍 Checking project structure...${NC}"
REQUIRED_DIRS=("scripts" "examples" "config")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo -e "${GREEN}✅ Directory $dir exists${NC}"
    else
        echo -e "${YELLOW}⚠️  Directory $dir missing (optional)${NC}"
    fi
done

# Check file permissions
echo -e "${YELLOW}🔍 Checking file permissions...${NC}"
if [ -x "scripts/deploy.sh" ]; then
    echo -e "${GREEN}✅ deploy.sh is executable${NC}"
else
    echo -e "${RED}❌ deploy.sh is not executable${NC}"
    chmod +x scripts/deploy.sh
    echo -e "${YELLOW}   Fixed: Made deploy.sh executable${NC}"
fi

if [ -x "scripts/rollback.sh" ]; then
    echo -e "${GREEN}✅ rollback.sh is executable${NC}"
else
    echo -e "${RED}❌ rollback.sh is not executable${NC}"
    chmod +x scripts/rollback.sh
    echo -e "${YELLOW}   Fixed: Made rollback.sh executable${NC}"
fi

# Test dry-run deployment (simulate without actually deploying)
echo -e "${YELLOW}🔍 Testing dry-run deployment...${NC}"

# Check if running as root (for actual deployment)
if [ "$EUID" -eq 0 ]; then
    echo -e "${YELLOW}⚠️  Running as root - this will perform actual deployment${NC}"
    echo -e "${YELLOW}   For dry-run testing, run as non-root user${NC}"
else
    echo -e "${GREEN}✅ Running as non-root - safe for testing${NC}"
    
    # Test what would happen in deployment
    echo -e "${BLUE}📋 Deployment simulation:${NC}"
    echo -e "   Binary: target/release/lazabot"
    echo -e "   Service: lazabot.service"
    echo -e "   Install dir: /opt/lazabot"
    echo -e "   Binary dir: /usr/local/bin"
    echo -e "   Service dir: /etc/systemd/system"
    echo -e "   User: lazabot"
    echo -e "   Group: lazabot"
fi

# Show deployment readiness
echo -e "${GREEN}🎉 Deployment test completed successfully!${NC}"
echo -e "${BLUE}📊 Test Results:${NC}"
echo -e "   Binary: ✅ Ready"
echo -e "   Service file: ✅ Valid"
echo -e "   Deploy script: ✅ Valid"
echo -e "   Rollback script: ✅ Valid"
echo -e "   Permissions: ✅ Correct"

echo -e "${BLUE}🚀 Ready for deployment!${NC}"
echo -e "${YELLOW}   To deploy: sudo ./scripts/deploy.sh${NC}"
echo -e "${YELLOW}   To rollback: sudo ./scripts/rollback.sh${NC}"

