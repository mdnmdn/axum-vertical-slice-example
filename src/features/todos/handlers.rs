use super::models::{CreateTodoPayload, Todo, UpdateTodoPayload};
use crate::app_state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
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
    .await;

    match todo {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn get_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        "#,
    )
    .fetch_all(&state.db_pool)
    .await;

    match todos {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn get_todo(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at, updated_at
        FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db_pool)
    .await;

    match todo {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodoPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
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
    .await;

    match todo {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.db_pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND).into_response()
            } else {
                (StatusCode::NO_CONTENT).into_response()
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}
