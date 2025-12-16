mod app;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    app::run().await;
}
