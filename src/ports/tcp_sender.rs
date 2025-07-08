use tokio::{net::TcpStream, io::AsyncWriteExt};
use crate::domain::message::DeviceMessage;
use tokio::sync::mpsc::Receiver;
use std::time::Duration;

use crate::config::CONFIG;

use tracing::{error, debug};

pub async fn start_tcp_sender(mut rx: Receiver<DeviceMessage>) {
    let addr = format!("{}:{}", CONFIG.tcp_host, CONFIG.tcp_port);
    loop {
        match TcpStream::connect(addr.clone()).await {
            Ok(mut stream) => {
                debug!("Conectado al servidor TCP en {}", addr);
                while let Some(msg) = rx.recv().await {
                    if let Err(e) = stream.write_all(msg.raw.as_bytes()).await {
                        eprintln!("Error al escribir en TCP: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Error al conectar al servidor TCP {}: {:?}", addr, e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}