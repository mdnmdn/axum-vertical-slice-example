use super::handlers::{create_user, get_user};
use crate::app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
}
