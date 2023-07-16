use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(pool: &PgPool, username: &str) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let username = username.to_string();

    sqlx::query(include_str!("db/create_user.sql"))
        .bind(id)
        .bind(username)
        .execute(pool)
        .await
        .unwrap();

    Ok(id)
}
