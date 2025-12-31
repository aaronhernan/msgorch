use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};

use msgorch::{app, config::Config};

pub struct TestApp {
    pub address: String,
    pub address_public: String,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("No se pudo bindear puerto");

    let listener_public = TcpListener::bind(format!("127.0.0.1:0"))
        .await
        .expect("No se pudo bindear puerto para servidor publico");
    let port = listener.local_addr().unwrap().port();
    let port_public = listener_public.local_addr().unwrap().port();

    unsafe {
        std::env::set_var("LISTEN_HOST", "127.0.0.1");
        std::env::set_var("LISTEN_PORT", port.to_string());
        std::env::set_var("LISTEN_PUBLIC_PORT", port_public.to_string());
        std::env::set_var("WEBHOOK_TOKEN", "secret");

        std::env::set_var("EVOLUTION_BASE_URL", "http://localhost");
        std::env::set_var("EVOLUTION_API_KEY", "test-api-key");
    }

    let config = Config::from_env();

    tokio::spawn(async move {
        app::run_with_listener(listener, listener_public, config)
            .await
            .expect("server error");
    });

    // evitar race condition
    sleep(Duration::from_millis(100)).await;

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        address_public: format!("http://127.0.0.1:{}", port_public),
    }
}
