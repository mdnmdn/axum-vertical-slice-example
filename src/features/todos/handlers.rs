use super::models::{CreateTodoPayload, Todo, UpdateTodoPayload};
use crate::{app_state::AppState, error::AppError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodoPayload,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todos"
)]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoPayload>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (id, title, completed, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, completed, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(payload.title)
    .bind(false)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of todos", body = Vec<Todo>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todos"
)]
pub async fn get_todos(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        "#,
    )
    .fetch_all(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(todos)))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo id")
    ),
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todos"
)]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo id")
    ),
    request_body = UpdateTodoPayload,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todos"
)]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodoPayload>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET title = COALESCE($1, title), completed = COALESCE($2, completed), updated_at = $3
        WHERE id = $4
        RETURNING id, title, completed, created_at, updated_at
        "#,
    )
    .bind(payload.title)
    .bind(payload.completed)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id" = Uuid, Path, description = "Todo id")
    ),
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Todos"
)]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Todo not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
