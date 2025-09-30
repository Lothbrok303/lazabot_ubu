#!/bin/bash

echo "=== Lazabot Local Deployment Test ==="
echo

# Test 1: Check if services exist
echo "1. Checking systemd services..."
if systemctl list-unit-files | grep -q lazabot; then
    echo "   ✓ Lazabot services found"
else
    echo "   ✗ Lazabot services not found"
fi

# Test 2: Check service status
echo "2. Checking service status..."
LAZABOT_STATUS=$(systemctl is-active lazabot.service 2>/dev/null || echo "inactive")
PLAYWRIGHT_STATUS=$(systemctl is-active lazabot-playwright.service 2>/dev/null || echo "inactive")

echo "   Lazabot service: $LAZABOT_STATUS"
echo "   Playwright service: $PLAYWRIGHT_STATUS"

# Test 3: Check application directory
echo "3. Checking application directory..."
if [ -d "/opt/lazabot" ]; then
    echo "   ✓ Application directory exists"
    ls -la /opt/lazabot/
else
    echo "   ✗ Application directory not found"
fi

# Test 4: Check health endpoint
echo "4. Testing health endpoint..."
if curl -s http://localhost:8081/health > /dev/null; then
    echo "   ✓ Health endpoint responding"
    curl -s http://localhost:8081/health | jq . 2>/dev/null || curl -s http://localhost:8081/health
else
    echo "   ✗ Health endpoint not responding"
fi

# Test 5: Check ports
echo "5. Checking ports..."
if netstat -tlnp | grep -q ":8081"; then
    echo "   ✓ Port 8081 is listening"
else
    echo "   ✗ Port 8081 is not listening"
fi

echo
echo "=== Test Complete ==="
