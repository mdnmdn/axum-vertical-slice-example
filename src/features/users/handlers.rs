use super::{
    create_user::create_user as create_user_logic, get_user::get_user as get_user_logic,
    models::CreateUserPayload,
};
use crate::{app_state::AppState, error::AppError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use tracing::instrument;

#[instrument(skip(state), name = "Create User Handler")]
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users"
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    let user = create_user_logic(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[instrument(skip(state), name = "Get User Handler")]
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users"
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_user_logic(&state.db_pool, user_id).await?;

    Ok((StatusCode::OK, Json(user)))
}
