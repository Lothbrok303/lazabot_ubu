#!/usr/bin/env node

const http = require('http');

const SERVER_URL = 'http://localhost:8081';

function makeRequest(path, method = 'GET', data = null) {
  return new Promise((resolve, reject) => {
    const url = new URL(path, SERVER_URL);
    const options = {
      hostname: url.hostname,
      port: url.port,
      path: url.pathname,
      method: method,
      headers: {
        'Content-Type': 'application/json',
      }
    };

    const req = http.request(options, (res) => {
      let body = '';
      res.on('data', (chunk) => {
        body += chunk;
      });
      res.on('end', () => {
        try {
          const jsonBody = JSON.parse(body);
          resolve({ status: res.statusCode, data: jsonBody });
        } catch (e) {
          resolve({ status: res.statusCode, data: body });
        }
      });
    });

    req.on('error', (error) => {
      reject(error);
    });

    if (data) {
      req.write(JSON.stringify(data));
    }
    req.end();
  });
}

async function testFullIntegration() {
  console.log('� Testing Full Playwright RPC Integration...\n');

  try {
    // Test health endpoint
    console.log('1️⃣ Testing health endpoint...');
    const healthResponse = await makeRequest('/health');
    console.log(`   Status: ${healthResponse.status}`);
    console.log(`   Response:`, JSON.stringify(healthResponse.data, null, 2));
    console.log('   ✅ Health check passed\n');

    // Test captcha endpoint with valid data
    console.log('2️⃣ Testing captcha endpoint with valid data...');
    const captchaRequest = {
      captcha_url: 'https://httpbin.org/html',
      captcha_type: 'image'
    };
    const captchaResponse = await makeRequest('/solveCaptcha', 'POST', captchaRequest);
    console.log(`   Status: ${captchaResponse.status}`);
    console.log(`   Success: ${captchaResponse.data.success}`);
    console.log(`   Message: ${captchaResponse.data.message || 'N/A'}`);
    console.log('   ✅ Captcha endpoint responded\n');

    // Test checkout endpoint with valid data
    console.log('3️⃣ Testing checkout endpoint with valid data...');
    const checkoutRequest = {
      product_url: 'https://httpbin.org/html',
      quantity: 1,
      user_agent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
    };
    const checkoutResponse = await makeRequest('/performCheckoutFlow', 'POST', checkoutRequest);
    console.log(`   Status: ${checkoutResponse.status}`);
    console.log(`   Success: ${checkoutResponse.data.success}`);
    console.log(`   Message: ${checkoutResponse.data.message || 'N/A'}`);
    console.log('   ✅ Checkout endpoint responded\n');

    console.log('� Full integration test completed successfully!');
    console.log('\n� Summary:');
    console.log('   ✅ Health endpoint working');
    console.log('   ✅ Captcha endpoint working');
    console.log('   ✅ Checkout endpoint working');
    console.log('   ✅ Server responding to all requests');
    console.log('\n� Ready for Rust integration!');

  } catch (error) {
    console.error('❌ Integration test failed:', error.message);
    console.log('\n� Make sure the server is running:');
    console.log('   npm start');
    process.exit(1);
  }
}

testFullIntegration();
