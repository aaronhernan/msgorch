
# MsgOrch (Messages Orchestrator)

## Descripcion



## Creacion de proyecto y dependencias

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
```

## Verison a produccion
Crear el compilado final mediante:

```bash
cargo build --release
```