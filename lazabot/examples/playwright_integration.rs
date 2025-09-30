use lazabot::integrations::playwright::PlaywrightClient;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting Playwright integration example");
    let mut client = PlaywrightClient::new();
    match client.ensure_server_running().await {
        Ok(_) => info!("Playwright server is ready"),
        Err(e) => {
            println!("Failed to start Playwright server: {}", e);
            return Err(e.into());
        }
    }
    match client.is_server_healthy().await {
        Ok(health) => info!("Server health: {:?}", health),
        Err(e) => println!("Health check failed: {}", e),
    }
    info!("Example completed successfully!");
    Ok(())
}