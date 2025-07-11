use crate::ports::{udp_receiver, tcp_sender};

/// Starts the main application logic.
/// 
/// - Creates a channel for communication between UDP receiver and TCP sender.
/// - Spawns the UDP listener and TCP sender tasks.
/// - Waits for a Ctrl+C signal to gracefully shut down the proxy.
pub async fn start_application() {
    let (tx, rx) = tokio::sync::mpsc::channel(10_000);

    tokio::spawn(udp_receiver::start_udp_listener(tx));
    tokio::spawn(tcp_sender::start_tcp_sender(rx));

    // Infinite wait to keep the process alive until Ctrl+C is received
    tokio::signal::ctrl_c().await.expect("Ctrl-C failed");
    println!("Shutting down UDP → TCP proxy...");
}