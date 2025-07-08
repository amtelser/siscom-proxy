mod config;
mod domain;
mod ports;
mod app;

use tracing_subscriber::EnvFilter;

use tracing::{info};

#[tokio::main]
async fn main() {
    // No necesitas: use config::CONFIG;
    info!("Starting SiscomProxy...");
    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("🚀 Iniciando proxy UDP → TCP");
    app::service::start_application().await;
}