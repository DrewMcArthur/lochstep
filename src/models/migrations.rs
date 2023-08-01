use libsql_client::{args, Statement};

use crate::errors::Errors;

static GET_LATEST_MIGRATION: &str = "SELECT id FROM migrations ORDER BY id DESC LIMIT 1;";

pub async fn get_latest(client: &libsql_client::Client) -> Result<usize, Errors> {
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

pub async fn set_latest(client: &libsql_client::Client, id: usize) -> Result<(), Errors> {
    let stmt = Statement::with_args("INSERT INTO migrations (id) VALUES (?)", args![id]);
    client
        .execute(stmt)
        .await
        .map(|_| ())
        .map_err(|e| Errors::DbInsertError(e))
}
