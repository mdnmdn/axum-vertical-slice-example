use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
        Ok(Self { db_pool })
    }
}
