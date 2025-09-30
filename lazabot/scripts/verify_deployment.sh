#!/bin/bash

# Lazabot Deployment Verification Script
# This script verifies that Lazabot is properly deployed and running

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
}

info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Check if running as root
if [[ $EUID -eq 0 ]]; then
    error "This script should not be run as root. Please run as a regular user."
    exit 1
fi

log "Starting Lazabot Deployment Verification..."

# Check if lazabot user exists
if id "$LAZABOT_USER" &>/dev/null; then
    success "User $LAZABOT_USER exists"
else
    error "User $LAZABOT_USER does not exist"
    exit 1
fi

# Check if application directory exists
if [[ -d "$LAZABOT_HOME" ]]; then
    success "Application directory exists: $LAZABOT_HOME"
else
    error "Application directory does not exist: $LAZABOT_HOME"
    exit 1
fi

# Check directory structure
log "Checking directory structure..."
for dir in bin config data logs scripts; do
    if [[ -d "$LAZABOT_HOME/$dir" ]]; then
        success "Directory exists: $LAZABOT_HOME/$dir"
    else
        error "Directory missing: $LAZABOT_HOME/$dir"
    fi
done

# Check file permissions
log "Checking file permissions..."
if [[ -r "$LAZABOT_HOME/config/.env" ]]; then
    success "Environment file exists and is readable"
else
    error "Environment file missing or not readable"
fi

if [[ -x "$LAZABOT_HOME/bin/lazabot" ]]; then
    success "Lazabot binary exists and is executable"
else
    error "Lazabot binary missing or not executable"
fi

# Check systemd services
log "Checking systemd services..."
if systemctl is-active --quiet lazabot.service; then
    success "Lazabot service is active"
else
    warning "Lazabot service is not active"
fi

if systemctl is-active --quiet lazabot-playwright.service; then
    success "Playwright service is active"
else
    warning "Playwright service is not active"
fi

# Check if services are enabled
if systemctl is-enabled --quiet lazabot.service; then
    success "Lazabot service is enabled"
else
    warning "Lazabot service is not enabled"
fi

if systemctl is-enabled --quiet lazabot-playwright.service; then
    success "Playwright service is enabled"
else
    warning "Playwright service is not enabled"
fi

# Check port listening
log "Checking port status..."
if netstat -tlnp 2>/dev/null | grep -q ":$LAZABOT_PORT "; then
    success "Port $LAZABOT_PORT is listening"
else
    warning "Port $LAZABOT_PORT is not listening"
fi

# Test health endpoint
log "Testing health endpoint..."
if curl -s --max-time 10 http://localhost:$LAZABOT_PORT/health > /dev/null; then
    success "Health endpoint is responding"
    
    # Get health response
    health_response=$(curl -s --max-time 10 http://localhost:$LAZABOT_PORT/health)
    if echo "$health_response" | jq -e '.status == "ok"' > /dev/null 2>&1; then
        success "Health check returned OK status"
        info "Health response: $health_response"
    else
        warning "Health check returned unexpected status"
        info "Health response: $health_response"
    fi
else
    error "Health endpoint is not responding"
fi

# Check processes
log "Checking processes..."
lazabot_processes=$(ps aux | grep -E "(lazabot|playwright)" | grep -v grep | wc -l)
if [[ $lazabot_processes -gt 0 ]]; then
    success "Found $lazabot_processes Lazabot-related processes"
    ps aux | grep -E "(lazabot|playwright)" | grep -v grep | while read line; do
        info "Process: $line"
    done
else
    warning "No Lazabot processes found"
fi

# Check log files
log "Checking log files..."
if [[ -d "$LAZABOT_HOME/logs" ]]; then
    log_files=$(find "$LAZABOT_HOME/logs" -name "*.log" 2>/dev/null | wc -l)
    if [[ $log_files -gt 0 ]]; then
        success "Found $log_files log files"
    else
        info "No log files found (this is normal for new installations)"
    fi
else
    warning "Log directory does not exist"
fi

# Check system resources
log "Checking system resources..."
disk_usage=$(df -h "$LAZABOT_HOME" | tail -1 | awk '{print $5}' | sed 's/%//')
if [[ $disk_usage -lt 80 ]]; then
    success "Disk usage is acceptable: ${disk_usage}%"
else
    warning "Disk usage is high: ${disk_usage}%"
fi

memory_usage=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
if [[ $memory_usage -lt 80 ]]; then
    success "Memory usage is acceptable: ${memory_usage}%"
else
    warning "Memory usage is high: ${memory_usage}%"
fi

# Check network connectivity
log "Checking network connectivity..."
if ping -c 1 8.8.8.8 > /dev/null 2>&1; then
    success "Internet connectivity is working"
else
    warning "Internet connectivity issues detected"
fi

# Check dependencies
log "Checking dependencies..."
if command -v rustc &> /dev/null; then
    success "Rust is installed: $(rustc --version)"
else
    warning "Rust is not installed"
fi

if command -v node &> /dev/null; then
    success "Node.js is installed: $(node --version)"
else
    warning "Node.js is not installed"
fi

if command -v curl &> /dev/null; then
    success "curl is available"
else
    warning "curl is not available"
fi

if command -v jq &> /dev/null; then
    success "jq is available"
else
    warning "jq is not available"
fi

# Final summary
echo
log "=== Verification Summary ==="
echo

success "Deployment verification completed!"
echo
echo "Next steps:"
echo "1. Update configuration: sudo nano $LAZABOT_HOME/config/.env"
echo "2. Test API endpoints: curl http://localhost:$LAZABOT_PORT/health"
echo "3. Monitor logs: sudo journalctl -u lazabot.service -f"
echo "4. Use management script: $LAZABOT_HOME/bin/manage status"
echo
log "Verification completed at $(date)"
