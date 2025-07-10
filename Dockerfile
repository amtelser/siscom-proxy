FROM rust:1.78-slim-bullseye

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo fetch

RUN cargo build --release

EXPOSE 5013/udp

# No copies .env aquí
CMD ["./target/release/udp_tcp_proxy"]