use axum_sessions::async_session::chrono;
use libsql_client::{args, Statement};

use crate::errors::Errors;

mod queries;

// this array should only ever be added to; never changed
pub static MIGRATIONS: [&str; 4] = [
    queries::CREATE_MIGRATIONS_TABLE,
    queries::CREATE_USERS_TABLE,
    queries::CREATE_KEYS_TABLE,
    queries::CREATE_PROPOSALS_TABLE,
];

pub async fn migrate_db(
    client: &libsql_client::Client,
    migrations: &Vec<&str>,
) -> Result<usize, Errors> {
    let mut migrations_executed: usize = 0;
    let latest_migration = get_latest(client).await;
    let mut i = latest_migration.unwrap_or(0);

    while i < migrations.len() {
        let query = migrations[i];
        exec_migration(client, query).await.unwrap();
        add(client, i + 1, query)
            .await
            .expect("error updating migrations table");

        i += 1;
        migrations_executed += 1;
    }

    Ok(migrations_executed)
}

async fn get_latest(client: &libsql_client::Client) -> Result<usize, Errors> {
    client
        .execute(queries::GET_LATEST_MIGRATION)
        .await
        .map(|rs: libsql_client::ResultSet| {
            rs.rows
                .first()
                .map(|row: &libsql_client::Row| row.try_get(0).unwrap())
                .unwrap_or(0)
        })
        .map_err(|e| Errors::DbFetchLatestMigrationError(e))
}

async fn exec_migration(client: &libsql_client::Client, query: &str) -> Result<(), Errors> {
    client
        .execute(query)
        .await
        .unwrap_or_else(|e| panic!("error initializing db on query {query}: {e}"));

    Ok(())
}

async fn add(client: &libsql_client::Client, id: usize, query: &str) -> Result<(), Errors> {
    let now = chrono::offset::Utc::now().to_rfc3339();
    let stmt = Statement::with_args(queries::ADD_MIGRATION, args![id, now, query]);
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
    async fn test_migrate_db() {
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
