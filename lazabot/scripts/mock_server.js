const express = require('express');
const cors = require('cors');
const app = express();
const port = 3001;

app.use(cors());
app.use(express.json());

let productState = {
    id: 'smoke-test-product',
    name: 'Test Product',
    price: 100.00,
    stock: 0,
    isAvailable: false,
    isFlashSale: false,
    flashSalePrice: 50.00
};

// Trigger flash sale after 5 seconds
setTimeout(() => {
    console.log('🔥 Triggering flash sale!');
    productState.stock = 10;
    productState.isAvailable = true;
    productState.isFlashSale = true;
}, 5000);

app.get('/health', (req, res) => {
    res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

app.get('/api/products/:id', (req, res) => {
    const { id } = req.params;
    if (id !== productState.id) {
        return res.status(404).json({ error: 'Product not found' });
    }
    res.json({
        id: productState.id,
        name: productState.name,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        stock: productState.stock,
        isAvailable: productState.isAvailable,
        isFlashSale: productState.isFlashSale,
        timestamp: new Date().toISOString()
    });
});

app.post('/api/cart/add', (req, res) => {
    const { productId, quantity = 1 } = req.body;
    if (productId !== productState.id) {
        return res.status(404).json({ error: 'Product not found' });
    }
    if (!productState.isAvailable || productState.stock < quantity) {
        return res.status(400).json({ error: 'Product not available' });
    }
    const cartId = `cart_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    res.json({
        success: true,
        cartId: cartId,
        productId: productId,
        quantity: quantity,
        price: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

app.post('/api/checkout', (req, res) => {
    const { cartId } = req.body;
    if (!cartId) {
        return res.status(400).json({ error: 'Cart ID required' });
    }
    const orderId = `order_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    res.json({
        success: true,
        orderId: orderId,
        cartId: cartId,
        status: 'pending',
        totalAmount: productState.isFlashSale ? productState.flashSalePrice : productState.price,
        timestamp: new Date().toISOString()
    });
});

app.listen(port, () => {
    console.log(`🚀 Mock server running on port ${port}`);
});

process.on('SIGINT', () => {
    console.log('\n🛑 Shutting down...');
    process.exit(0);
});
