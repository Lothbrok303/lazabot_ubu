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

async function testServer() {
  console.log('Ì∑™ Testing Playwright RPC Server...\n');

  try {
    // Test health endpoint
    console.log('1Ô∏è‚É£ Testing health endpoint...');
    const healthResponse = await makeRequest('/health');
    console.log(`   Status: ${healthResponse.status}`);
    console.log(`   Response:`, JSON.stringify(healthResponse.data, null, 2));
    console.log('   ‚úÖ Health check passed\n');

    // Test captcha endpoint
    console.log('2Ô∏è‚É£ Testing captcha endpoint...');
    const captchaRequest = {
      captcha_url: 'https://example.com/captcha',
      captcha_type: 'image'
    };
    const captchaResponse = await makeRequest('/solveCaptcha', 'POST', captchaRequest);
    console.log(`   Status: ${captchaResponse.status}`);
    console.log(`   Response:`, JSON.stringify(captchaResponse.data, null, 2));
    console.log('   ‚úÖ Captcha endpoint responded\n');

    // Test checkout endpoint
    console.log('3Ô∏è‚É£ Testing checkout endpoint...');
    const checkoutRequest = {
      product_url: 'https://example.com/product',
      quantity: 1
    };
    const checkoutResponse = await makeRequest('/performCheckoutFlow', 'POST', checkoutRequest);
    console.log(`   Status: ${checkoutResponse.status}`);
    console.log(`   Response:`, JSON.stringify(checkoutResponse.data, null, 2));
    console.log('   ‚úÖ Checkout endpoint responded\n');

    console.log('Ìæâ All tests completed successfully!');

  } catch (error) {
    console.error('‚ùå Test failed:', error.message);
    console.log('\nÌ≤° Make sure the server is running:');
    console.log('   npm start');
    process.exit(1);
  }
}

testServer();
