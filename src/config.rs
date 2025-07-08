use once_cell::sync::Lazy;
use std::env;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();
    Config::from_env()
});

pub struct Config {
    pub udp_port: u16,
    pub tcp_host: String,
    pub tcp_port: u16,
}

impl Config {
    fn from_env() -> Self {
        Self {
            udp_port: env::var("UDP_PORT")
                .unwrap_or_else(|_| "9000".to_string())
                .parse()
                .expect("UDP_PORT debe ser un número"),
            tcp_host: env::var("TCP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            tcp_port: env::var("TCP_PORT")
                .unwrap_or_else(|_| "4000".to_string())
                .parse()
                .expect("TCP_PORT debe ser un número"),
        }
    }
}