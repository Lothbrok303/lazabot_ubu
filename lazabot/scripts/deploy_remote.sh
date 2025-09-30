#!/bin/bash

# Remote Deployment Script for Lazabot
# This script deploys the application to a remote Ubuntu server

set -euo pipefail

# Configuration
REMOTE_HOST="${REMOTE_HOST:-}"
REMOTE_USER="${REMOTE_USER:-ubuntu}"
REMOTE_PORT="${REMOTE_PORT:-22}"
LOCAL_SOURCE_DIR="$(pwd)"
REMOTE_SOURCE_DIR="/tmp/lazabot-source"
REMOTE_APP_DIR="/opt/lazabot"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if remote host is provided
if [ -z "$REMOTE_HOST" ]; then
    log_error "REMOTE_HOST environment variable is required"
    echo "Usage: REMOTE_HOST=192.168.1.56 ./scripts/deploy_remote.sh"
    exit 1
fi

log_info "Deploying Lazabot to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PORT"

# Check if SSH key exists
if [ ! -f ~/.ssh/id_rsa ] && [ ! -f ~/.ssh/id_ed25519 ]; then
    log_warning "No SSH key found. You may need to enter password multiple times."
fi

# Test SSH connection
log_info "Testing SSH connection..."
if ! ssh -o ConnectTimeout=10 -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "echo 'SSH connection successful'"; then
    log_error "Failed to connect to remote server"
    exit 1
fi

# Run setup script on remote server if not already done
log_info "Checking if setup is required..."
if ! ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "test -f $REMOTE_APP_DIR/scripts/deploy.sh"; then
    log_info "Running setup script on remote server..."
    ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "curl -fsSL https://raw.githubusercontent.com/your-repo/lazabot/main/scripts/setup.sh | bash" || {
        log_warning "Remote setup failed. Please run setup manually:"
        echo "ssh $REMOTE_USER@$REMOTE_HOST"
        echo "curl -fsSL https://raw.githubusercontent.com/your-repo/lazabot/main/scripts/setup.sh | bash"
    }
fi

# Copy source code to remote server
log_info "Copying source code to remote server..."
rsync -avz --delete \
    --exclude='.git' \
    --exclude='target' \
    --exclude='node_modules' \
    --exclude='logs' \
    --exclude='data' \
    --exclude='backups' \
    -e "ssh -p $REMOTE_PORT" \
    "$LOCAL_SOURCE_DIR/" \
    "$REMOTE_USER@$REMOTE_HOST:$REMOTE_SOURCE_DIR/"

# Run deployment on remote server
log_info "Running deployment on remote server..."
ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo $REMOTE_APP_DIR/scripts/deploy.sh"

# Check if services are running
log_info "Checking service status..."
ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo systemctl status lazabot lazabot-playwright --no-pager"

# Test health endpoint
log_info "Testing health endpoint..."
if ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "curl -f http://localhost:8081/health"; then
    log_success "Health check passed!"
else
    log_warning "Health check failed. Check logs:"
    ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo journalctl -u lazabot -n 20"
fi

log_success "Deployment completed!"
echo ""
echo "í´§ Useful commands:"
echo "  - SSH to server: ssh -p $REMOTE_PORT $REMOTE_USER@$REMOTE_HOST"
echo "  - Check status: ssh -p $REMOTE_PORT $REMOTE_USER@$REMOTE_HOST 'sudo systemctl status lazabot lazabot-playwright'"
echo "  - View logs: ssh -p $REMOTE_PORT $REMOTE_USER@$REMOTE_HOST 'sudo journalctl -u lazabot -f'"
echo "  - Health check: ssh -p $REMOTE_PORT $REMOTE_USER@$REMOTE_HOST 'curl http://localhost:8081/health'"
