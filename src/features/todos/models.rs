use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
#[schema(example = json!({"id": "a67e4810-2624-42f0-9485-219593c836a0", "title": "Buy milk", "completed": false, "created_at": "2024-01-01T12:00:00Z", "updated_at": "2024-01-01T12:00:00Z"}))]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
#[schema(example = json!({"title": "Buy milk"}))]
pub struct CreateTodoPayload {
    #[validate(length(min = 1))]
    pub title: String,
}

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
#[schema(example = json!({"title": "Buy milk and bread", "completed": true}))]
pub struct UpdateTodoPayload {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub completed: Option<bool>,
}
