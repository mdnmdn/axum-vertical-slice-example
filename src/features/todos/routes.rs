use super::handlers::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use crate::app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", post(create_todo).get(get_todos))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
}
