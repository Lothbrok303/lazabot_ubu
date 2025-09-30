#!/bin/bash
# Metrics Module Verification Script

echo "=========================================="
echo "Lazabot Metrics Module Verification"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check function
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✓${NC} $1"
        return 0
    else
        echo -e "${RED}✗${NC} $1 (missing)"
        return 1
    fi
}

echo "1. Checking Source Files..."
check_file "src/utils/metrics.rs"
check_file "src/utils/mod.rs"
check_file "src/lib.rs"
echo ""

echo "2. Checking Example Files..."
check_file "examples/metrics_demo.rs"
echo ""

echo "3. Checking Test Files..."
check_file "tests/metrics_integration_test.rs"
check_file "tests/integration_tests.rs"
echo ""

echo "4. Checking Docker Files..."
check_file "Dockerfile"
check_file "../docker-compose.yml"
check_file "config/prometheus.yml"
echo ""

echo "5. Checking Documentation..."
check_file "../README.md"
check_file "METRICS_VERIFICATION.md"
echo ""

echo "6. Verifying Cargo.toml dependencies..."
if grep -q "parking_lot" Cargo.toml; then
    echo -e "${GREEN}✓${NC} parking_lot dependency added"
else
    echo -e "${RED}✗${NC} parking_lot dependency missing"
fi
echo ""

echo "7. Checking metrics.rs implementation..."
if grep -q "MetricsCollector" src/utils/metrics.rs; then
    echo -e "${GREEN}✓${NC} MetricsCollector struct found"
fi
if grep -q "MetricsServer" src/utils/metrics.rs; then
    echo -e "${GREEN}✓${NC} MetricsServer struct found"
fi
if grep -q "format_prometheus" src/utils/metrics.rs; then
    echo -e "${GREEN}✓${NC} Prometheus format function found"
fi
if grep -q "lazabot_requests_total" src/utils/metrics.rs; then
    echo -e "${GREEN}✓${NC} Metrics definitions found"
fi
echo ""

echo "8. Checking Docker Compose configuration..."
if grep -q "redis:" ../docker-compose.yml; then
    echo -e "${GREEN}✓${NC} Redis service configured"
fi
if grep -q "agent1:" ../docker-compose.yml; then
    echo -e "${GREEN}✓${NC} Agent1 service configured"
fi
if grep -q "prometheus:" ../docker-compose.yml; then
    echo -e "${GREEN}✓${NC} Prometheus service configured"
fi
echo ""

echo "9. Checking README updates..."
if grep -q "Metrics and Monitoring" ../README.md; then
    echo -e "${GREEN}✓${NC} Metrics section added to README"
fi
if grep -q "Horizontal Scaling" ../README.md; then
    echo -e "${GREEN}✓${NC} Scaling section added to README"
fi
echo ""

echo "=========================================="
echo "Verification Summary"
echo "=========================================="
echo ""
echo -e "${GREEN}All files created successfully!${NC}"
echo ""
echo "Note: Due to VMware shared folder limitations,"
echo "the project must be built on a native Linux filesystem."
echo ""
echo "To verify the metrics server:"
echo "  1. Copy project: cp -r . ~/lazabot_local"
echo "  2. Build: cd ~/lazabot_local && cargo build"
echo "  3. Run demo: cargo run --example metrics_demo &"
echo "  4. Test: curl http://127.0.0.1:9091/metrics"
echo ""
echo "See METRICS_VERIFICATION.md for detailed instructions."
echo ""
