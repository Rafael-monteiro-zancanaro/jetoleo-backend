use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{self, Utc};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub birth_date: chrono::DateTime<Utc>,
}
