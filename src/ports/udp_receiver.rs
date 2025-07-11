use tokio::net::UdpSocket;
use crate::domain::message::DeviceMessage;
use tokio::sync::mpsc::Sender;

use crate::config::CONFIG;
use tracing::{debug};

/// Starts a UDP listener that binds to the configured port.
/// For each received UDP packet, it tries to parse it as a DeviceMessage.
/// If parsing succeeds, it sends an ACK back to the sender and forwards the message through a channel.
pub async fn start_udp_listener(tx: Sender<DeviceMessage>) -> tokio::io::Result<()> {
    let addr = format!("0.0.0.0:{}", CONFIG.udp_port);
    let socket = UdpSocket::bind(addr).await?;
    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        if let Ok(text) = std::str::from_utf8(&buf[..len]) {
            debug!("Message received from {}: {}", addr, text);
            if let Some(msg) = DeviceMessage::parse(text) {
                debug!("Parsed message: {:?}", msg);
                // Send ACK back to the sender
                let _ = socket.send_to(msg.ack().as_bytes(), addr).await;
                // Forward the parsed message to the channel
                let _ = tx.send(msg).await;
            }
        }
    }
}