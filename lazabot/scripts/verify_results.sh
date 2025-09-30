#!/bin/bash

# Lazabot Smoke Test Results Verification Script
# Verifies that the smoke test results are correct

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MOCK_SERVER_PORT=3001
TEST_DB="smoke_test.db"
LAZABOT_LOG="lazabot.log"
MOCK_SERVER_LOG="mock_server.log"

# Print functions
print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                SMOKE TEST VERIFICATION                       ║"
    echo "║                                                              ║"
    echo "║  Verifying: Monitor → Flash Sale → Checkout → Database       ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_status() {
    echo -e "${BLUE}[$(date '+%H:%M:%S')]${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Check if mock server is running
check_mock_server() {
    print_status "Checking if mock server is running..."
    
    if curl -s "http://localhost:$MOCK_SERVER_PORT/health" > /dev/null 2>&1; then
        print_success "Mock server is running"
        return 0
    else
        print_warning "Mock server is not running"
        return 1
    fi
}

# Check lazabot logs for monitoring activity
check_monitoring_logs() {
    print_status "Checking lazabot monitoring logs..."
    
    if [ ! -f "$LAZABOT_LOG" ]; then
        print_error "Lazabot log file not found: $LAZABOT_LOG"
        return 1
    fi
    
    # Check for successful monitoring requests
    local monitoring_requests=$(grep -c "Request successful.*200.*api/products" "$LAZABOT_LOG" 2>/dev/null || echo "0")
    
    if [ "$monitoring_requests" -gt 0 ]; then
        print_success "Found $monitoring_requests monitoring requests in logs"
        return 0
    else
        print_error "No monitoring requests found in logs"
        return 1
    fi
}

# Check for flash sale detection
check_flash_sale_detection() {
    print_status "Checking for flash sale detection..."
    
    # Check mock server logs for flash sale trigger
    if [ -f "$MOCK_SERVER_LOG" ]; then
        if grep -q "FLASH SALE TRIGGERED" "$MOCK_SERVER_LOG" 2>/dev/null; then
            print_success "Flash sale triggered detected in mock server logs"
            return 0
        fi
    fi
    
    # Check lazabot logs for availability detection
    if [ -f "$LAZABOT_LOG" ]; then
        if grep -q "Product.*is now AVAILABLE" "$LAZABOT_LOG" 2>/dev/null; then
            print_success "Product availability detected in lazabot logs"
            return 0
        fi
    fi
    
    print_warning "Flash sale detection not clearly visible in logs"
    return 1
}

# Check for checkout attempts
check_checkout_attempts() {
    print_status "Checking for checkout attempts..."
    
    # Check mock server logs for checkout requests
    if [ -f "$MOCK_SERVER_LOG" ]; then
        if grep -q "CHECKOUT TRIGGERED" "$MOCK_SERVER_LOG" 2>/dev/null; then
            print_success "Checkout triggered detected in mock server logs"
            return 0
        fi
    fi
    
    # Check lazabot logs for checkout attempts
    if [ -f "$LAZABOT_LOG" ]; then
        if grep -q "checkout\|Checkout\|CHECKOUT" "$LAZABOT_LOG" 2>/dev/null; then
            print_success "Checkout activity detected in lazabot logs"
            return 0
        fi
    fi
    
    print_warning "Checkout attempts not clearly visible in logs"
    return 1
}

# Check database for orders
check_database_orders() {
    print_status "Checking database for orders..."
    
    # Check if database file exists
    if [ -f "$TEST_DB" ]; then
        if command -v sqlite3 &> /dev/null; then
            local order_count=$(sqlite3 "$TEST_DB" "SELECT COUNT(*) FROM orders;" 2>/dev/null || echo "0")
            if [ "$order_count" -gt 0 ]; then
                print_success "Found $order_count orders in database"
                
                # Show order details
                print_info "Order details:"
                sqlite3 "$TEST_DB" "SELECT * FROM orders;" 2>/dev/null || true
                return 0
            fi
        fi
    fi
    
    # Check mock server for orders
    if curl -s "http://localhost:$MOCK_SERVER_PORT/api/orders" > /dev/null 2>&1; then
        local orders_response=$(curl -s "http://localhost:$MOCK_SERVER_PORT/api/orders")
        local order_count=$(echo "$orders_response" | grep -o '"count":[0-9]*' | grep -o '[0-9]*' || echo "0")
        
        if [ "$order_count" -gt 0 ]; then
            print_success "Found $order_count orders in mock server"
            print_info "Orders: $orders_response"
            return 0
        fi
    fi
    
    print_warning "No orders found in database or mock server"
    return 1
}

# Show log analysis
show_log_analysis() {
    print_status "Analyzing logs..."
    
    echo
    echo -e "${BLUE}Log Analysis:${NC}"
    
    if [ -f "$LAZABOT_LOG" ]; then
        echo -e "${BLUE}Lazabot Log Summary:${NC}"
        echo "  Total lines: $(wc -l < "$LAZABOT_LOG" 2>/dev/null || echo "0")"
        echo "  Monitoring requests: $(grep -c "Request successful.*200.*api/products" "$LAZABOT_LOG" 2>/dev/null || echo "0")"
        echo "  Performance logs: $(grep -c "Operation.*completed" "$LAZABOT_LOG" 2>/dev/null || echo "0")"
        echo "  Availability changes: $(grep -c "is now AVAILABLE\|is now UNAVAILABLE" "$LAZABOT_LOG" 2>/dev/null || echo "0")"
        echo
    fi
    
    if [ -f "$MOCK_SERVER_LOG" ]; then
        echo -e "${BLUE}Mock Server Log Summary:${NC}"
        echo "  Total lines: $(wc -l < "$MOCK_SERVER_LOG" 2>/dev/null || echo "0")"
        echo "  Flash sales triggered: $(grep -c "FLASH SALE TRIGGERED" "$MOCK_SERVER_LOG" 2>/dev/null || echo "0")"
        echo "  Checkouts triggered: $(grep -c "CHECKOUT TRIGGERED" "$MOCK_SERVER_LOG" 2>/dev/null || echo "0")"
        echo "  Orders created: $(grep -c "Order:" "$MOCK_SERVER_LOG" 2>/dev/null || echo "0")"
        echo
    fi
}

# Generate verification report
generate_verification_report() {
    print_status "Generating verification report..."
    
    echo
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║                    VERIFICATION REPORT                      ║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo
    
    # Run checks
    local mock_server_ok=false
    local monitoring_ok=false
    local flash_sale_ok=false
    local checkout_ok=false
    local database_ok=false
    
    check_mock_server && mock_server_ok=true
    check_monitoring_logs && monitoring_ok=true
    check_flash_sale_detection && flash_sale_ok=true
    check_checkout_attempts && checkout_ok=true
    check_database_orders && database_ok=true
    
    # Show results
    echo -e "${BLUE}Verification Results:${NC}"
    echo -e "  Mock Server Running: $([ "$mock_server_ok" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Monitoring Activity: $([ "$monitoring_ok" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Flash Sale Detection: $([ "$flash_sale_ok" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Checkout Attempts: $([ "$checkout_ok" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Database Orders: $([ "$database_ok" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo
    
    # Overall result
    if [ "$mock_server_ok" = true ] && [ "$monitoring_ok" = true ] && [ "$flash_sale_ok" = true ] && [ "$checkout_ok" = true ] && [ "$database_ok" = true ]; then
        echo -e "${GREEN}🎉 ALL VERIFICATIONS PASSED! 🎉${NC}"
        echo -e "${GREEN}The smoke test results are valid and complete.${NC}"
        return 0
    else
        echo -e "${RED}❌ SOME VERIFICATIONS FAILED ❌${NC}"
        echo -e "${RED}Please check the logs and run the smoke test again.${NC}"
        return 1
    fi
}

# Main execution
main() {
    print_header
    
    # Run verification steps
    show_log_analysis
    generate_verification_report
    
    # Exit with appropriate code
    if [ $? -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Run main function
main "$@"
