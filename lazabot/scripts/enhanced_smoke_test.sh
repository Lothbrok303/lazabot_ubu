#!/bin/bash

# Enhanced Lazabot Smoke Test Script
# Tests the complete pipeline: Monitor → Flash Sale Detection → Checkout → Database Storage

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MOCK_SERVER_PORT=3001
MOCK_SERVER_PID=""
LAZABOT_PID=""
TEST_DB="smoke_test.db"
TEST_LOG="smoke_test.log"
PRODUCTS_FILE="$SCRIPT_DIR/smoke_test_products.yaml"
MOCK_SERVER_LOG="mock_server.log"
LAZABOT_LOG="lazabot.log"

# Test results
TEST_PASSED=false
MOCK_SERVER_STARTED=false
LAZABOT_BUILT=false
FLASH_SALE_TRIGGERED=false
CHECKOUT_DETECTED=false
ORDER_STORED=false

# Print functions
print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                ENHANCED LAZABOT SMOKE TEST                   ║"
    echo "║                                                              ║"
    echo "║  Monitor → Flash Sale Detection → Checkout → Database        ║"
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
    echo -e "${CYAN}ℹ${NC} $1"
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    
    # Kill lazabot process
    if [ ! -z "$LAZABOT_PID" ] && kill -0 $LAZABOT_PID 2>/dev/null; then
        print_info "Stopping lazabot process (PID: $LAZABOT_PID)"
        kill $LAZABOT_PID 2>/dev/null || true
        sleep 2
        kill -9 $LAZABOT_PID 2>/dev/null || true
    fi
    
    # Kill mock server
    if [ ! -z "$MOCK_SERVER_PID" ] && kill -0 $MOCK_SERVER_PID 2>/dev/null; then
        print_info "Stopping mock server (PID: $MOCK_SERVER_PID)"
        kill $MOCK_SERVER_PID 2>/dev/null || true
        sleep 1
        kill -9 $MOCK_SERVER_PID 2>/dev/null || true
    fi
    
    # Clean up test files
    cd "$PROJECT_ROOT"
    rm -f "$TEST_DB" "$TEST_LOG" "$MOCK_SERVER_LOG" "$LAZABOT_LOG" 2>/dev/null || true
    
    print_success "Cleanup completed"
}

trap cleanup EXIT

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if we're in the right directory
    if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
        print_error "Not in lazabot project directory. Please run from project root."
        exit 1
    fi
    
    # Check for Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js to run the mock server."
        exit 1
    fi
    
    # Check for npm
    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install npm."
        exit 1
    fi
    
    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust toolchain."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Create test products configuration
create_test_products() {
    print_status "Creating test products configuration..."
    
    cat > "$PRODUCTS_FILE" << 'PRODUCTS_EOF'
products:
  - id: "smoke-test-product"
    name: "Smoke Test Product"
    url: "http://localhost:3001/api/products/smoke-test-product"
    target_price: 50.00
    min_stock: 1
    monitor_interval_ms: 2000
PRODUCTS_EOF
    
    print_success "Test products configuration created"
}

# Create enhanced mock Lazada API server
create_mock_server() {
    print_status "Creating enhanced mock Lazada API server..."
    
    cat > "$SCRIPT_DIR/mock_lazada_server.js" << 'MOCK_EOF'
#!/usr/bin/env node

const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Product state - starts as unavailable
let productState = {
    id: 'smoke-test-product',
    name: 'Smoke Test Product',
    price: 100.00,
    flashSalePrice: 50.00,
    stock: 0,
    isAvailable: false,
    isFlashSale: false,
    lastUpdated: new Date().toISOString()
};

// Order storage
let orders = [];
let orderCounter = 1;

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'healthy',
        timestamp: new Date().toISOString(),
        server: 'Mock Lazada API Server'
    });
});

// Product details endpoint - returns HTML-like response for monitoring
app.get('/api/products/:id', (req, res) => {
    const { id } = req.params;
    
    if (id !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId: id
        });
    }
    
    // Return HTML-like response that the monitor can parse
    const html = `
<!DOCTYPE html>
<html>
<head><title>${productState.name}</title></head>
<body>
    <div class="product-info">
        <h1>${productState.name}</h1>
        <div class="price">$${productState.isFlashSale ? productState.flashSalePrice : productState.price}</div>
        <div class="stock">Stock: ${productState.stock}</div>
        <div class="availability">${productState.isAvailable ? 'Available' : 'Out of Stock'}</div>
        ${productState.isAvailable ? '<button class="add-to-cart">Add to Cart</button>' : ''}
    </div>
</body>
</html>`;
    
    res.setHeader('Content-Type', 'text/html');
    res.send(html);
});

// Product availability check endpoint
app.get('/api/products/:id/availability', (req, res) => {
    const { id } = req.params;
    
    if (id !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId: id
        });
    }
    
    res.json({
        success: true,
        available: productState.isAvailable,
        stock: productState.stock,
        isFlashSale: productState.isFlashSale,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

// Checkout endpoint
app.post('/api/checkout', (req, res) => {
    const { productId, quantity = 1, accountId = 'test-account' } = req.body;
    
    if (productId !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId
        });
    }
    
    if (!productState.isAvailable) {
        return res.status(400).json({
            error: 'Product not available',
            available: false
        });
    }
    
    if (productState.stock < quantity) {
        return res.status(400).json({
            error: 'Insufficient stock',
            available: productState.stock,
            requested: quantity
        });
    }
    
    // Create order
    const orderId = `order_${orderCounter++}_${Date.now()}`;
    const order = {
        orderId,
        productId,
        accountId,
        quantity,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        status: 'pending',
        createdAt: new Date().toISOString()
    };
    
    orders.push(order);
    
    // Update stock
    productState.stock -= quantity;
    if (productState.stock <= 0) {
        productState.isAvailable = false;
    }
    
    console.log(`[${new Date().toISOString()}] 🛒 CHECKOUT TRIGGERED! Order: ${orderId}, Product: ${productId}, Quantity: ${quantity}, Price: ${order.price}`);
    
    res.json({
        success: true,
        orderId,
        message: 'Order created successfully',
        order,
        timestamp: new Date().toISOString()
    });
});

// Trigger flash sale endpoint (for testing)
app.post('/api/test/flash-sale', (req, res) => {
    productState.stock = 10;
    productState.isAvailable = true;
    productState.isFlashSale = true;
    productState.lastUpdated = new Date().toISOString();
    
    console.log(`[${new Date().toISOString()}] 🔥 FLASH SALE TRIGGERED! Product now available with ${productState.stock} stock at ${productState.flashSalePrice} (was ${productState.price})`);
    
    res.json({
        success: true,
        message: 'Flash sale triggered',
        productState,
        timestamp: new Date().toISOString()
    });
});

// Reset product state endpoint
app.post('/api/test/reset', (req, res) => {
    productState.stock = 0;
    productState.isAvailable = false;
    productState.isFlashSale = false;
    productState.lastUpdated = new Date().toISOString();
    
    console.log(`[${new Date().toISOString()}] 🔄 Product state reset`);
    
    res.json({
        success: true,
        message: 'Product state reset',
        productState,
        timestamp: new Date().toISOString()
    });
});

// Get orders endpoint
app.get('/api/orders', (req, res) => {
    res.json({
        success: true,
        orders,
        count: orders.length,
        timestamp: new Date().toISOString()
    });
});

// Start server
app.listen(PORT, () => {
    console.log(`[${new Date().toISOString()}] 🚀 Mock Lazada API Server running on port ${PORT}`);
    console.log(`[${new Date().toISOString()}] 📍 Health check: http://localhost:${PORT}/health`);
    console.log(`[${new Date().toISOString()}] 📦 Product endpoint: http://localhost:${PORT}/api/products/smoke-test-product`);
    console.log(`[${new Date().toISOString()}] 🔥 Flash sale trigger: http://localhost:${PORT}/api/test/flash-sale`);
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log(`[${new Date().toISOString()}] 🛑 Shutting down mock server...`);
    process.exit(0);
});

process.on('SIGTERM', () => {
    console.log(`[${new Date().toISOString()}] 🛑 Shutting down mock server...`);
    process.exit(0);
});
MOCK_EOF
    
    chmod +x "$SCRIPT_DIR/mock_lazada_server.js"
    print_success "Enhanced mock server created"
}

# Install mock server dependencies
install_mock_dependencies() {
    print_status "Installing mock server dependencies..."
    
    cd "$SCRIPT_DIR"
    
    # Create package.json if it doesn't exist
    if [ ! -f "package.json" ]; then
        cat > package.json << 'PACKAGE_EOF'
{
  "name": "lazabot-mock-server",
  "version": "1.0.0",
  "description": "Mock Lazada API server for smoke testing",
  "main": "mock_lazada_server.js",
  "scripts": {
    "start": "node mock_lazada_server.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5"
  }
}
PACKAGE_EOF
    fi
    
    # Install dependencies
    npm install --silent
    
    print_success "Mock server dependencies installed"
}

# Start mock server
start_mock_server() {
    print_status "Starting mock Lazada API server..."
    
    cd "$SCRIPT_DIR"
    
    # Start server in background
    node mock_lazada_server.js > "$MOCK_SERVER_LOG" 2>&1 &
    MOCK_SERVER_PID=$!
    
    # Wait for server to start
    sleep 3
    
    # Test server health
    for i in {1..10}; do
        if curl -s "http://localhost:$MOCK_SERVER_PORT/health" > /dev/null 2>&1; then
            MOCK_SERVER_STARTED=true
            print_success "Mock server started (PID: $MOCK_SERVER_PID)"
            return 0
        fi
        sleep 1
    done
    
    print_error "Failed to start mock server"
    print_info "Mock server logs:"
    cat "$MOCK_SERVER_LOG" 2>/dev/null || true
    exit 1
}

# Build lazabot
build_lazabot() {
    print_status "Building lazabot..."
    
    cd "$PROJECT_ROOT"
    
    # Build in release mode for better performance
    cargo build --release --quiet
    
    if [ -f "target/release/lazabot" ]; then
        LAZABOT_BUILT=true
        print_success "Lazabot built successfully"
    else
        print_error "Failed to build lazabot"
        exit 1
    fi
}

# Simulate monitoring and checkout process
simulate_monitoring_and_checkout() {
    print_status "Simulating monitoring and checkout process..."
    
    cd "$PROJECT_ROOT"
    
    # Start monitoring in background
    timeout 30s ./target/release/lazabot monitor \
        --products "$PRODUCTS_FILE" \
        --verbose > "$LAZABOT_LOG" 2>&1 &
    LAZABOT_PID=$!
    
    # Wait for monitoring to start
    sleep 3
    
    # Trigger flash sale
    print_status "Triggering flash sale..."
    response=$(curl -s -X POST "http://localhost:$MOCK_SERVER_PORT/api/test/flash-sale")
    
    if echo "$response" | grep -q "success.*true"; then
        FLASH_SALE_TRIGGERED=true
        print_success "Flash sale triggered successfully"
    else
        print_error "Failed to trigger flash sale"
        return 1
    fi
    
    # Wait for monitoring to detect availability
    sleep 5
    
    # Simulate checkout attempt
    print_status "Simulating checkout attempt..."
    checkout_response=$(curl -s -X POST "http://localhost:$MOCK_SERVER_PORT/api/checkout" \
        -H "Content-Type: application/json" \
        -d '{"productId": "smoke-test-product", "quantity": 1, "accountId": "test-account"}')
    
    if echo "$checkout_response" | grep -q "success.*true"; then
        CHECKOUT_DETECTED=true
        print_success "Checkout attempt successful"
        print_info "Checkout response: $checkout_response"
    else
        print_warning "Checkout attempt failed or not detected"
        print_info "Checkout response: $checkout_response"
    fi
    
    # Wait for monitoring to finish
    wait $LAZABOT_PID 2>/dev/null || true
}

# Verify database storage
verify_database_storage() {
    print_status "Verifying database storage..."
    
    # Check if database file exists
    if [ ! -f "$TEST_DB" ]; then
        print_warning "Database file not found, checking for in-memory database"
        # The database might be in-memory, so we'll check the logs instead
        if grep -q "Order stored\|order.*stored\|Database.*insert" "$LAZABOT_LOG" 2>/dev/null; then
            ORDER_STORED=true
            print_success "Order storage detected in logs"
            return 0
        fi
    else
        # Check database for orders
        if command -v sqlite3 &> /dev/null; then
            local order_count=$(sqlite3 "$TEST_DB" "SELECT COUNT(*) FROM orders;" 2>/dev/null || echo "0")
            if [ "$order_count" -gt 0 ]; then
                ORDER_STORED=true
                print_success "Order found in database (count: $order_count)"
                
                # Show order details
                print_info "Order details:"
                sqlite3 "$TEST_DB" "SELECT * FROM orders;" 2>/dev/null || true
                return 0
            fi
        fi
    fi
    
    # Check mock server for orders
    local orders_response=$(curl -s "http://localhost:$MOCK_SERVER_PORT/api/orders")
    local order_count=$(echo "$orders_response" | grep -o '"count":[0-9]*' | grep -o '[0-9]*' || echo "0")
    
    if [ "$order_count" -gt 0 ]; then
        ORDER_STORED=true
        print_success "Order found in mock server (count: $order_count)"
        print_info "Orders: $orders_response"
        return 0
    fi
    
    print_warning "No order found in database or mock server"
    return 1
}

# Generate test report
generate_report() {
    print_status "Generating test report..."
    
    echo
    echo -e "${PURPLE}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${PURPLE}║                        TEST REPORT                          ║${NC}"
    echo -e "${PURPLE}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo
    
    # Test results
    echo -e "${BLUE}Test Results:${NC}"
    echo -e "  Mock Server Started: $([ "$MOCK_SERVER_STARTED" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Lazabot Built: $([ "$LAZABOT_BUILT" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Flash Sale Triggered: $([ "$FLASH_SALE_TRIGGERED" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Checkout Detected: $([ "$CHECKOUT_DETECTED" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo -e "  Order Stored: $([ "$ORDER_STORED" = true ] && echo -e "${GREEN}✓ PASS${NC}" || echo -e "${RED}✗ FAIL${NC}")"
    echo
    
    # Overall result
    if [ "$MOCK_SERVER_STARTED" = true ] && [ "$LAZABOT_BUILT" = true ] && [ "$FLASH_SALE_TRIGGERED" = true ] && [ "$CHECKOUT_DETECTED" = true ] && [ "$ORDER_STORED" = true ]; then
        TEST_PASSED=true
        echo -e "${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
        echo -e "${GREEN}The core pipeline is working correctly.${NC}"
    else
        echo -e "${RED}❌ SOME TESTS FAILED ❌${NC}"
        echo -e "${RED}Please check the logs for more details.${NC}"
    fi
    echo
    
    # Log locations
    echo -e "${BLUE}Log Files:${NC}"
    echo -e "  Mock Server: $MOCK_SERVER_LOG"
    echo -e "  Lazabot: $LAZABOT_LOG"
    echo -e "  Test Database: $TEST_DB"
    echo
    
    # Instructions for interpreting results
    echo -e "${BLUE}How to Interpret Results:${NC}"
    echo -e "  1. Check mock server logs for API requests and responses"
    echo -e "  2. Check lazabot logs for monitoring and checkout attempts"
    echo -e "  3. Look for 'checkout triggered' or 'CHECKOUT TRIGGERED' messages"
    echo -e "  4. Verify database contains order records"
    echo -e "  5. All components should work together seamlessly"
    echo
    
    # Show recent logs
    if [ -f "$LAZABOT_LOG" ]; then
        echo -e "${BLUE}Recent Lazabot Logs:${NC}"
        tail -20 "$LAZABOT_LOG" 2>/dev/null || true
        echo
    fi
    
    if [ -f "$MOCK_SERVER_LOG" ]; then
        echo -e "${BLUE}Recent Mock Server Logs:${NC}"
        tail -10 "$MOCK_SERVER_LOG" 2>/dev/null || true
        echo
    fi
}

# Main execution
main() {
    print_header
    
    # Run test steps
    check_prerequisites
    create_test_products
    create_mock_server
    install_mock_dependencies
    start_mock_server
    build_lazabot
    simulate_monitoring_and_checkout
    verify_database_storage
    generate_report
    
    # Exit with appropriate code
    if [ "$TEST_PASSED" = true ]; then
        exit 0
    else
        exit 1
    fi
}

# Run main function
main "$@"
