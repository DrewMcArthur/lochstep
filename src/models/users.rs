use std::sync::Arc;

use uuid::Uuid;

pub async fn create_user(
    client: &Arc<libsql_client::Client>,
    username: &str,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let username = username.to_string();

    client
        .execute(format!(include_str!("db/create_user.sql"), id, username))
        .await
        .unwrap();

    // sqlx::query(include_str!("db/create_user.sql"))
    //     .bind(id)
    //     .bind(username)
    //     .execute(pool)
    //     .await
    //     .unwrap();

    Ok(id)
}
