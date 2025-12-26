use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StoredMessage {
    pub id: i64,                    // PK interna
    pub evolution_id: String,       // id del mensaje en Evolution
    pub instance: String,           // instancia WhatsApp
    pub remote_jid: String,         // jid remoto
    pub remote_jid_alt: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<i64>,     // timestamp original del mensaje
    pub created_at: DateTime<Utc>,  // cuando lo persistimos
}
