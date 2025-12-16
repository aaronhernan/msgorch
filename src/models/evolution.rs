use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub event: String,
    pub data: MessageData,
}

#[derive(Debug, Deserialize)]
pub struct MessageData {
    pub key: MessageKey,
    pub message: Option<MessageContent>,
}

#[derive(Debug, Deserialize)]
pub struct MessageKey {
    #[serde(rename = "remoteJid")]
    pub remote_jid: String,
    #[serde(rename = "fromMe")]
    pub from_me: bool,
}

#[derive(Debug, Deserialize)]
pub struct MessageContent {
    pub conversation: Option<String>,
}