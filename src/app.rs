use axum::{
    routing::post,
    Router,
    middleware,
};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    handlers::webhook::webhook_handler,
    services::evolution::EvolutionService,
    config::Config,
};

pub async fn run(config: Config) {
    let evolution = EvolutionService::new(&config);
    let addr = format!("{}:{}", config.listen_host, config.listen_port);
    let state = (config.clone(), evolution);
    let app = Router::new()
        .route(
            "/webhook",
            post(webhook_handler)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    crate::middleware::webhook_auth,
                )),
        )
        .with_state(state);
    // let app = Router::new()
    //     .route("/webhook", 
    //         post(webhook_handler).route_layer(
    //             middleware::from_fn_with_state(
    //                 config.clone(),
    //                 crate::middleware::webhook_auth,
    //             )
    //         ),
    //     )
    //     .with_state((config.clone(), evolution)); // Estado global, servicio

    let listener = TcpListener::bind(&addr)
        .await
        .expect("No se pudo abrir el puerto");

    info!("Escuchando en http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
