#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('� Setting up Playwright browsers and stealth plugin...');

try {
  // Install dependencies
  console.log('� Installing Node.js dependencies...');
  execSync('npm install', { stdio: 'inherit' });

  // Install Playwright browsers
  console.log('� Installing Playwright browsers...');
  execSync('npx playwright install chromium', { stdio: 'inherit' });

  // Install stealth plugin
  console.log('� Installing stealth plugin...');
  execSync('npm install puppeteer-extra-plugin-stealth', { stdio: 'inherit' });

  console.log('✅ Browser setup completed successfully!');
  console.log('� You can now run: npm start');
  
} catch (error) {
  console.error('❌ Error during browser setup:', error.message);
  process.exit(1);
}
