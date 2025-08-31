use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::features::{todos::routes::todo_routes, users::routes::user_routes};
use axum::{routing::get, Router};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let pg_image = Postgres::default();
    let postgres_container = pg_image.start().await.unwrap();
    let port = postgres_container.get_host_port_ipv4(5432).await.unwrap();
    let database_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let mut config = AppConfig::new().unwrap();
    config.database_url = database_url;

    let mut conn = PgConnection::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let db_name = "test";
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database");

    let database_url = format!("{}/{}", config.database_url, db_name);
    config.database_url = database_url;

    let db_pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    let app_state = AppState {
        db_pool: db_pool.clone(),
    };

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(user_routes())
        .merge(todo_routes())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    let server_addr = format!("http://{}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestApp {
        address: server_addr,
        db_pool,
    }
}
