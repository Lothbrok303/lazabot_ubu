# Lazabot Smoke Test

This directory contains a comprehensive smoke test for the Lazabot CLI application that validates the complete pipeline from product monitoring to checkout completion.

## Overview

The smoke test simulates a real-world scenario where:
1. A product starts as unavailable (out of stock)
2. A flash sale is triggered (product becomes available at a reduced price)
3. Lazabot detects the availability change
4. Lazabot attempts to checkout the product
5. The order is stored in the SQLite database

## Files

- `smoke_test.sh` - Main smoke test script
- `mock_lazada_server.js` - Mock Lazada API server
- `smoke_test_products.yaml` - Test product configuration
- `verify_results.sh` - Script to verify test results
- `package.json` - Node.js dependencies for mock server

## Prerequisites

- Rust (for building lazabot)
- Node.js and npm (for mock server)
- curl (for API testing)
- sqlite3 (optional, for database verification)

## Quick Start

```bash
# Run the complete smoke test
bash scripts/smoke_test.sh

# Verify results
bash scripts/verify_results.sh
```

## Detailed Usage

### 1. Run Smoke Test

```bash
cd lazabot
bash scripts/smoke_test.sh
```

This will:
- Check prerequisites
- Build lazabot binary
- Install mock server dependencies
- Start mock Lazada API server
- Start lazabot monitoring
- Trigger a flash sale
- Monitor for checkout detection
- Verify order storage in database
- Generate a test report

### 2. Verify Results

```bash
bash scripts/verify_results.sh
```

This will:
- Check if database file was created
- Query database for orders
- Check logs for checkout indicators
- Look for errors in logs
- Provide a summary

### 3. Manual Testing

You can also test components individually:

```bash
# Start mock server only
cd scripts
npm install
node mock_lazada_server.js

# Test API endpoints
curl http://localhost:3001/health
curl http://localhost:3001/api/products/smoke-test-product
curl -X POST http://localhost:3001/api/test/flash-sale

# Run lazabot monitoring
cd ..
./target/release/lazabot monitor --products scripts/smoke_test_products.yaml --interval 2 --verbose
```

## Expected Output

### Successful Test Run

```
╔══════════════════════════════════════════════════════════════╗
║                    LAZABOT SMOKE TEST                        ║
║                                                              ║
║  Monitor → Flash Sale Detection → Checkout → Database        ║
╚══════════════════════════════════════════════════════════════╝

[12:34:56] Checking prerequisites...
✓ Prerequisites check passed
[12:34:57] Building lazabot...
✓ Lazabot built successfully
[12:34:58] Installing mock server dependencies...
✓ Mock server dependencies installed
[12:34:59] Starting mock Lazada API server...
✓ Mock server is running on port 3001
[12:35:00] Starting lazabot monitoring...
ℹ Lazabot started with PID: 12345
[12:35:01] Triggering flash sale...
✓ Flash sale triggered!
[12:35:02] Monitoring for checkout detection...
✓ Checkout detected in logs!
[12:35:03] Verifying order in database...
✓ Database file created: smoke_test.db
✓ Order found in database! Count: 1
✓ Test report generated: smoke_test_report.md

🎉 Smoke test completed successfully!

Test Results:
  Mock Server: ✅ Running
  Flash Sale: ✅ Triggered
  Checkout: ✅ Detected
  Database: ✅ Order Stored

📊 Report: scripts/smoke_test_report.md
📝 Logs: scripts/smoke_test.log
🗄️  Database: scripts/smoke_test.db
```

### Verification Output

```
🔍 Verifying Smoke Test Results
================================
✅ Database file found: smoke_test.db

📊 Database Contents:
--------------------
orders

📋 Orders Table:
1|order_1234567890_abc123|smoke-test-product|test-account|pending|50.0|1|{}|2024-01-01T12:35:00Z|2024-01-01T12:35:00Z

📈 Order Count:
1

✅ Log file found: smoke_test.log

📝 Recent Log Entries:
----------------------
[12:35:00] Starting monitor for product: Test Product for Smoke Test (smoke-test-product)
[12:35:01] Product smoke-test-product check successful: available=true
[12:35:02] Checkout triggered for product: smoke-test-product
[12:35:03] Order completed: order_1234567890_abc123

🔍 Checkout Detection:
✅ Checkout detected in logs
[12:35:02] Checkout triggered for product: smoke-test-product

🔍 Error Detection:
✅ No errors found in logs

📊 Summary:
----------
Database: ✅ Found
Logs: ✅ Found
Checkout: ✅ Detected
Errors: ✅ None
```

## Test Configuration

### Product Configuration (`smoke_test_products.yaml`)

```yaml
products:
  - id: "smoke-test-product"
    name: "Test Product for Smoke Test"
    url: "http://localhost:3001/api/products/smoke-test-product"
    target_price: 75.00
    min_stock: 1
    monitor_interval_ms: 2000
```

### Mock Server Endpoints

- `GET /health` - Health check
- `GET /api/products/:id` - Product details
- `POST /api/cart/add` - Add to cart
- `POST /api/checkout` - Complete checkout
- `POST /api/test/flash-sale` - Trigger flash sale
- `POST /api/test/reset` - Reset product state

## Troubleshooting

### Common Issues

1. **Build Failures**
   - Ensure Rust is installed: `rustup --version`
   - Check Cargo.toml dependencies
   - Run `cargo clean && cargo build --release`

2. **Mock Server Issues**
   - Ensure Node.js is installed: `node --version`
   - Install dependencies: `cd scripts && npm install`
   - Check port availability: `netstat -tlnp | grep 3001`

3. **Database Issues**
   - Check SQLite installation: `sqlite3 --version`
   - Verify file permissions
   - Check disk space

4. **Network Issues**
   - Ensure localhost connectivity
   - Check firewall settings
   - Verify port 3001 is not blocked

### Debug Mode

```bash
# Enable verbose logging
export RUST_LOG=debug
bash scripts/smoke_test.sh

# Check individual components
cd scripts
node mock_lazada_server.js &
curl -v http://localhost:3001/health
```

### Log Analysis

```bash
# View lazabot logs
cat scripts/smoke_test.log

# View mock server logs
cat scripts/mock_server.log

# Search for specific patterns
grep -i "checkout" scripts/smoke_test.log
grep -i "error" scripts/smoke_test.log
```

## Integration with CI/CD

The smoke test can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Smoke Tests
  run: |
    cd lazabot
    bash scripts/smoke_test.sh
    
- name: Upload Test Results
  uses: actions/upload-artifact@v3
  with:
    name: smoke-test-results
    path: lazabot/scripts/smoke_test_results/
```

## Next Steps

1. **Real API Integration** - Test with actual Lazada API endpoints
2. **Performance Testing** - Add load testing and benchmarks
3. **Error Handling** - Test various failure scenarios
4. **Security Testing** - Validate security measures
5. **End-to-End Testing** - Complete browser automation tests

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review the generated test report
3. Check logs for error messages
4. Verify all prerequisites are met
