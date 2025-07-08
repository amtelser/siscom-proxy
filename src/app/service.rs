use crate::ports::{udp_receiver, tcp_sender};

pub async fn start_application() {
    let (tx, rx) = tokio::sync::mpsc::channel(10_000);

    tokio::spawn(udp_receiver::start_udp_listener(tx));
    tokio::spawn(tcp_sender::start_tcp_sender(rx));

    // Espera infinita para mantener el proceso vivo
    tokio::signal::ctrl_c().await.expect("falló el ctrl-c");
    println!("Cerrando el proxy UDP → TCP...");
}