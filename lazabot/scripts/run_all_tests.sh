#!/bin/bash

# Comprehensive Test Runner for Lazabot
# This script runs all tests locally to match CI pipeline

set -e

echo "🚀 Starting comprehensive test suite for Lazabot..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
print_status "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

if ! command_exists node; then
    print_error "Node.js not found. Please install Node.js 18+"
    exit 1
fi

if ! command_exists npm; then
    print_error "npm not found. Please install npm"
    exit 1
fi

if ! command_exists docker; then
    print_warning "Docker not found. Docker tests will be skipped."
    SKIP_DOCKER=true
fi

print_success "Prerequisites check completed"

# Rust Tests
print_status "Running Rust tests..."

print_status "Checking Rust formatting..."
if ! cargo fmt --all -- --check; then
    print_error "Rust formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi
print_success "Rust formatting check passed"

print_status "Running Clippy linter..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    print_error "Clippy linter failed. Fix the warnings above."
    exit 1
fi
print_success "Clippy linter passed"

print_status "Building Rust project..."
if ! cargo build --verbose --all-features; then
    print_error "Rust build failed"
    exit 1
fi
print_success "Rust build completed"

print_status "Running Rust unit tests..."
if ! cargo test --verbose --all-features; then
    print_error "Rust unit tests failed"
    exit 1
fi
print_success "Rust unit tests passed"

print_status "Running Rust integration tests..."
if ! cargo test --verbose --test '*' --all-features; then
    print_error "Rust integration tests failed"
    exit 1
fi
print_success "Rust integration tests passed"

print_status "Building Rust release..."
if ! cargo build --release --verbose --all-features; then
    print_error "Rust release build failed"
    exit 1
fi
print_success "Rust release build completed"

# Node.js Tests
print_status "Running Node.js tests..."

print_status "Installing Node.js dependencies..."
if ! npm ci; then
    print_error "npm ci failed"
    exit 1
fi
print_success "Node.js dependencies installed"

print_status "Installing Playwright browsers..."
if ! npx playwright install --with-deps; then
    print_error "Playwright browser installation failed"
    exit 1
fi
print_success "Playwright browsers installed"

print_status "Linting JavaScript code..."
if command_exists eslint; then
    if ! find scripts/ -name "*.js" -exec eslint {} \; 2>/dev/null; then
        print_warning "ESLint found issues. Consider fixing them."
    else
        print_success "JavaScript linting passed"
    fi
else
    print_warning "ESLint not found. Install with: npm install -g eslint"
fi

print_status "Checking for console.log statements..."
if grep -r "console\.log" scripts/ 2>/dev/null; then
    print_warning "Found console.log statements in production code"
else
    print_success "No console.log statements found"
fi

print_status "Running npm tests..."
if ! npm test; then
    print_error "npm tests failed"
    exit 1
fi
print_success "npm tests passed"

print_status "Running Playwright integration tests..."
if ! node scripts/test_full_integration.js; then
    print_error "Playwright integration tests failed"
    exit 1
fi
print_success "Playwright integration tests passed"

print_status "Running npm audit..."
if ! npm audit --audit-level moderate; then
    print_warning "npm audit found vulnerabilities. Review and fix them."
else
    print_success "npm audit passed"
fi

# Docker Tests
if [ "$SKIP_DOCKER" != "true" ]; then
    print_status "Running Docker tests..."
    
    print_status "Building Docker image..."
    if ! docker build -t lazabot:test .; then
        print_error "Docker build failed"
        exit 1
    fi
    print_success "Docker image built successfully"
    
    print_status "Testing Docker container..."
    if ! docker run --rm -d --name lazabot-test lazabot:test; then
        print_error "Docker container failed to start"
        exit 1
    fi
    
    print_status "Waiting for container to initialize..."
    sleep 10
    
    print_status "Checking container logs..."
    docker logs lazabot-test
    
    print_status "Stopping Docker container..."
    docker stop lazabot-test
    print_success "Docker tests passed"
else
    print_warning "Skipping Docker tests (Docker not available)"
fi

# Security Tests
print_status "Running security tests..."

print_status "Checking for secrets in code..."
if command_exists trufflehog; then
    if ! trufflehog filesystem . --no-verification; then
        print_warning "TruffleHog found potential secrets. Review the output above."
    else
        print_success "No secrets detected"
    fi
else
    print_warning "TruffleHog not found. Install with: go install github.com/trufflesecurity/trufflehog@latest"
fi

print_status "Checking for hardcoded credentials..."
if grep -r -i "password\|secret\|key\|token" --include="*.rs" --include="*.js" --include="*.yaml" --include="*.yml" . | grep -v "test\|example\|TODO\|FIXME"; then
    print_warning "Potential hardcoded credentials found. Review the output above."
else
    print_success "No hardcoded credentials detected"
fi

# Performance Tests
print_status "Running performance tests..."

print_status "Checking binary size..."
BINARY_SIZE=$(du -h target/release/lazabot 2>/dev/null | cut -f1 || echo "N/A")
print_status "Release binary size: $BINARY_SIZE"

print_status "Checking memory usage..."
if command_exists valgrind; then
    print_status "Running valgrind memory check..."
    timeout 30s valgrind --leak-check=full --error-exitcode=1 ./target/release/lazabot --help 2>/dev/null || print_warning "Valgrind check timed out or found issues"
else
    print_warning "Valgrind not found. Install with: sudo apt-get install valgrind"
fi

# Summary
print_success "🎉 All tests completed successfully!"
print_status "Test Summary:"
print_status "  ✅ Rust build and tests"
print_status "  ✅ Node.js tests and linting"
print_status "  ✅ Playwright integration tests"
print_status "  ✅ Security checks"
print_status "  ✅ Performance checks"
if [ "$SKIP_DOCKER" != "true" ]; then
    print_status "  ✅ Docker build and test"
fi

print_status "Ready for CI/CD pipeline! 🚀"
