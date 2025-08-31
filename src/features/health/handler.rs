use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check endpoint")
    ),
    tag = "Health"
)]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}
