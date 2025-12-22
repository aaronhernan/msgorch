use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PresenceUpdateData {
    #[serde(rename = "id")]
    pub id: Option<String>,

    pub presences: Option<HashMap<String, PresenceEntry>>,

    #[serde(rename = "remoteJid")]
    pub remote_jid: Option<String>,

    pub presence: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PresenceEntry {
    pub presence: Option<String>,

    #[serde(rename = "lastSeen")]
    pub last_seen: Option<i64>,
}