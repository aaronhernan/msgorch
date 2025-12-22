use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectionUpdateData {
    pub instance: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub reason: Option<String>,
}