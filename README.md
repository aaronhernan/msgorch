
# MsgOrch (Messages Orchestrator)

## Descripcion



# Creacion de proyecto y dependencias

```bash
cargo new msgorch
cd msgorg
# Instalacion de dependencias
cargo add axum
cargo add tokio --features full
cargo add serde_json
cargo add serde --features derive
cargo add tracing
cargo add tracing-subscriber --features fmt
cargo add dotenvy
cargo add reqwest --features json
# Dependencias solo para desarrollo y pruebas
cargo add wiremock --dev
cargo add reqwest --dev
cargo add serde_json --dev
# Continuamos con mas dependencias normales
cargo add deadpool-redis redis
# Utilizamos al anyhow para errores generos, lanzando la respuesta de error obtenida desde la libreria
cargo add anyhow
# Agregar thiserror para generar errores propios, y no utilizar anyhow dentro de la logica de dominio
cargo add thiserror
# Para calcular jitter
cargo add rand
```

## Verison a produccion
Crear el compilado final mediante:

```bash
cargo build --release
```

## Redis
Utilizar un servidor de redis existente, si no se tiene uno, vamos a crear un contenedos:
Crear las IP y puertos acorde a produccion, a la aplicacion, las variables de entorno y servidor a utilizar

```bash
# Si utilizamos persistencia, (ojo, falta activar el tipo de persistencia, solo defino el volume)
podman run -d --name redis-msgorch -v=reidismsgorch:/data -p 9001:6379 docker.io/library/redis:8.4.0
# Sin persistencia
podman run -d --name redis-msgorch -p 9001:6379 docker.io/library/redis:8.4.0
```

Archivo quadlets para produccion:

```bash
[Unit]
Description=Redis for YOURAPLICATIONAME
Requires=
After=

[Container]
Image=docker.io/library/redis:8.4.0
ContainerName=evolution-redis
Exec=redis-server --appendonly yes
Volume=evolution_redis:/data

[Service]
Restart=always
```
