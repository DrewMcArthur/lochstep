use log::info;

use crate::{models, Error};

pub mod db;
#[cfg(passkey)]
pub mod keys;
mod migrations;
pub mod passwords;
pub mod users;

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

    let latest_migration: usize = models::migrations::get_latest(client).await.unwrap_or(0);

    for (i, query) in MIGRATIONS.iter().enumerate() {
        if i > latest_migration {
            client
                .execute(query)
                .await
                .unwrap_or_else(|e| panic!("error initializing db on query {i}: {e}"));
        }
    }

    models::migrations::set_latest(client, MIGRATIONS.len())
        .await
        .expect("error setting latest migration");
    info!("done initializing db");
    Ok(())
}
