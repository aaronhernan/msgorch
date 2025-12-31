use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Message {
    /// ID interno (PK)
    pub id: Option<i64>,

    /// Indica si el mensaje es entrante (false) o saliente (true)
    pub from_me: bool,

    /// ID original de Evolution / WhatsApp
    pub transporter_message_id: String,

    /// Instancia de Evolution (Transporter)
    pub instance: String,

    /// JID remoto principal
    pub remote_jid: String,

    /// JID alternativo (fallback / debug)
    pub remote_jid_alt: Option<String>,

    /// Texto del mensaje
    pub text: String,

    /// Timestamp original del mensaje (si existe)
    pub origin_timestamp: Option<DateTime<Utc>>,

    /// Cuándo lo recibimos / persistimos
    pub created_at: DateTime<Utc>,
}
