#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Ì∫Ä Setting up Playwright browsers and stealth plugin...');

try {
  // Install dependencies
  console.log('Ì≥¶ Installing Node.js dependencies...');
  execSync('npm install', { stdio: 'inherit' });

  // Install Playwright browsers
  console.log('Ìºê Installing Playwright browsers...');
  execSync('npx playwright install chromium', { stdio: 'inherit' });

  // Install stealth plugin
  console.log('Ìµ∑ Installing stealth plugin...');
  execSync('npm install puppeteer-extra-plugin-stealth', { stdio: 'inherit' });

  console.log('‚úÖ Browser setup completed successfully!');
  console.log('Ì≥ù You can now run: npm start');
  
} catch (error) {
  console.error('‚ùå Error during browser setup:', error.message);
  process.exit(1);
}
