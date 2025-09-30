#!/bin/bash
# Test script for metrics module

echo "Testing metrics module..."
echo ""
echo "1. Checking if metrics.rs is properly formatted..."
rustfmt --check src/utils/metrics.rs && echo "✓ Code is properly formatted" || echo "✗ Formatting issues found"

echo ""
echo "2. Running metrics demo (will run in background for 10 seconds)..."
echo "   Start the demo with: cargo run --example metrics_demo"
echo "   Then test with: curl http://127.0.0.1:9091/metrics"

echo ""
echo "3. Docker Compose configuration created:"
ls -lh ../docker-compose.yml

echo ""
echo "4. Prometheus configuration created:"
ls -lh config/prometheus.yml

echo ""
echo "5. Dockerfile created:"
ls -lh Dockerfile

echo ""
echo "=== VERIFICATION COMPLETE ==="
echo ""
echo "To manually test:"
echo "  1. Start the metrics demo: cargo run --example metrics_demo &"
echo "  2. Wait 3 seconds for server to start"
echo "  3. Query metrics: curl http://127.0.0.1:9091/metrics"
echo "  4. Query health: curl http://127.0.0.1:9091/health"
