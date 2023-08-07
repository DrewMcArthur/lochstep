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

pub(crate) async fn init_db(client: &libsql_client::Client) -> Result<(), Error> {
    info!("initializing db");

    migrations::migrate_db(client, &migrations::MIGRATIONS.to_vec())
        .await
        .unwrap();

    info!("done initializing db");
    Ok(())
}
