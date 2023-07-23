use argon2::{password_hash::SaltString, PasswordHash};
use libsql_client::{args, Client, Statement};
use log::debug;
use uuid::Uuid;
#[cfg(passkey)]
use std::sync::Arc;

use crate::{errors::Errors, models::passwords};

#[cfg(passkey)]
pub async fn create_user(
    client: &Arc<libsql_client::Client>,
    username: &str,
) -> Result<Uuid, crate::Error> {
    let id: Uuid = Uuid::new_v4();
    debug!("creating user: {}, {}", id, username);
    let stmt: String = format!(
        "INSERT INTO users (id, username) VALUES (\"{}\", \"{}\");",
        id, username
    );
    debug!("stmt: {}", stmt);

    client.execute(stmt).await.expect("error creating user");

    debug!("user {} created", id);
    Ok(id)
}

pub async fn create_user_with_password(
    db: &Client,
    username: &str,
    password: &str,
) -> Result<(), Errors> {
    if user_exists(db, username).await? {
        return Err(Errors::UserAlreadyExists(username.to_owned()));
    }

    let uuid: Uuid = Uuid::new_v4();
    let salt: SaltString = passwords::generate_salt();
    let hash: PasswordHash<'_> = passwords::get_hash(password, &salt)?;

    let stmt: Statement = libsql_client::Statement::with_args(
        "INSERT INTO users (id, username, hash, salt) VALUES (?,?,?,?);",
        args!(
            uuid.to_string(),
            username,
            hash.to_string(),
            salt.to_string()
        ),
    );

    debug!("stmt: {}", stmt);
    db.execute(stmt)
        .await
        .map_err(Errors::DbInsertError)
        .map(|_| ())
}

async fn user_exists(db: &Client, username: &str) -> Result<bool, Errors> {
    let stmt = Statement::with_args("SELECT id FROM users WHERE username = ?;", args!(username));
    debug!("stmt: {}", stmt);
    db.execute(stmt)
        .await
        .map_err(Errors::DbFetchError)
        .map(|rs| rs.rows.len())
        .map(|num_users| num_users > 0)
}
