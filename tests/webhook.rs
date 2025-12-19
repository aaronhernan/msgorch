mod helpers;

use helpers::spawn_app;

#[tokio::test]
async fn webhook_rejects_without_token() {
    unsafe {
        std::env::set_var("WEBHOOK_TOKEN", "secret");
        std::env::set_var("EVOLUTION_BASE_URL", "http://localhost");
    }

    let app = spawn_app().await;

    let response = reqwest::Client::new()
        .post(format!("{}/webhook", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn webhook_accepts_with_valid_token() {
    unsafe {
        std::env::set_var("WEBHOOK_TOKEN", "secret");
        std::env::set_var("EVOLUTION_BASE_URL", "http://localhost");
    }

    let app = spawn_app().await;

    let payload = serde_json::json!({
        "event": "MESSAGES_UPSERT",
        "data": {
            "key": {
                "remoteJid": "521234567890@s.whatsapp.net",
                "fromMe": false
            },
            "message": {
                "conversation": "Hola"
            }
        }
    });

    let response = reqwest::Client::new()
        .post(format!("{}/webhook", app.address))
        .header("x-webhook-token", "secret")
        .json(&payload) 
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}