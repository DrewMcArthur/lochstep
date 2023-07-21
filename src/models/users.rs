use std::sync::Arc;

use log::debug;
use uuid::Uuid;

pub async fn create_user(
    client: &Arc<libsql_client::Client>,
    username: &str,
) -> Result<Uuid, crate::Error> {
    let id = Uuid::new_v4();
    debug!("creating user: {}, {}", id, username);
    let stmt = format!(
        "INSERT INTO users (id, username) VALUES (\"{}\", \"{}\");",
        id, username
    );
    debug!("stmt: {}", stmt);

    client.execute(stmt).await.expect("error creating user");

    debug!("user {} created", id);
    Ok(id)
}
