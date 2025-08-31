use app_state::AppState;
use axum::{routing::get, Router};
use clap::Parser;
use config::AppConfig;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app_state;
mod config;
pub mod error;
mod features;

#[cfg(all(test, feature = "integration-test"))]
mod tests;

use features::{health, todos, users};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::handler::health_check,
        users::handlers::create_user,
        users::handlers::get_user,
        todos::handlers::create_todo,
        todos::handlers::get_todos,
        todos::handlers::get_todo,
        todos::handlers::update_todo,
        todos::handlers::delete_todo,
    ),
    components(
        schemas(users::models::UserResponse, users::models::CreateUserPayload, todos::models::Todo, todos::models::CreateTodoPayload, todos::models::UpdateTodoPayload)
    ),
    tags(
        (name = "Rust VSA", description = "A production-ready Rust/Axum project template based on Vertical Slice Architecture.")
    )
)]
struct ApiDoc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run the server without a database connection.
    #[arg(long)]
    no_db: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    dotenvy::dotenv().ok();

    let mut config = config::AppConfig::new().unwrap();

    if cli.no_db {
        config.database_url = "postgres://user:password@localhost/db".to_string();
    }

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.log.level.clone()));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    if cli.no_db {
        run_without_db(config).await;
    } else {
        run_with_db(config).await;
    }
}

async fn run_with_db(config: AppConfig) {
    let app_state = AppState::new(&config).await.unwrap();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(features::health::handler::health_check))
        .merge(features::users::routes::user_routes())
        .merge(features::todos::routes::todo_routes())
        .with_state(app_state);

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .unwrap();
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn run_without_db(config: AppConfig) {
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(features::health::handler::health_check));

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .unwrap();
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
