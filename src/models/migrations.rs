use libsql_client::{args, Statement};

use crate::errors::Errors;

static GET_LATEST_MIGRATION: &str = "SELECT id FROM migrations ORDER BY id DESC LIMIT 1;";

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
            migrations_executed += 1;
        }
    }

    // TODO: should condition instead be migrations_executed > 0?
    if migrations.len() > latest_migration {
        set_latest(client, migrations.len())
            .await
            .expect("error setting latest migration");
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

async fn set_latest(client: &libsql_client::Client, id: usize) -> Result<(), Errors> {
    let stmt = Statement::with_args("INSERT INTO migrations (id) VALUES (?)", args![id]);
    client
        .execute(stmt)
        .await
        .map(|_| ())
        .map_err(|e| Errors::DbInsertError(e))
}

#[cfg(test)]
mod tests {
    use super::super::MIGRATIONS;
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
