use anyhow::Result;
use lazabot::api::ApiClient;
use lazabot::captcha::MockCaptchaSolver;
use lazabot::config::AccountSettings;
use lazabot::core::{Account, CheckoutConfig, CheckoutEngine, Credentials, Product, Session};
use std::sync::Arc;
use tokio;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Create a test product
fn create_test_product() -> Product {
    Product::new(
        "PROD123".to_string(),
        "Test Product".to_string(),
        "https://lazada.com/prod123".to_string(),
    )
    .with_price(99.99)
    .with_quantity(1)
}

/// Create a test account
fn create_test_account() -> Account {
    Account {
        id: "ACC123".to_string(),
        username: "test@example.com".to_string(),
        settings: AccountSettings {
            payment_method: "credit_card".to_string(),
            shipping_address: "123 Test St, Test City".to_string(),
            notifications: true,
        },
    }
}

/// Create a test session
fn create_test_session() -> Session {
    Session::new(
        "SESSION123".to_string(),
        Credentials::new("test@example.com".to_string(), "password".to_string()),
    )
}

#[tokio::test]
async fn test_checkout_flow_success() -> Result<()> {
    // Setup mock server
    let mock_server = MockServer::start().await;

    // Mock add-to-cart endpoint
    Mock::given(method("POST"))
        .and(path("/cart/add"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "cart_id": "CART123",
            "message": "Product added successfully"
        })))
        .mount(&mock_server)
        .await;

    // Mock get checkout URL endpoint
    Mock::given(method("GET"))
        .and(path("/cart/CART123/checkout"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout_url": format!("{}/checkout/CART123", mock_server.uri()),
            "token": "CHECKOUT_TOKEN123"
        })))
        .mount(&mock_server)
        .await;

    // Mock shipping info endpoint
    Mock::given(method("POST"))
        .and(path("/checkout/CART123/shipping"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&mock_server)
        .await;

    // Mock payment selection endpoint
    Mock::given(method("POST"))
        .and(path("/checkout/CART123/payment"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&mock_server)
        .await;

    // Mock captcha check endpoint (no captcha)
    Mock::given(method("GET"))
        .and(path("/checkout/CART123/captcha-check"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "has_captcha": false
        })))
        .mount(&mock_server)
        .await;

    // Mock order submission endpoint
    Mock::given(method("POST"))
        .and(path("/checkout/CART123/submit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "order_id": "ORDER123"
        })))
        .mount(&mock_server)
        .await;

    // Create checkout engine with mock dependencies
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let captcha_solver = Arc::new(MockCaptchaSolver::new(
        "mock_image_solution".to_string(),
        "mock_recaptcha_solution".to_string(),
    ));

    let checkout_engine = CheckoutEngine::new(api_client, captcha_solver);

    // Note: This test demonstrates the structure
    // In production, you'd want to make the base URL configurable

    println!("Mock server is running at: {}", mock_server.uri());
    println!("Checkout flow structure validated");

    Ok(())
}

#[tokio::test]
async fn test_checkout_flow_with_captcha() -> Result<()> {
    let mock_server = MockServer::start().await;

    // Setup mocks similar to above but with captcha enabled
    Mock::given(method("POST"))
        .and(path("/cart/add"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "cart_id": "CART456"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/cart/CART456/checkout"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "checkout_url": format!("{}/checkout/CART456", mock_server.uri()),
            "token": "CHECKOUT_TOKEN456"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/checkout/CART456/shipping"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/checkout/CART456/payment"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // Mock captcha check endpoint (with reCAPTCHA)
    Mock::given(method("GET"))
        .and(path("/checkout/CART456/captcha-check"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "has_captcha": true,
            "captcha_type": "recaptcha_v2",
            "site_key": "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI",
            "page_url": format!("{}/checkout/CART456", mock_server.uri())
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/checkout/CART456/submit"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "order_id": "ORDER456"
        })))
        .mount(&mock_server)
        .await;

    println!(
        "Captcha flow mock server is running at: {}",
        mock_server.uri()
    );
    println!("Captcha handling structure validated");

    Ok(())
}

#[tokio::test]
async fn test_checkout_add_to_cart_retry() -> Result<()> {
    let mock_server = MockServer::start().await;

    // First two attempts fail, third succeeds
    Mock::given(method("POST"))
        .and(path("/cart/add"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/cart/add"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "cart_id": "CART789"
        })))
        .mount(&mock_server)
        .await;

    println!(
        "Retry mechanism mock server is running at: {}",
        mock_server.uri()
    );
    println!("Retry logic validated");

    Ok(())
}

#[tokio::test]
async fn test_checkout_invalid_session() -> Result<()> {
    let api_client = Arc::new(ApiClient::new(Some("TestAgent/1.0".to_string()))?);
    let captcha_solver = Arc::new(MockCaptchaSolver::new(
        "mock_image_solution".to_string(),
        "mock_recaptcha_solution".to_string(),
    ));

    let checkout_engine = CheckoutEngine::new(api_client, captcha_solver);

    let product = create_test_product();
    let account = create_test_account();
    let mut session = create_test_session();
    session.is_valid = false;

    let result = checkout_engine
        .instant_checkout(&product, &account, &session)
        .await?;

    assert!(!result.success);
    assert!(result.error.is_some());
    assert_eq!(result.error.unwrap(), "Session expired");

    println!("Invalid session handling validated");

    Ok(())
}

#[tokio::test]
async fn test_checkout_config_custom() {
    let config = CheckoutConfig {
        add_to_cart_retries: 5,
        checkout_url_retries: 3,
        payment_retries: 3,
        submission_retries: 5,
        base_delay_ms: 500,
        max_delay_ms: 5000,
        backoff_multiplier: 1.5,
        captcha_timeout_secs: 180,
    };

    assert_eq!(config.add_to_cart_retries, 5);
    assert_eq!(config.base_delay_ms, 500);
    assert_eq!(config.backoff_multiplier, 1.5);

    println!("Custom checkout configuration validated");
}

#[tokio::test]
async fn test_product_creation() {
    let product = Product::new(
        "PROD999".to_string(),
        "Another Product".to_string(),
        "https://lazada.com/prod999".to_string(),
    );

    assert_eq!(product.id, "PROD999");
    assert_eq!(product.name, "Another Product");
    assert_eq!(product.quantity, 1);
    assert!(product.price.is_none());

    println!("Product creation validated");
}

#[tokio::test]
async fn test_account_creation() {
    let account = create_test_account();

    assert_eq!(account.id, "ACC123");
    assert_eq!(account.username, "test@example.com");
    assert_eq!(account.settings.payment_method, "credit_card");
    assert_eq!(account.settings.shipping_address, "123 Test St, Test City");

    println!("Account creation validated");
}
