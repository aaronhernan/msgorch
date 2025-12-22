use serde::Deserialize;

// Estructura generica, por si fallan las versiones
// #[derive(Debug, Deserialize)]
// pub struct MessageUpdateData {
//     pub key: MessageKey,
//     pub update: serde_json::Value,
// }

#[derive(Debug, Deserialize)]
pub struct MessageUpdateData {
    pub key: MessageKey,
    pub update: MessageUpdate,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct MessageKey {
    pub id: Option<String>,

    #[serde(rename = "remoteJid")]
    pub remote_jid: Option<String>,

    #[serde(rename = "fromMe")]
    pub from_me: bool,
}

#[derive(Debug, Deserialize)]
pub struct MessageUpdate {
    pub status: Option<i32>, // sent, delivered, read, etc.
    pub edited: Option<bool>,
    pub reactions: Option<serde_json::Value>,
}