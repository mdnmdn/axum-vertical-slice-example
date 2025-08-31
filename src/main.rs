use app_state::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio;

mod app_state;
mod config;
mod features;

use features::{
    health::handler::health_check, todos::routes::todo_routes, users::routes::user_routes,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = config::AppConfig::new().unwrap();
    let app_state = AppState::new(&config).await.unwrap();

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(user_routes())
        .merge(todo_routes())
        .with_state(app_state);

    let addr: SocketAddr = config.server_addr.parse().unwrap();
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
