mod config;
mod domain;
mod ports;
mod app;

use tracing_subscriber::EnvFilter;
use tracing::{info};

/// Entry point for the SiscomProxy application.
/// 
/// - Initializes logging and tracing.
/// - Starts the UDP to TCP proxy service.
#[tokio::main]
async fn main() {
    info!("Starting SiscomProxy...");
    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("🚀 Starting UDP → TCP proxy");
    app::service::start_application().await;
}