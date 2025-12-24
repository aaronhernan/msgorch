#[derive(Debug, Clone)]
pub struct IncomingMessage {
    pub id: String,
    pub instance: String,
    pub remote_jid: String,
    pub remote_jid_alt: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<i64>,
    pub from_me: bool,
}