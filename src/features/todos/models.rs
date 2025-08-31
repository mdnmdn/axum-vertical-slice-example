use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct CreateTodoPayload {
    #[validate(length(min = 1))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct UpdateTodoPayload {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub completed: Option<bool>,
}
