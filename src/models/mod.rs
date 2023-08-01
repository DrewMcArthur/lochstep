use log::info;

use crate::Error;

pub mod db;
#[cfg(passkey)]
pub mod keys;
pub mod passwords;
pub mod proposals;
pub mod users;

pub(crate) async fn init_db(client: &libsql_client::Client) -> Result<(), Error> {
    info!("initializing db");
    let create_users_table = "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE,
            hash TEXT,
            salt TEXT
          );";
    let create_keys_table = "CREATE TABLE IF NOT EXISTS keys (
            id INT PRIMARY KEY,
            userid TEXT,
            key TEXT
          );";

    client
        .execute(create_users_table)
        .await
        .expect("error creating users table");
    client
        .execute(create_keys_table)
        .await
        .expect("error creating keys table");

    info!("done initializing db");
    Ok(())
}
