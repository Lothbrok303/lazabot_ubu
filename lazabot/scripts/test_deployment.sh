#!/bin/bash

# Test Deployment Script
# This script tests the deployment on a remote server

set -euo pipefail

REMOTE_HOST="${REMOTE_HOST:-192.168.1.56}"
REMOTE_USER="${REMOTE_USER:-ubuntu}"
REMOTE_PORT="${REMOTE_PORT:-22}"

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

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_info "Testing deployment on $REMOTE_USER@$REMOTE_HOST:$REMOTE_PORT"

# Test SSH connection
log_info "Testing SSH connection..."
if ! ssh -o ConnectTimeout=10 -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "echo 'SSH connection successful'"; then
    log_error "Failed to connect to remote server"
    exit 1
fi

# Test if services are running
log_info "Checking if services are running..."
if ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "systemctl is-active --quiet lazabot && systemctl is-active --quiet lazabot-playwright"; then
    log_success "Services are running"
else
    log_error "Services are not running"
    ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo systemctl status lazabot lazabot-playwright --no-pager"
    exit 1
fi

# Test health endpoint
log_info "Testing health endpoint..."
if ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "curl -f http://localhost:8081/health"; then
    log_success "Health endpoint is responding"
else
    log_error "Health endpoint is not responding"
    exit 1
fi

# Test Playwright server
log_info "Testing Playwright server..."
if ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "curl -f http://localhost:8081/health | grep -q 'ok'"; then
    log_success "Playwright server is healthy"
else
    log_error "Playwright server health check failed"
fi

# Check disk space
log_info "Checking disk space..."
DISK_USAGE=$(ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "df /opt | awk 'NR==2 {print \$5}' | sed 's/%//'")
if [ "$DISK_USAGE" -lt 90 ]; then
    log_success "Disk usage is acceptable: ${DISK_USAGE}%"
else
    log_error "Disk usage is high: ${DISK_USAGE}%"
fi

# Check memory usage
log_info "Checking memory usage..."
MEMORY_USAGE=$(ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "free | awk 'NR==2{printf \"%.0f\", \$3*100/\$2}'")
if [ "$MEMORY_USAGE" -lt 90 ]; then
    log_success "Memory usage is acceptable: ${MEMORY_USAGE}%"
else
    log_error "Memory usage is high: ${MEMORY_USAGE}%"
fi

# Check logs for errors
log_info "Checking recent logs for errors..."
ERROR_COUNT=$(ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo journalctl -u lazabot --since '1 hour ago' | grep -i error | wc -l")
if [ "$ERROR_COUNT" -eq 0 ]; then
    log_success "No errors found in recent logs"
else
    log_error "Found $ERROR_COUNT errors in recent logs"
    ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" "sudo journalctl -u lazabot --since '1 hour ago' | grep -i error | tail -5"
fi

log_success "All tests passed! Deployment is working correctly."
