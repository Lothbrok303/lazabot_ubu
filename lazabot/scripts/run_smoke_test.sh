#!/bin/bash

# Comprehensive Lazabot Smoke Test Runner
# This script orchestrates the complete E2E test pipeline

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
MOCK_SERVER_PORT=3001
MOCK_SERVER_PID=""
TEST_RESULTS_DIR="$SCRIPT_DIR/smoke_test_results"
TEST_START_TIME=$(date +%s)

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}Cleaning up...${NC}"
    
    if [ ! -z "$MOCK_SERVER_PID" ]; then
        echo "Stopping mock server (PID: $MOCK_SERVER_PID)"
        kill $MOCK_SERVER_PID 2>/dev/null || true
        wait $MOCK_SERVER_PID 2>/dev/null || true
    fi
    
    echo -e "${GREEN}Cleanup completed${NC}"
}

# Set up trap for cleanup
trap cleanup EXIT

# Function to print colored output
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

# Create test results directory
create_test_results_dir() {
    print_status "Creating test results directory..."
    mkdir -p "$TEST_RESULTS_DIR"
    print_success "Test results directory created: $TEST_RESULTS_DIR"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first."
        exit 1
    fi
    
    # Check if Node.js is installed
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js first."
        exit 1
    fi
    
    # Check if we're in the right directory
    if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
        print_error "Not in the lazabot project root. Please run from the project directory."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Build the lazabot binary
build_lazabot() {
    print_status "Building lazabot..."
    
    cd "$PROJECT_ROOT"
    
    if ! cargo build --release --bin lazabot; then
        print_error "Failed to build lazabot"
        exit 1
    fi
    
    print_success "Lazabot built successfully"
}

# Create and start mock server
setup_mock_server() {
    print_status "Setting up mock Lazada API server..."
    
    # Create mock server
    cat > "$TEST_RESULTS_DIR/mock_server.js" << 'MOCK_EOF'
const express = require('express');
const cors = require('cors');
const app = express();
const port = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Product state
let productState = {
    id: 'smoke-test-product',
    name: 'Test Product for Smoke Test',
    price: 100.00,
    stock: 0, // Start with no stock
    isAvailable: false,
    isFlashSale: false,
    flashSalePrice: 50.00
};

// Simulate flash sale trigger after 5 seconds
setTimeout(() => {
    console.log('🔥 Triggering flash sale!');
    productState.stock = 10;
    productState.isAvailable = true;
    productState.isFlashSale = true;
}, 5000);

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// Product details endpoint
app.get('/api/products/:id', (req, res) => {
    const { id } = req.params;
    
    if (id !== productState.id) {
        return res.status(404).json({ error: 'Product not found' });
    }
    
    res.json({
        id: productState.id,
        name: productState.name,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        stock: productState.stock,
        isAvailable: productState.isAvailable,
        isFlashSale: productState.isFlashSale,
        timestamp: new Date().toISOString()
    });
});

// Add to cart endpoint
app.post('/api/cart/add', (req, res) => {
    const { productId, quantity = 1 } = req.body;
    
    if (productId !== productState.id) {
        return res.status(404).json({ error: 'Product not found' });
    }
    
    if (!productState.isAvailable || productState.stock < quantity) {
        return res.status(400).json({ error: 'Product not available or insufficient stock' });
    }
    
    // Simulate cart addition
    const cartId = `cart_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    res.json({
        success: true,
        cartId: cartId,
        productId: productId,
        quantity: quantity,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

// Checkout endpoint
app.post('/api/checkout', (req, res) => {
    const { cartId, paymentMethod = 'credit_card' } = req.body;
    
    if (!cartId) {
        return res.status(400).json({ error: 'Cart ID required' });
    }
    
    // Simulate checkout processing
    const orderId = `order_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    res.json({
        success: true,
        orderId: orderId,
        cartId: cartId,
        status: 'pending',
        totalAmount: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

// Start server
app.listen(port, () => {
    console.log(`🚀 Mock Lazada API server running on port ${port}`);
    console.log(`📱 Product endpoint: http://localhost:${port}/api/products/${productState.id}`);
    console.log(`🛒 Add to cart: POST http://localhost:${port}/api/cart/add`);
    console.log(`💳 Checkout: POST http://localhost:${port}/api/checkout`);
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n🛑 Shutting down mock server...');
    process.exit(0);
});
MOCK_EOF

    # Create package.json
    cat > "$TEST_RESULTS_DIR/package.json" << 'PACKAGE_EOF'
{
  "name": "lazabot-mock-server",
  "version": "1.0.0",
  "description": "Mock Lazada API server for smoke testing",
  "main": "mock_server.js",
  "scripts": {
    "start": "node mock_server.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5"
  }
}
PACKAGE_EOF

    # Install dependencies
    cd "$TEST_RESULTS_DIR"
    if ! npm install --silent; then
        print_error "Failed to install mock server dependencies"
        exit 1
    fi
    
    # Start server
    node mock_server.js > mock_server.log 2>&1 &
    MOCK_SERVER_PID=$!
    
    # Wait for server to start
    sleep 3
    
    # Check if server is running
    if ! kill -0 $MOCK_SERVER_PID 2>/dev/null; then
        print_error "Failed to start mock server"
        exit 1
    fi
    
    # Test server health
    for i in {1..10}; do
        if curl -s "http://localhost:$MOCK_SERVER_PORT/health" > /dev/null 2>&1; then
            print_success "Mock server is running and healthy"
            return 0
        fi
        sleep 1
    done
    
    print_error "Mock server failed to start or is not responding"
    exit 1
}

# Create test products configuration
create_test_products() {
    print_status "Creating test products configuration..."
    
    cat > "$TEST_RESULTS_DIR/test_products.yaml" << EOF
products:
  - id: "smoke-test-product"
    name: "Smoke Test Product"
    url: "http://localhost:$MOCK_SERVER_PORT/api/products/smoke-test-product"
    target_price: 80.00
    min_stock: 1
    monitor_interval_ms: 1000
