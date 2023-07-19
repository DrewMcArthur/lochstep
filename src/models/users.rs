use std::sync::Arc;

use uuid::Uuid;

pub async fn create_user(
    client: &Arc<libsql_client::Client>,
    username: &str,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let username = username.to_string();

    client
        .execute(format!(
            "INSERT INTO users (id, username) VALUES ({}, {});",
            id, username
        ))
        .await
        .expect("error creating user");

    Ok(id)
}
