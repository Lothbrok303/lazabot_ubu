use anyhow::Result;
use lazabot::captcha::{CaptchaSolver, CaptchaSolverTrait};
use std::env;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Example 1: Create solver with API key from environment
    let solver = match CaptchaSolver::from_env() {
        Ok(solver) => {
            info!("Created captcha solver from environment variable");
            solver
        }
        Err(_) => {
            info!("CAPTCHA_API_KEY not set, using mock solver for demonstration");
            // For demonstration purposes, we'll use a mock solver
            // In real usage, you would set CAPTCHA_API_KEY environment variable
            return run_mock_example().await;
        }
    };

    // Example 2: Solve an image captcha
    let image_data = b"fake_image_data_for_demonstration";
    match solver.solve_image(image_data).await {
        Ok(result) => info!("Image captcha solved: {}", result),
        Err(e) => info!("Failed to solve image captcha: {}", e),
    }

    // Example 3: Solve a reCAPTCHA
    let site_key = "6Le-wvkSAAAAAPBMRTvw0Q4Muexq9bi0DJwx_mJ-";
    let page_url = "https://www.google.com/recaptcha/api2/demo";

    match solver.solve_recaptcha(site_key, page_url).await {
        Ok(result) => info!("reCAPTCHA solved: {}", result),
        Err(e) => info!("Failed to solve reCAPTCHA: {}", e),
    }

    Ok(())
}

async fn run_mock_example() -> Result<()> {
    use lazabot::captcha::MockCaptchaSolver;

    info!("Running mock captcha solver example");

    let mock_solver = MockCaptchaSolver::new(
        "mock_image_result".to_string(),
        "mock_recaptcha_result".to_string(),
    );

    // Test image captcha solving
    let image_data = b"fake_image_data";
    let result = mock_solver.solve_image(image_data).await?;
    info!("Mock image captcha result: {}", result);

    // Test reCAPTCHA solving
    let site_key = "test_site_key";
    let page_url = "https://example.com";
    let result = mock_solver.solve_recaptcha(site_key, page_url).await?;
    info!("Mock reCAPTCHA result: {}", result);

    Ok(())
}
