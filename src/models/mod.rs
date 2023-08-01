use log::info;

use crate::Error;

pub mod db;
#[cfg(passkey)]
pub mod keys;
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
static MIGRATIONS: [&str; 3] = [
    CREATE_USERS_TABLE,
    CREATE_KEYS_TABLE,
    // create proposals tables
    "CREATE TABLE IF NOT EXISTS proposals (
            id INT PRIMARY KEY,
            title TEXT,
            description TEXT,
            authorId TEXT,
            createdAt TEXT,
            updatedAt TEXT
        )",
];

pub(crate) async fn init_db(client: &libsql_client::Client) -> Result<(), Error> {
    info!("initializing db");
    for (i, query) in MIGRATIONS.iter().enumerate() {
        client
            .execute(query)
            .await
            .unwrap_or_else(|e| panic!("error initializing db on query {i}: {e}"));
    }
    info!("done initializing db");
    Ok(())
}
