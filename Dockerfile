# Imagen base con toolchain Rust + glibc adecuado (Debian compatible con Kubernetes)
FROM rust:1.78-slim-bullseye

# Instala dependencias necesarias para compilar
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

WORKDIR /app

# Copiamos manifiestos primero para aprovechar la caché de Docker
COPY Cargo.toml Cargo.lock ./

# Descarga dependencias
RUN cargo fetch

# Ahora copiamos el código fuente
COPY src ./src

# Compila en modo release
RUN cargo build --release

# Definimos el comando y el puerto expuesto
EXPOSE 5013/udp

ENV RUST_LOG=info
CMD ["./target/release/udp_tcp_proxy"]