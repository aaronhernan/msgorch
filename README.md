
# MsgOrch (Messages Orchestrator)

## Descripcion
Actualizaremos descipcion cuando vaya tomando forma la cosa...
Y si ves este comentario y no te parece, estas invitado a contribuir y mejorar la app !
Y si no, tambien.

# Informacion, tracing, debug, logs

## Tipos de eventos
Vamos a definir diferentes niveles de registro para poder filtrarlos en un 
agregador de logs como Loki en un futuro, o poder cambiar el estado de la 
aplicacion para poder para que lance mas o menor informacion sobre eventos.
Los eventos debemos definirlos de la siguiente manera:


| Nivel  | Cuándo usarlo                               |
| ------ | ------------------------------------------- |
| trace! | Detalles internos, casi debug de bajo nivel |
| debug! | Flujo normal pero muy frecuente             |
| info!  | Hechos relevantes del sistema               |
| warn!  | Algo raro pero esperado                     |
| error! | Algo falló y no debería                     |

En resumen:
- Funcionamiento → debug / info
- Problema esperado → warn
- Problema real → error

Para iniciar el programa mediante un nivel de registro, podemos exportar 
la variable de entorno, de manera temporal antes de executar la explicacion, 
de manera permanente en el entorno del sistema, o en el archivo de env.
```bash
# De manera temporal
RUST_LOG=error,info cargo run
# De variable del sistema
export RUST_LOG=error,info
cargo run
```

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
cargo add sqlx --features runtime-tokio-rustls,postgres,macros,migrate,chrono
cargo install sqlx-cli --no-default-features --features postgres
# Por si utilizamos UUIDs
# cargo add uuid --features v4,serde
cargo add chrono --features serde
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

## Postgres
Requerimos postgres, para guardar persistencia entre mensajes e historial de mensajes.
Si no se tiene un servidor, se puede agregar uno mediante podman o crear las credenciales de uso.

### Crear db y usuarios
```bash
#Si se tiene psql
psql -U postgres -h localhost

# Si no se tiene, se utiliza el del contenedor
podman exec -it postgres-general /bin/bash
psql -U postgres

# Despues
# Si se quiere utilizar password:
CREATE USER msgorchuser WITH ENCRYPTED PASSWORD 'local.Pass9';
CREATE DATABASE msgorch_db OWNER msgorchuser;
# Opcional, pero en veces necsario, si se utiliza un esquema de permisos complejo
GRANT ALL PRIVILEGES ON DATABASE msgorch_db TO msgorchuser;
\q
```
Nota: si estas dentro de un contenedor, utilizar [Ctrl-p, Ctrl+q] para des-attach del contenedor 
      y no detenerlo

### Crear migraciones y correrlas
Se utiliza la carpeta del proyecto "./migrations/" para guardar las migraciones en codigo sql, las cuales son 
generadas por el comando sqlx. Para la instalacion y creacion de migraciones :

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate add -r create_incoming_messages
# Despues editar los archivos para executar y revertir la migracion, despues:
sqlx migrate run
# Revertir:
sqlx migrate revert
```
