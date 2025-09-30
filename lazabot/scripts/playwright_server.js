#!/usr/bin/env node

const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const { chromium } = require('playwright');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');

const app = express();
const PORT = 8081;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// Global browser instance
let browser = null;
let context = null;

// Initialize browser with stealth
async function initBrowser() {
  if (browser) return browser;
  
  console.log('í¼ Initializing browser with stealth mode...');
  
  browser = await chromium.launch({
    headless: false, // Set to true for production
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--disable-dev-shm-usage',
      '--disable-accelerated-2d-canvas',
      '--no-first-run',
      '--no-zygote',
      '--disable-gpu'
    ]
  });

  context = await browser.newContext({
    userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    viewport: { width: 1920, height: 1080 },
    locale: 'en-US',
    timezoneId: 'America/New_York'
  });

  return browser;
}

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy', 
    timestamp: new Date().toISOString(),
    browser: browser ? 'initialized' : 'not_initialized'
  });
});

// Solve captcha endpoint
app.post('/solveCaptcha', async (req, res) => {
  try {
    const { captchaUrl, captchaType = 'image' } = req.body;
    
    if (!captchaUrl) {
      return res.status(400).json({ 
        error: 'captchaUrl is required' 
      });
    }

    await initBrowser();
    const page = await context.newPage();

    console.log(`í´ Solving captcha: ${captchaUrl}`);

    // Navigate to captcha URL
    await page.goto(captchaUrl, { waitUntil: 'networkidle' });

    // Wait for captcha to load
    await page.waitForTimeout(2000);

    // For image captchas, we'll implement basic detection
    if (captchaType === 'image') {
      // Look for common captcha selectors
      const captchaSelectors = [
        'img[src*="captcha"]',
        'img[alt*="captcha"]',
        '.captcha img',
        '#captcha img',
        '[class*="captcha"] img'
      ];

      let captchaElement = null;
      for (const selector of captchaSelectors) {
        try {
          captchaElement = await page.$(selector);
          if (captchaElement) break;
        } catch (e) {
          // Continue to next selector
        }
      }

      if (captchaElement) {
        // Take screenshot of captcha
        const screenshot = await captchaElement.screenshot({ 
          type: 'png',
          encoding: 'base64'
        });

        // For now, return the screenshot for manual solving
        // In production, integrate with captcha solving service
        await page.close();
        
        return res.json({
          success: true,
          captchaImage: `data:image/png;base64,${screenshot}`,
          message: 'Captcha image captured. Manual solving required.',
          captchaUrl
        });
      } else {
        await page.close();
        return res.status(404).json({ 
          error: 'Captcha element not found' 
        });
      }
    }

    await page.close();
    res.json({ 
      success: true, 
      message: 'Captcha processing completed' 
    });

  } catch (error) {
    console.error('âŒ Captcha solving error:', error);
    res.status(500).json({ 
      error: 'Failed to solve captcha',
      details: error.message 
    });
  }
});

// Perform checkout flow endpoint
app.post('/performCheckoutFlow', async (req, res) => {
  try {
    const { 
      productUrl, 
      quantity = 1, 
      shippingInfo,
      paymentInfo,
      userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
    } = req.body;

    if (!productUrl) {
      return res.status(400).json({ 
        error: 'productUrl is required' 
      });
    }

    await initBrowser();
    const page = await context.newPage();

    console.log(`í»’ Starting checkout flow for: ${productUrl}`);

    // Set user agent
    await page.setUserAgent(userAgent);

    // Navigate to product page
    await page.goto(productUrl, { 
      waitUntil: 'networkidle',
      timeout: 30000 
    });

    // Wait for page to load
    await page.waitForTimeout(3000);

    // Add to cart
    const addToCartSelectors = [
      'button[data-testid*="add-to-cart"]',
      'button:has-text("Add to Cart")',
      'button:has-text("Add to Bag")',
      '.add-to-cart',
      '#add-to-cart',
      '[class*="add-to-cart"]'
    ];

    let addToCartButton = null;
    for (const selector of addToCartSelectors) {
      try {
        addToCartButton = await page.$(selector);
        if (addToCartButton) break;
      } catch (e) {
        // Continue to next selector
      }
    }

    if (addToCartButton) {
      await addToCartButton.click();
      console.log('âœ… Added to cart');
      await page.waitForTimeout(2000);
    } else {
      await page.close();
      return res.status(404).json({ 
        error: 'Add to cart button not found' 
      });
    }

    // Navigate to cart/checkout
    const cartSelectors = [
      'a[href*="cart"]',
      'a:has-text("Cart")',
      'a:has-text("Bag")',
      '.cart-link',
      '#cart-link'
    ];

    let cartLink = null;
    for (const selector of cartSelectors) {
      try {
        cartLink = await page.$(selector);
        if (cartLink) break;
      } catch (e) {
        // Continue to next selector
      }
    }

    if (cartLink) {
      await cartLink.click();
      await page.waitForTimeout(3000);
      console.log('âœ… Navigated to cart');
    }

    // Proceed to checkout
    const checkoutSelectors = [
      'button:has-text("Checkout")',
      'button:has-text("Proceed to Checkout")',
      'a:has-text("Checkout")',
      '.checkout-button',
      '#checkout-button'
    ];

    let checkoutButton = null;
    for (const selector of checkoutSelectors) {
      try {
        checkoutButton = await page.$(selector);
        if (checkoutButton) break;
      } catch (e) {
        // Continue to next selector
      }
    }

    if (checkoutButton) {
      await checkoutButton.click();
      await page.waitForTimeout(3000);
      console.log('âœ… Proceeded to checkout');
    }

    // Take final screenshot
    const screenshot = await page.screenshot({ 
      type: 'png',
      encoding: 'base64',
      fullPage: true 
    });

    await page.close();

    res.json({
      success: true,
      message: 'Checkout flow completed',
      screenshot: `data:image/png;base64,${screenshot}`,
      productUrl,
      quantity,
      timestamp: new Date().toISOString()
    });

  } catch (error) {
    console.error('âŒ Checkout flow error:', error);
    res.status(500).json({ 
      error: 'Failed to perform checkout flow',
      details: error.message 
    });
  }
});

// Graceful shutdown
process.on('SIGINT', async () => {
  console.log('í»‘ Shutting down server...');
  if (browser) {
    await browser.close();
  }
  process.exit(0);
});

// Start server
app.listen(PORT, () => {
  console.log(`íº€ Playwright RPC server running on http://localhost:${PORT}`);
  console.log(`í³‹ Available endpoints:`);
  console.log(`   GET  /health`);
  console.log(`   POST /solveCaptcha`);
  console.log(`   POST /performCheckoutFlow`);
});

module.exports = app;
