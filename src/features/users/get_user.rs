use super::models::{User, UserResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user(db_pool: &PgPool, user_id: Uuid) -> Result<UserResponse, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(db_pool)
    .await?;

    Ok(user.into())
}
