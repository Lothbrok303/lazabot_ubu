use anyhow::Result;
use reqwest::Method;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use wiremock::{
    matchers::{method, path, header},
    Mock, MockServer, ResponseTemplate,
};

use lazabot::api::{ApiClient, ProxyInfo, RetryConfig};

#[tokio::test]
async fn test_api_client_get_request() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "status": "success",
                "data": {
                    "id": 123,
                    "name": "Test Product",
                    "price": 29.99
                }
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let response = client.request(
        Method::GET,
        &format!("{}/test", mock_server.uri()),
        None,
        None,
        None,
    ).await?;

    assert_eq!(response.status, 200);
    assert!(response.text.contains("Test Product"));
    assert!(response.text.contains("29.99"));
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_post_request() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/api/data"))
        .respond_with(ResponseTemplate::new(201)
            .set_body_json(json!({
                "id": 456,
                "created": true,
                "message": "Data created successfully"
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let request_body = json!({
        "name": "New Item",
        "value": 100
    }).to_string().into_bytes();
    
    let response = client.request(
        Method::POST,
        &format!("{}/api/data", mock_server.uri()),
        None,
        Some(request_body),
        None,
    ).await?;

    assert_eq!(response.status, 201);
    assert!(response.text.contains("created successfully"));
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_with_custom_headers() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/protected"))
        .and(header("authorization", "Bearer test-token"))
        .and(header("x-custom-header", "test-value"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "authenticated": true,
                "user": "test-user"
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("authorization", "Bearer test-token".parse()?);
    headers.insert("x-custom-header", "test-value".parse()?);
    
    let response = client.request(
        Method::GET,
        &format!("{}/protected", mock_server.uri()),
        Some(headers),
        None,
        None,
    ).await?;

    assert_eq!(response.status, 200);
    assert!(response.text.contains("authenticated"));
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_retry_mechanism() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/retry-test"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;
    
    Mock::given(method("GET"))
        .and(path("/retry-test"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "success": true,
                "attempt": 3
            })))
        .mount(&mock_server)
        .await;

    let retry_config = RetryConfig {
        max_retries: 3,
        base_delay_ms: 100,
        max_delay_ms: 1000,
        backoff_multiplier: 2.0,
    };
    
    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?
        .with_retry_config(retry_config);
    
    let response = client.request(
        Method::GET,
        &format!("{}/retry-test", mock_server.uri()),
        None,
        None,
        None,
    ).await?;

    assert_eq!(response.status, 500);
    assert!(response.text.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_with_proxy() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/proxy-test"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "proxy_used": true,
                "message": "Request through proxy"
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let proxy = ProxyInfo::new("127.0.0.1".to_string(), 8080)
        .with_auth("user".to_string(), "pass".to_string());
    
    let result = client.request(
        Method::GET,
        &format!("{}/proxy-test", mock_server.uri()),
        None,
        None,
        Some(proxy),
    ).await;

    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_timeout() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/slow"))
        .respond_with(ResponseTemplate::new(200)
            .set_delay(Duration::from_secs(5))
            .set_body_json(json!({
                "slow_response": true
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let result = timeout(
        Duration::from_secs(1),
        client.request(
            Method::GET,
            &format!("{}/slow", mock_server.uri()),
            None,
            None,
            None,
        )
    ).await;

    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_error_handling() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/not-found"))
        .respond_with(ResponseTemplate::new(404)
            .set_body_json(json!({
                "error": "Not Found",
                "message": "Resource not found"
            })))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let response = client.request(
        Method::GET,
        &format!("{}/not-found", mock_server.uri()),
        None,
        None,
        None,
    ).await?;

    assert_eq!(response.status, 404);
    assert!(response.text.contains("Not Found"));
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_json_response_parsing() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    let expected_data = json!({
        "products": [
            {
                "id": "prod-1",
                "name": "Laptop",
                "price": 999.99,
                "in_stock": true,
                "stock_count": 5
            },
            {
                "id": "prod-2", 
                "name": "Mouse",
                "price": 29.99,
                "in_stock": false,
                "stock_count": 0
            }
        ],
        "total": 2,
        "page": 1
    });
    
    Mock::given(method("GET"))
        .and(path("/products"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(expected_data))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let response = client.request(
        Method::GET,
        &format!("{}/products", mock_server.uri()),
        None,
        None,
        None,
    ).await?;

    assert_eq!(response.status, 200);
    
    let json_response: serde_json::Value = serde_json::from_str(&response.text)?;
    assert_eq!(json_response["total"], 2);
    assert_eq!(json_response["products"].as_array().unwrap().len(), 2);
    assert_eq!(json_response["products"][0]["name"], "Laptop");
    assert_eq!(json_response["products"][0]["in_stock"], true);
    assert_eq!(json_response["products"][1]["in_stock"], false);
    
    Ok(())
}

#[tokio::test]
async fn test_api_client_large_response() -> Result<()> {
    let mock_server = MockServer::start().await;
    
    let large_data: Vec<serde_json::Value> = (0..1000)
        .map(|i| json!({
            "id": i,
            "name": format!("Item {}", i),
            "description": format!("Description for item {}", i),
            "price": (i as f64) * 10.0,
            "category": format!("Category {}", i % 10)
        }))
        .collect();
    
    let response_data = json!({
        "items": large_data,
        "total": 1000
    });
    
    Mock::given(method("GET"))
        .and(path("/large-data"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(response_data))
        .mount(&mock_server)
        .await;

    let client = ApiClient::new(Some("TestAgent/1.0".to_string()))?;
    
    let response = client.request(
        Method::GET,
        &format!("{}/large-data", mock_server.uri()),
        None,
        None,
        None,
    ).await?;

    assert_eq!(response.status, 200);
    
    let json_response: serde_json::Value = serde_json::from_str(&response.text)?;
    assert_eq!(json_response["total"], 1000);
    assert_eq!(json_response["items"].as_array().unwrap().len(), 1000);
    
    Ok(())
}

#[tokio::test]
async fn test_proxy_info_creation() -> Result<()> {
    let proxy1 = ProxyInfo::new("192.168.1.1".to_string(), 8080);
    assert_eq!(proxy1.host, "192.168.1.1");
    assert_eq!(proxy1.port, 8080);
    assert!(proxy1.username.is_none());
    assert!(proxy1.password.is_none());
    
    let proxy2 = ProxyInfo::new("10.0.0.1".to_string(), 3128)
        .with_auth("user123".to_string(), "pass456".to_string());
    assert_eq!(proxy2.host, "10.0.0.1");
    assert_eq!(proxy2.port, 3128);
    assert_eq!(proxy2.username, Some("user123".to_string()));
    assert_eq!(proxy2.password, Some("pass456".to_string()));
    
    let url1 = proxy1.to_url()?;
    assert_eq!(url1, "http://192.168.1.1:8080");
    
    let url2 = proxy2.to_url()?;
    assert_eq!(url2, "http://user123:pass456@10.0.0.1:3128");
    
    Ok(())
}

#[tokio::test]
async fn test_retry_config() {
    let default_config = RetryConfig::default();
    assert_eq!(default_config.max_retries, 3);
    assert_eq!(default_config.base_delay_ms, 1000);
    assert_eq!(default_config.max_delay_ms, 10000);
    assert_eq!(default_config.backoff_multiplier, 2.0);
    
    let custom_config = RetryConfig {
        max_retries: 5,
        base_delay_ms: 500,
        max_delay_ms: 5000,
        backoff_multiplier: 1.5,
    };
    
    assert_eq!(custom_config.max_retries, 5);
    assert_eq!(custom_config.base_delay_ms, 500);
    assert_eq!(custom_config.max_delay_ms, 5000);
    assert_eq!(custom_config.backoff_multiplier, 1.5);
}
