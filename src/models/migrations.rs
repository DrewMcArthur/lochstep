use axum_sessions::async_session::chrono;
use libsql_client::{args, Statement};

use crate::errors::Errors;

static GET_LATEST_MIGRATION: &str = "SELECT id FROM migrations ORDER BY id DESC LIMIT 1;";
static ADD_MIGRATION: &str = "INSERT INTO migrations (id, date, query) VALUES (?, ?, ?);";

static CREATE_MIGRATIONS_TABLE: &str = "CREATE TABLE IF NOT EXISTS migrations (
        id INT PRIMARY KEY,
        date TEXT,
        query TEXT
    );";
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
static CREATE_PROPOSALS_TABLE: &str = "CREATE TABLE IF NOT EXISTS proposals (
        id INT PRIMARY KEY,
        title TEXT,
        description TEXT,
        authorId TEXT,
        createdAt TEXT,
        updatedAt TEXT
    );";

// this array should only ever be added to; never changed
pub static MIGRATIONS: [&str; 4] = [
    CREATE_MIGRATIONS_TABLE,
    CREATE_USERS_TABLE,
    CREATE_KEYS_TABLE,
    CREATE_PROPOSALS_TABLE,
];

pub async fn migrate_db(
    client: &libsql_client::Client,
    migrations: &Vec<&str>,
) -> Result<usize, Errors> {
    let mut migrations_executed: usize = 0;
    let latest_migration: usize = get_latest(client).await.unwrap_or(0);

    for (i, query) in migrations.iter().enumerate() {
        if i >= latest_migration {
            client
                .execute(query)
                .await
                .unwrap_or_else(|e| panic!("error initializing db on query {i}: {e}"));

            add(client, i + 1, query)
                .await
                .expect("error updating migrations table");

            migrations_executed += 1;
        }
    }

    Ok(migrations_executed)
}

async fn get_latest(client: &libsql_client::Client) -> Result<usize, Errors> {
    client
        .execute(GET_LATEST_MIGRATION)
        .await
        .map(|rs: libsql_client::ResultSet| {
            rs.rows
                .first()
                .map(|row: &libsql_client::Row| row.try_get(0).unwrap())
                .unwrap_or(0)
        })
        .map_err(|e| Errors::DbFetchLatestMigrationError(e))
}

async fn add(client: &libsql_client::Client, id: usize, query: &str) -> Result<(), Errors> {
    let now = chrono::offset::Utc::now().to_rfc3339();
    let stmt = Statement::with_args(ADD_MIGRATION, args![id, now, query]);
    client
        .execute(stmt)
        .await
        .map(|_| ())
        .map_err(|e| Errors::DbInsertError(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db() {
        let client = libsql_client::Client::in_memory().unwrap();
        let mut migrations = MIGRATIONS.to_vec();
        assert!(get_latest(&client).await.is_err());

        let num_executions = migrate_db(&client, &migrations).await.unwrap();
        assert_eq!(num_executions, 4);
        assert_eq!(get_latest(&client).await.unwrap(), 4);

        let num_executions = migrate_db(&client, &migrations).await.unwrap();
        assert_eq!(num_executions, 0);
        assert_eq!(get_latest(&client).await.unwrap(), 4);

        migrations.push("CREATE TABLE IF NOT EXISTS test_table (id INT PRIMARY KEY);");
        let num_executions = migrate_db(&client, &migrations).await.unwrap();
        assert_eq!(num_executions, 1);
        assert_eq!(get_latest(&client).await.unwrap(), 5);
    }
}