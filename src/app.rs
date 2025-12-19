use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::Config,
    handlers,
    middleware,
    services::evolution::EvolutionService,
};

pub type AppState = (Config, EvolutionService);

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/webhook",
            axum::routing::post(handlers::webhook::webhook_handler),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::webhook_auth,
        ))
        .with_state(state)
}

pub async fn run(config: Config) -> Result<(), std::io::Error> {
    let addr = format!("{}:{}", config.listen_host, config.listen_port);
    let listener = TcpListener::bind(addr).await?;
    info!("Escuchando en http://{}", listener.local_addr()?);
    run_with_listener(listener, config).await
}

pub async fn run_with_listener(
    listener: TcpListener,
    config: Config,
) -> Result<(), std::io::Error> {
    let evolution = EvolutionService::new(&config);
    let state = (config, evolution);

    let app = build_router(state);
    axum::serve(listener, app).await
}
