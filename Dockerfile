FROM rust:1 AS build

RUN apt-get update && apt-get install -y librdkafka-dev cmake pkg-config libssl-dev zlib1g-dev clang lld

WORKDIR /app

COPY Cargo.lock Cargo.toml .
RUN cargo fetch
COPY .cargo .cargo
COPY src src
RUN cargo install --path . --root /release --locked


FROM ubuntu:latest

RUN apt-get update && apt-get install -y sudo gnupg ca-certificates wget
RUN wget -qO- https://repos.influxdata.com/influxdata-archive.key | \
  gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/influxdata-archive.gpg > /dev/null
RUN echo 'deb [signed-by=/etc/apt/trusted.gpg.d/influxdata-archive.gpg] https://repos.influxdata.com/ubuntu stable main' | \
  sudo tee /etc/apt/sources.list.d/influxdata.list
RUN apt-get update && apt-get install -y telegraf

COPY docker-files/telegraf.conf /etc/telegraf/telegraf.conf
RUN telegraf --test --config /etc/telegraf/telegraf.conf

COPY --from=build /release/bin/* /usr/local/bin/

ENV APP_USER=siscom-proxy

RUN useradd --system --no-create-home $APP_USER \
  && usermod -aG sudo $APP_USER \
  && echo "$APP_USER ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers \
  && chown -R $APP_USER:$APP_USER /usr/local/bin \
  && chown -R $APP_USER:$APP_USER /etc/telegraf

USER $APP_USER

COPY --chmod=755 docker-files/entrypoint.sh /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]

EXPOSE 5013/udp
