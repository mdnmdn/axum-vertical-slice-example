use super::models::{CreateUserPayload, User, UserResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_user(
    db_pool: &PgPool,
    payload: CreateUserPayload,
) -> Result<UserResponse, sqlx::Error> {
    let password_hash = bcrypt::hash(payload.password, 12).unwrap();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, email, password_hash, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(payload.email)
    .bind(password_hash)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(db_pool)
    .await?;

    Ok(user.into())
}
