use std::sync::Arc;

use log::debug;
use uuid::Uuid;

pub async fn create_user(
    client: &Arc<libsql_client::Client>,
    username: &str,
) -> Result<Uuid, crate::Error> {
    let id = Uuid::new_v4();

    debug!("creating user: {}, {}", id, username);
    client
        .execute(format!(
            "INSERT INTO users (id, username) VALUES ({}, {});",
            id.to_string(),
            username.to_string()
        ))
        .await
        .expect("error creating user");

    debug!("user {} created", id);
    Ok(id)
}
