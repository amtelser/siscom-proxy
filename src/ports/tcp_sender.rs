use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::domain::message::DeviceMessage;
use tokio::sync::mpsc::Receiver;
use std::time::Duration;

use crate::config::CONFIG;

use tracing::{error, debug};

/// Starts a TCP sender loop that connects to a configured TCP server and sends messages received from a channel.
/// If the connection fails, it retries every 5 seconds.
/// If sending a message fails, it breaks the inner loop and tries to reconnect.
pub async fn start_tcp_sender(mut rx: Receiver<DeviceMessage>) {
    let addr = format!("{}:{}", CONFIG.tcp_host, CONFIG.tcp_port);
    loop {
        match TcpStream::connect(addr.clone()).await {
            Ok(mut stream) => {
                debug!("Connected to TCP server at {}", addr);
                while let Some(msg) = rx.recv().await {
                    // Send the raw message bytes over the TCP stream
                    if let Err(e) = stream.write_all(msg.raw.as_bytes()).await {
                        eprintln!("Error writing to TCP: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Error connecting to TCP server {}: {:?}", addr, e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}