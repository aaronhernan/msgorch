use chrono::prelude::*;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
