use bytes::BytesMut;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use std::env;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio_util::codec::Decoder;
use tokio_util::udp::UdpFramed;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

static UDP_PORT: Lazy<String> = Lazy::new(|| env::var("UDP_PORT").unwrap_or("5013".to_string()));
static TCP_SERVER: Lazy<String> = Lazy::new(|| {
    let tcp_host = env::var("TCP_HOST").unwrap_or("localhost".to_string());
    let tcp_port = env::var("TCP_PORT").unwrap_or("9001".to_string());

    format!("{}:{}", tcp_host, tcp_port)
});

#[derive(Debug, Clone)]
pub struct Protocol {
    report_tail: u8,
}

impl Decoder for Protocol {
    type Item = String;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut tail_pos = None;
        let mut idx = 0;
        for i in 0..src.len() {
            let b = src[i];
            if b <= 127 {
                if tail_pos.is_none() && b == self.report_tail {
                    tail_pos = Some(idx);
                }
                src[idx] = b;
                idx += 1;
            }
        }
        src.truncate(idx);

        if src.len() == 0 {
            return Ok(None);
        }

        if let Some(pos) = tail_pos {
            let line = src.split_to(pos + 1);
            let message = String::from_utf8_lossy(&line).to_string();

            return Ok(Some(message));
        }

        Ok(None)
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if !buf.is_empty() {
                    debug!("Bytes remaining on stream: {:?}", buf);
                }
                Ok(None)
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // TODO: Handle Queclink protocol
    let protocol = Protocol { report_tail: b'\r' };
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", *UDP_PORT)).await?;
    let mut framed = UdpFramed::new(socket, protocol);

    info!("Starting SiscomProxy...");

    loop {
        tokio::select! {
            Some(result) = framed.next() => {
                // TODO: Notify Sentry for errors
                match result {
                    Ok((message, addr)) => {
                        debug!("Received message from {}: {:?}", addr, message);
                        tokio::spawn(async move {
                            if let Err(e) = forward_message(message.as_bytes()).await {
                                error!("Error forwarding message to TCP server: {:?}", e);
                                return;
                            }
                            if let Err(e) = acknowledge(message, addr.to_string()).await {
                                error!("Error sending ACK to UDP client: {:?}", e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Error receiving UDP message: {:?}", e);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Received CTRL+C, terminating...");
                break;
            }
        }
    }

    Ok(())
}

async fn forward_message(message: &[u8]) -> Result<(), Error> {
    match TcpStream::connect(&*TCP_SERVER).await {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(message).await {
                return Err(Error::Io(e));
            }
        }
        Err(e) => {
            return Err(Error::Io(e));
        }
    }

    Ok(())
}

async fn acknowledge(message: String, addr: String) -> Result<(), Error> {
    // TODO: Handle ack for Queclink protocol
    let ack = message.replace("STT", "ACK").replace("ALT", "ACK");
    let ack_socket = UdpSocket::bind("0.0.0.0:0").await?;

    ack_socket.send_to(ack.as_bytes(), addr).await?;

    Ok(())
}

// TODO: Add unit tests
