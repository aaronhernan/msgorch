use axum::http::StatusCode;
use serde_json::Value;
use tracing::{debug, warn};

use crate::models::evolution::presence::PresenceUpdateData;
use crate::app::AppState;

pub async fn handle(
    _state: &AppState,
    data: Value,
) -> StatusCode {
    let parsed: PresenceUpdateData = match serde_json::from_value(data) {
        Ok(p) => p,
        Err(err) => {
            warn!("presence.update inválido: {}", err);
            return StatusCode::OK;
        }
    };

    // Caso 1: presences map
    if let Some(presences) = parsed.presences {
        for (jid, entry) in presences {
            debug!(
                jid = %jid,
                presence = ?entry.presence,
                last_seen = ?entry.last_seen,
                "Presence update"
            );
        }
        return StatusCode::OK;
    }

    // Caso 2: formato simple
    if let Some(jid) = parsed.remote_jid.or(parsed.id) {
        debug!(
            jid = %jid,
            presence = ?parsed.presence,
            "Presence update"
        );
        return StatusCode::OK;
    }

    debug!("presence.update recibido sin datos útiles");
    StatusCode::OK
}