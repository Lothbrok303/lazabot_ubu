use lazabot::captcha::{CaptchaSolver, CaptchaSolverTrait, MockCaptchaSolver};
use std::env;
use tokio;

#[tokio::test]
async fn test_captcha_solver_creation() {
    let solver = CaptchaSolver::new("test_api_key".to_string());
    // Test that the solver was created successfully
    assert!(!solver.api_key.is_empty());
}

#[tokio::test]
async fn test_mock_image_captcha_solving() {
    let solver = MockCaptchaSolver::new(
        "test_image_result".to_string(),
        "test_recaptcha_result".to_string(),
    );
    
    let result = solver.solve_image(b"fake_image_data").await.unwrap();
    assert_eq!(result, "test_image_result");
}

#[tokio::test]
async fn test_mock_recaptcha_solving() {
    let solver = MockCaptchaSolver::new(
        "test_image_result".to_string(),
        "test_recaptcha_result".to_string(),
    );
    
    let result = solver.solve_recaptcha("test_site_key", "https://example.com").await.unwrap();
    assert_eq!(result, "test_recaptcha_result");
}

#[tokio::test]
async fn test_captcha_solver_from_env() {
    // Set a test environment variable
    env::set_var("CAPTCHA_API_KEY", "test_env_api_key");
    
    let solver = CaptchaSolver::from_env().unwrap();
    assert_eq!(solver.api_key, "test_env_api_key");
    
    // Clean up
    env::remove_var("CAPTCHA_API_KEY");
}

#[tokio::test]
async fn test_captcha_solver_from_env_missing() {
    // Ensure the environment variable is not set
    env::remove_var("CAPTCHA_API_KEY");
    
    let result = CaptchaSolver::from_env();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("CAPTCHA_API_KEY environment variable not set"));
}

#[tokio::test]
async fn test_captcha_type_methods() {
    let solver = CaptchaSolver::new("test_api_key".to_string());
    
    use lazabot::captcha::CaptchaType;
    assert_eq!(solver.get_method(&CaptchaType::Image), "base64");
    assert_eq!(solver.get_method(&CaptchaType::ReCaptchaV2), "userrecaptcha");
    assert_eq!(solver.get_method(&CaptchaType::ReCaptchaV3), "userrecaptcha");
}

#[tokio::test]
async fn test_mock_solver_trait_implementation() {
    let mock_solver = MockCaptchaSolver::new(
        "mock_image".to_string(),
        "mock_recaptcha".to_string(),
    );
    
    // Test that the mock solver implements the trait correctly
    let image_result = mock_solver.solve_image(b"test").await.unwrap();
    assert_eq!(image_result, "mock_image");
    
    let recaptcha_result = mock_solver.solve_recaptcha("site", "url").await.unwrap();
    assert_eq!(recaptcha_result, "mock_recaptcha");
}

// Integration test that would work with a real 2Captcha API key
#[tokio::test]
#[ignore] // This test requires a real API key and should be run manually
async fn test_real_2captcha_integration() {
    if let Ok(api_key) = env::var("CAPTCHA_API_KEY") {
        let solver = CaptchaSolver::new(api_key);
        
        // Test with a simple image (this would fail with real API but tests the flow)
        let result = solver.solve_image(b"fake_image_data").await;
        // We expect this to fail with a real API call, but it tests the integration
        assert!(result.is_err());
    } else {
        println!("Skipping real 2Captcha integration test - no API key provided");
    }
}
