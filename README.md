# Siscom Proxy UDP → TCP

Proxy de alta performance escrito en **Rust**, diseñado para recibir mensajes por **UDP**, responder un **ACK inmediato** y reenviar el mensaje por **TCP** a un servidor remoto. Está pensado para aplicaciones críticas donde se reciben miles de mensajes por segundo.

---

## 🚀 Características

- Recibe mensajes UDP en un puerto configurable.
- Responde automáticamente un ACK con el formato: `ST300ACK;<device_id>`.
- Reenvía los mensajes por TCP a un servidor configurado.
- Maneja reconexiones TCP automáticas en caso de caída.
- Arquitectura **hexagonal (puertos y adaptadores)**.
- Logs estructurados con `tracing` y nivel configurable vía `RUST_LOG`.
- Compatible con despliegues en **Kubernetes** usando variables de entorno.

---

## 🧱 Variables de entorno

Estas variables se pueden definir vía `.env` en desarrollo, o como **Variables del repositorio en GitHub** o `env:` en Kubernetes.

| Variable    | Descripción                          | Ejemplo                  |
|-------------|--------------------------------------|--------------------------|
| `UDP_PORT`  | Puerto de escucha UDP                | `5013`                   |
| `TCP_HOST`  | Host del servidor destino TCP        | `iot-tcp-prod.enk.mx`    |
| `TCP_PORT`  | Puerto del servidor TCP              | `4000`                   |
| `RUST_LOG`  | Nivel de logs (`info`, `debug`, ...) | `info`                   |

---

## 🐳 Docker

### Construir imagen local:

```bash
docker build -t siscom-proxy .
```

### Ejecutar localmente:

```bash
docker run --rm -it \
  -e UDP_PORT=5013 \
  -e TCP_HOST=iot-tcp-prod.enk.mx \
  -e TCP_PORT=4000 \
  -e RUST_LOG=info \
  -p 5013:5013/udp \
  siscom-proxy
```

---

## ☸️ Despliegue en Kubernetes

El proyecto incluye un manifiesto `deployment.yaml` que puedes generar dinámicamente desde GitHub Actions, o mantener manualmente bajo `k8s/`.

Ejemplo de `env:` en el `Deployment`:

```yaml
env:
  - name: UDP_PORT
    value: "5013"
  - name: TCP_HOST
    value: "iot-tcp-prod.enk.mx"
  - name: TCP_PORT
    value: "4000"
  - name: RUST_LOG
    value: "info"
```

---

## 🔄 Integración con GitHub Actions

Este proyecto está preparado para:
- Construir la imagen en GitHub Actions
- Usar **Variables de GitHub** (`Settings → Variables`)
- Desplegar automáticamente a un clúster Kubernetes

---

## 🧪 Pruebas locales con netcat

Envía un mensaje UDP al proxy:

```bash
echo -n "ST300STT;511926097;..." | nc -u 127.0.0.1 5013
```

Deberías recibir:

```
ST300ACK;511926097
```

---

## 📂 Estructura del proyecto

```bash
src/
├── app/           # Orquestador principal
├── domain/        # Lógica de dominio y modelo
├── ports/         # Interfaces UDP y TCP
├── config.rs      # Lectura de variables de entorno
└── main.rs        # Punto de entrada
```

---

## ⚙️ `docker-compose` (opcional)

Puedes ejecutar el proxy en un entorno local con `docker-compose`:

```yaml
version: '3.8'
services:
  proxy:
    build: .
    image: siscom-proxy
    ports:
      - "5013:5013/udp"
    environment:
      - UDP_PORT=5013
      - TCP_HOST=iot-tcp-prod.enk.mx
      - TCP_PORT=4000
      - RUST_LOG=info
```

Comando:

```bash
docker-compose up --build
```

---

## 📜 Licencia

MIT © 2025 Encontrack
