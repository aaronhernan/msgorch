use std::time::Duration;

use deadpool_redis::{
    redis::{AsyncCommands},
    Pool,
};
use thiserror::Error;

/// Errores de idempotencia (nivel dominio)
#[derive(Debug, Error)]
pub enum IdempotencyError {
    #[error("Redis pool error: {0}")]
    Pool(#[from] deadpool_redis::PoolError),

    #[error("Redis command error: {0}")]
    Redis(#[from] deadpool_redis::redis::RedisError),
}

/// Store de idempotencia basado en Redis
#[derive(Clone)]
pub struct RedisIdempotencyStore {
    pool: Pool,
    prefix: String,
    ttl: Duration,
}

impl RedisIdempotencyStore {
    pub fn new(pool: Pool, prefix: impl Into<String>, ttl: Duration) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
            ttl,
        }
    }

    /// Devuelve:
    /// - Ok(true)  → evento NUEVO (se marcó en Redis)
    /// - Ok(false) → evento DUPLICADO
    pub async fn check_and_mark(&self, key: &str) -> Result<bool, IdempotencyError> {
        let redis_key = format!("{}:{}", self.prefix, key);

        let mut conn = self.pool.get().await?;

        // SETNX → solo escribe si no existe
        let was_set: bool = conn.set_nx(&redis_key, 1).await?;

        if was_set {
            // Redis EXPIRE requiere i64
            let ttl_secs = self.ttl.as_secs() as i64;
            let _: bool = conn.expire(&redis_key, ttl_secs).await?;
        }

        Ok(was_set)
    }
}