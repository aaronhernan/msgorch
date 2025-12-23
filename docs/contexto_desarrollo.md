# Contexto de Desarrollo – Webhook & Events (Rust)

Fecha de exportación: 2025-12-23T07:05:28.688893Z

## Objetivo del proyecto
Implementar un backend en Rust para manejar webhooks de Evolution API (WhatsApp),
con arquitectura basada en eventos, idempotencia, retries con backoff real,
y trazas (tracing) normalizadas para observabilidad (Loki).

## Puntos clave acordados

### Arquitectura
- **Webhook handler**:
  - Recibe `WebhookEnvelope`
  - Crea un `info_span` común (app, instance, event, jid/remote_jid si aplica)
  - Delegación inmediata al `dispatcher`
- **Dispatcher**:
  - Hace match por `payload.event`
  - Llama a `events::<evento>::handle`
  - No contiene lógica de negocio
- **Eventos**:
  - Carpeta `src/events/`
  - Cada evento en su archivo (`message_upsert`, `messages_update`, `connection_update`, etc.)
  - No refactorizar a handlers consolidados por ahora

### Modelos
- `models/webhook.rs` → `WebhookEnvelope`
- `models/evolution/` → estructuras tal cual llegan del webhook
- `models/domain/` → modelos propios de la app

#### Modelo de dominio principal
```rust
#[derive(Debug, Clone)]
pub struct IncomingMessage {
    pub id: String,
    pub remote_jid: String,
    pub remote_jid_alt: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<i64>,
    pub from_me: bool,
}
```

### message_upsert
- Deserializa `serde_json::Value` → `MessageUpsertData`
- Mapea **una sola vez** a `IncomingMessage`
- Aplica **idempotencia solo aquí**
- Filtra mensajes `from_me`
- Llama a `process_message`

### message_processor
Contiene:
- `ProcessError`
- `is_retryable`
- `process_message`:
  - control de retries
  - backoff exponencial + jitter real
- `handle_message`:
  - lógica de negocio (por ahora enviar ACK por Evolution API)

### Idempotencia
- Solo en `message_upsert`
- No duplicar chequeos en `message_processor`

### Retries
- Backoff exponencial real
- Delay aleatorio entre `0..max_delay`
- Reintentos configurables
- Solo errores retryables

### Tracing
- `info_span` creado en webhook/dispatcher
- Campos comunes:
  - app
  - instance
  - event
  - jid (propio)
  - remote_jid
  - message_id (si existe)
- El span afecta `info!`, `warn!`, `error!`, `debug!`
- Se pueden loggear campos no definidos en el span

### Convenciones
- Rust `snake_case` en structs
- `serde(rename = "...")` para camelCase entrante
- No mover firmas de funciones sin acuerdo previo 😄
- No refactorizaciones grandes prematuras

## Estado actual
- Compila sin warnings
- Webhooks funcionando
- Respuestas enviadas correctamente a WhatsApp
- Eventos implementados:
  - messages.upsert
  - messages.update
  - message.update
  - connection.update
  - presence.update (best-effort)
  - chats.update

## Pendiente / Futuro
- Validaciones más formales
- Persistencia de historial/memoria
- Métricas
- Nuevos eventos
- Posible EventContext (más adelante)

---
Este documento sirve como contexto base para retomar el desarrollo en el futuro.
