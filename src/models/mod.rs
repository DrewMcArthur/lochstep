use log::info;

use crate::Error;

pub mod db;
#[cfg(passkey)]
pub mod keys;
mod migrations;
pub mod passwords;
pub mod users;

#[cfg(test)]
mod test;

static CREATE_USERS_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        username TEXT UNIQUE,
        hash TEXT,
        salt TEXT
        );";
static CREATE_KEYS_TABLE: &str = "CREATE TABLE IF NOT EXISTS keys (
        id INT PRIMARY KEY,
        userid TEXT,
        key TEXT
        );";

// this array should not be changed, only added to
static MIGRATIONS: [&str; 4] = [
    CREATE_USERS_TABLE,
    CREATE_KEYS_TABLE,
    "CREATE TABLE IF NOT EXISTS migrations (
        id INT PRIMARY KEY
    );",
    // create proposals tables
    "CREATE TABLE IF NOT EXISTS proposals (
        id INT PRIMARY KEY,
        title TEXT,
        description TEXT,
        authorId TEXT,
        createdAt TEXT,
        updatedAt TEXT
    );",
];

pub(crate) async fn init_db(client: &libsql_client::Client) -> Result<(), Error> {
    info!("initializing db");

    migrations::migrate_db(client, &MIGRATIONS.to_vec())
        .await
        .unwrap();

    info!("done initializing db");
    Ok(())
}
