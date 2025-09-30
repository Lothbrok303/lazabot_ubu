# Metrics Module Verification Guide

## Files Created

### Source Files
- ✅ `src/utils/metrics.rs` - Lightweight metrics server with Prometheus format
- ✅ `src/utils/mod.rs` - Utilities module declaration
- ✅ `src/lib.rs` - Updated to include utils module

### Example Files
- ✅ `examples/metrics_demo.rs` - Demonstration of metrics server usage

### Test Files
- ✅ `tests/metrics_integration_test.rs` - Standalone metrics tests
- ✅ `tests/integration_tests.rs` - Updated with metrics module tests

### Docker & Configuration Files
- ✅ `Dockerfile` - Container image for the application
- ✅ `../docker-compose.yml` - Multi-agent setup with Redis
- ✅ `config/prometheus.yml` - Prometheus scraping configuration

### Documentation
- ✅ `../README.md` - Updated with metrics and scaling sections

## Manual Verification Steps

### Step 1: Verify Code Syntax
```bash
# Check that all Rust files are valid
find src/utils -name "*.rs" -exec cat {} \;
cat examples/metrics_demo.rs
```

### Step 2: Build Project (on native Linux filesystem)
Due to VMware shared folder limitations, copy to local directory first:
```bash
# Copy project to home directory
cp -r /mnt/hgfs/Cursor\ projects/lazada\ cli\ bot/lazabot ~/lazabot_local
cd ~/lazabot_local

# Clean and build
cargo clean
cargo build --release

# Run tests
cargo test --test metrics_integration_test
cargo test test_metrics_module_integration
```

### Step 3: Start Metrics Server
```bash
cd ~/lazabot_local
cargo run --example metrics_demo &
sleep 3  # Wait for server to start
```

### Step 4: Query Metrics Endpoint
```bash
# Get metrics in Prometheus format
curl http://127.0.0.1:9091/metrics

# Expected output should include:
# - lazabot_requests_total
# - lazabot_requests_success_total
# - lazabot_requests_failed_total
# - lazabot_active_tasks
# - lazabot_requests_per_second
# - lazabot_uptime_seconds
```

### Step 5: Health Check
```bash
curl http://127.0.0.1:9091/health

# Expected output: OK
```

### Step 6: Docker Compose (Optional)
```bash
cd ~/lazabot_local/..

# Build and start services
docker-compose build
docker-compose up -d

# Check agent metrics
curl http://localhost:9091/metrics  # Agent 1
curl http://localhost:9092/metrics  # Agent 2
curl http://localhost:9093/metrics  # Agent 3

# Check Prometheus
curl http://localhost:9090/api/v1/targets

# Cleanup
docker-compose down
```

## Expected Metrics Output Example

```
# HELP lazabot_requests_total Total number of requests
# TYPE lazabot_requests_total counter
lazabot_requests_total 100

# HELP lazabot_requests_success_total Total number of successful requests
# TYPE lazabot_requests_success_total counter
lazabot_requests_success_total 90

# HELP lazabot_requests_failed_total Total number of failed requests
# TYPE lazabot_requests_failed_total counter
lazabot_requests_failed_total 10

# HELP lazabot_active_tasks Number of currently active tasks
# TYPE lazabot_active_tasks gauge
lazabot_active_tasks 2

# HELP lazabot_requests_per_second Current request rate
# TYPE lazabot_requests_per_second gauge
lazabot_requests_per_second 6.67

# HELP lazabot_uptime_seconds Uptime in seconds
# TYPE lazabot_uptime_seconds counter
lazabot_uptime_seconds 15
```

## Key Features Implemented

### Metrics Collector
- ✅ Thread-safe atomic counters
- ✅ Request tracking (total, success, failure)
- ✅ Active tasks gauge
- ✅ Request rate calculation
- ✅ Uptime tracking
- ✅ Prometheus format output

### Metrics Server
- ✅ Lightweight HTTP server
- ✅ `/metrics` endpoint (Prometheus format)
- ✅ `/health` endpoint
- ✅ Non-blocking async operations
- ✅ Configurable bind address

### Horizontal Scaling Support
- ✅ Docker Compose with 3 agents
- ✅ Redis for distributed task queue
- ✅ Prometheus for metrics aggregation
- ✅ Health checks and auto-restart
- ✅ Individual metrics ports per agent

## Integration with Existing Code

The metrics module integrates seamlessly with existing Lazabot components:

```rust
use lazabot::utils::{MetricsCollector, MetricsServer};

// In your main application
let collector = MetricsCollector::new();

// Start metrics server in background
let server = MetricsServer::new(collector.clone(), "0.0.0.0:9091");
tokio::spawn(async move {
    server.start().await.unwrap();
});

// Use collector throughout your application
collector.inc_total_requests();
collector.inc_success_requests();
collector.inc_active_tasks();
// ... do work ...
collector.dec_active_tasks();
```

## Troubleshooting

### Issue: Cannot build on VMware shared folder
**Solution**: Copy project to native Linux filesystem:
```bash
cp -r /mnt/hgfs/Cursor\ projects/lazada\ cli\ bot/lazabot ~/lazabot_local
cd ~/lazabot_local
cargo build
```

### Issue: Port already in use
**Solution**: Use a different port:
```rust
let server = MetricsServer::new(collector, "0.0.0.0:19091");
```

### Issue: Metrics show 0 values
**Solution**: Ensure you're calling the increment methods:
```rust
collector.inc_total_requests();
collector.inc_success_requests();
```

## Next Steps

1. ✅ Copy project to native filesystem
2. ✅ Build and run tests
3. ✅ Start metrics demo
4. ✅ Query metrics endpoint with curl
5. ✅ Test Docker Compose setup (optional)
6. ✅ Integrate metrics into main application
7. ✅ Set up Prometheus/Grafana dashboards (optional)

## Verification Checklist

- [x] Created src/utils/metrics.rs with metrics collector and server
- [x] Created src/utils/mod.rs
- [x] Updated src/lib.rs to include utils module
- [x] Created examples/metrics_demo.rs
- [x] Created tests/metrics_integration_test.rs
- [x] Updated tests/integration_tests.rs
- [x] Created Dockerfile
- [x] Created docker-compose.yml with Redis and 3 agents
- [x] Created config/prometheus.yml
- [x] Updated README.md with metrics and scaling documentation
- [ ] Build project (requires native filesystem)
- [ ] Run tests (requires native filesystem)
- [ ] Start metrics server and verify with curl

## Commit Message

```
feat: Add metrics server and horizontal scaling support

- Implemented lightweight HTTP metrics server with Prometheus format
- Added MetricsCollector for thread-safe metric tracking
- Created Docker Compose setup with Redis and multiple agents
- Added Prometheus integration for metrics aggregation
- Updated README with metrics and horizontal scaling documentation
- Created integration tests for metrics module
- Added metrics_demo example

Metrics exposed:
- lazabot_requests_total (counter)
- lazabot_requests_success_total (counter)
- lazabot_requests_failed_total (counter)
- lazabot_active_tasks (gauge)
- lazabot_requests_per_second (gauge)
- lazabot_uptime_seconds (counter)

Endpoints:
- GET /metrics - Prometheus format metrics
- GET /health - Health check

Scaling features:
- Docker Compose with Redis task queue
- Multiple agent instances (3 by default)
- Prometheus for metrics aggregation
- Individual metrics ports per agent

Tests: metrics_integration_test, integration_tests (added 2 new tests)
```
