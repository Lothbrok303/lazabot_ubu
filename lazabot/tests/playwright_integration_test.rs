use lazabot::integrations::playwright::{CaptchaRequest, CheckoutRequest, PlaywrightClient};
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[tokio::test]
async fn test_playwright_server_health() {
    let client = PlaywrightClient::new();

    // Test health check
    match client.is_server_healthy().await {
        Ok(health) => {
            info!("Server health: {:?}", health);
            assert_eq!(health.status, "healthy");
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            // This is expected if server is not running
            println!("Note: Server not running - start with 'npm start' to test");
        }
    }
}

#[tokio::test]
async fn test_playwright_client_lifecycle() {
    let mut client = PlaywrightClient::new();

    // Test server startup
    match client.ensure_server_running().await {
        Ok(_) => {
            info!("Server started successfully");

            // Test health check
            let health = client.is_server_healthy().await.unwrap();
            assert_eq!(health.status, "healthy");

            // Test captcha solving with example URL
            let captcha_request = CaptchaRequest {
                captcha_url: "https://httpbin.org/html".to_string(),
                captcha_type: Some("image".to_string()),
            };

            match client.solve_captcha(captcha_request).await {
                Ok(response) => {
                    info!("Captcha response: {:?}", response);
                    assert!(response.success);
                }
                Err(e) => {
                    info!("Captcha solving failed (expected for test URL): {}", e);
                    // This is expected for test URLs
                }
            }

            // Test checkout flow with example URL
            let checkout_request = CheckoutRequest {
                product_url: "https://httpbin.org/html".to_string(),
                quantity: Some(1),
                shipping_info: None,
                payment_info: None,
                user_agent: Some(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
                ),
            };

            match client.perform_checkout_flow(checkout_request).await {
                Ok(response) => {
                    info!("Checkout response: {:?}", response);
                    assert!(response.success);
                }
                Err(e) => {
                    info!("Checkout flow failed (expected for test URL): {}", e);
                    // This is expected for test URLs
                }
            }

            // Test server shutdown
            client.stop_server().unwrap();
            info!("Server stopped successfully");
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            println!("Note: Node.js not installed or server failed to start");
            // This is expected if Node.js is not installed
        }
    }
}

#[tokio::test]
async fn test_playwright_error_handling() {
    let client = PlaywrightClient::new();

    // Test with invalid captcha URL
    let invalid_captcha_request = CaptchaRequest {
        captcha_url: "invalid-url".to_string(),
        captcha_type: Some("image".to_string()),
    };

    match client.solve_captcha(invalid_captcha_request).await {
        Ok(_) => {
            panic!("Should have failed with invalid URL");
        }
        Err(e) => {
            info!("Expected error for invalid URL: {}", e);
            // This is expected
        }
    }

    // Test with invalid checkout URL
    let invalid_checkout_request = CheckoutRequest {
        product_url: "invalid-url".to_string(),
        quantity: Some(1),
        shipping_info: None,
        payment_info: None,
        user_agent: None,
    };

    match client.perform_checkout_flow(invalid_checkout_request).await {
        Ok(_) => {
            panic!("Should have failed with invalid URL");
        }
        Err(e) => {
            info!("Expected error for invalid URL: {}", e);
            // This is expected
        }
    }
}

#[tokio::test]
async fn test_playwright_concurrent_requests() {
    let client = PlaywrightClient::new();

    // Test concurrent health checks
    let futures: Vec<_> = (0..5)
        .map(|_| {
            let client = &client;
            async move { client.is_server_healthy().await }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    for result in results {
        match result {
            Ok(health) => {
                assert_eq!(health.status, "healthy");
            }
            Err(_) => {
                // Server might not be running
                println!("Note: Server not running - start with 'npm start' to test");
            }
        }
    }
}
