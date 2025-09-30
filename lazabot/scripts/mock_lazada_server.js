#!/usr/bin/env node

const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Product state - starts as unavailable
let productState = {
    id: 'smoke-test-product',
    name: 'Smoke Test Product',
    price: 100.00,
    flashSalePrice: 50.00,
    stock: 0,
    isAvailable: false,
    isFlashSale: false,
    lastUpdated: new Date().toISOString()
};

// Order storage
let orders = [];
let orderCounter = 1;

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'healthy',
        timestamp: new Date().toISOString(),
        server: 'Mock Lazada API Server'
    });
});

// Product details endpoint - returns HTML-like response for monitoring
app.get('/api/products/:id', (req, res) => {
    const { id } = req.params;
    
    if (id !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId: id
        });
    }
    
    // Return HTML-like response that the monitor can parse
    const html = `
<!DOCTYPE html>
<html>
<head><title>${productState.name}</title></head>
<body>
    <div class="product-info">
        <h1>${productState.name}</h1>
        <div class="price">$${productState.isFlashSale ? productState.flashSalePrice : productState.price}</div>
        <div class="stock">Stock: ${productState.stock}</div>
        <div class="availability">${productState.isAvailable ? 'Available' : 'Out of Stock'}</div>
        ${productState.isAvailable ? '<button class="add-to-cart">Add to Cart</button>' : ''}
    </div>
</body>
</html>`;
    
    res.setHeader('Content-Type', 'text/html');
    res.send(html);
});

// Product availability check endpoint
app.get('/api/products/:id/availability', (req, res) => {
    const { id } = req.params;
    
    if (id !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId: id
        });
    }
    
    res.json({
        success: true,
        available: productState.isAvailable,
        stock: productState.stock,
        isFlashSale: productState.isFlashSale,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

// Checkout endpoint
app.post('/api/checkout', (req, res) => {
    const { productId, quantity = 1, accountId = 'test-account' } = req.body;
    
    if (productId !== productState.id) {
        return res.status(404).json({
            error: 'Product not found',
            productId
        });
    }
    
    if (!productState.isAvailable) {
        return res.status(400).json({
            error: 'Product not available',
            available: false
        });
    }
    
    if (productState.stock < quantity) {
        return res.status(400).json({
            error: 'Insufficient stock',
            available: productState.stock,
            requested: quantity
        });
    }
    
    // Create order
    const orderId = `order_${orderCounter++}_${Date.now()}`;
    const order = {
        orderId,
        productId,
        accountId,
        quantity,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        status: 'pending',
        createdAt: new Date().toISOString()
    };
    
    orders.push(order);
    
    // Update stock
    productState.stock -= quantity;
    if (productState.stock <= 0) {
        productState.isAvailable = false;
    }
    
    console.log(`[${new Date().toISOString()}] 🛒 CHECKOUT TRIGGERED! Order: ${orderId}, Product: ${productId}, Quantity: ${quantity}, Price: ${order.price}`);
    
    res.json({
        success: true,
        orderId,
        message: 'Order created successfully',
        order,
        timestamp: new Date().toISOString()
    });
});

// Trigger flash sale endpoint (for testing)
app.post('/api/test/flash-sale', (req, res) => {
    productState.stock = 10;
    productState.isAvailable = true;
    productState.isFlashSale = true;
    productState.lastUpdated = new Date().toISOString();
    
    console.log(`[${new Date().toISOString()}] 🔥 FLASH SALE TRIGGERED! Product now available with ${productState.stock} stock at ${productState.flashSalePrice} (was ${productState.price})`);
    
    res.json({
        success: true,
        message: 'Flash sale triggered',
        productState,
        timestamp: new Date().toISOString()
    });
});

// Reset product state endpoint
app.post('/api/test/reset', (req, res) => {
    productState.stock = 0;
    productState.isAvailable = false;
    productState.isFlashSale = false;
    productState.lastUpdated = new Date().toISOString();
    
    console.log(`[${new Date().toISOString()}] 🔄 Product state reset`);
    
    res.json({
        success: true,
        message: 'Product state reset',
        productState,
        timestamp: new Date().toISOString()
    });
});

// Get orders endpoint
app.get('/api/orders', (req, res) => {
    res.json({
        success: true,
        orders,
        count: orders.length,
        timestamp: new Date().toISOString()
    });
});

// Start server
app.listen(PORT, () => {
    console.log(`[${new Date().toISOString()}] 🚀 Mock Lazada API Server running on port ${PORT}`);
    console.log(`[${new Date().toISOString()}] 📍 Health check: http://localhost:${PORT}/health`);
    console.log(`[${new Date().toISOString()}] 📦 Product endpoint: http://localhost:${PORT}/api/products/smoke-test-product`);
    console.log(`[${new Date().toISOString()}] 🔥 Flash sale trigger: http://localhost:${PORT}/api/test/flash-sale`);
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log(`[${new Date().toISOString()}] 🛑 Shutting down mock server...`);
    process.exit(0);
});

process.on('SIGTERM', () => {
    console.log(`[${new Date().toISOString()}] 🛑 Shutting down mock server...`);
    process.exit(0);
});
