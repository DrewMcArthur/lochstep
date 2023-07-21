use argon2::{password_hash::SaltString, PasswordHasher};
use libsql_client::{args, Client, Statement};
use log::debug;
use rand::rngs::OsRng;
use uuid::Uuid;

use crate::errors::Errors;

// returns UUID if valid, error otherwise
// TODO: clean this up with map/map_err etc
pub(crate) async fn validate_password(
    db: &Client,
    username: &str,
    password: &str,
) -> Result<Uuid, Errors> {
    let stmt = Statement::with_args(
        "SELECT salt FROM users WHERE username = ?;",
        args!(username),
    );
    debug!("stmt: {}", stmt);
    let rows = match db.execute(stmt).await {
        Ok(rs) => rs.rows,
        Err(e) => {
            return Err(Errors::DbFetchError(e));
        }
    };
    let salt: &str = match rows.first() {
        Some(row) => row.try_column("salt").unwrap(),
        None => return Err(Errors::DbUserNotFound(username.to_string())),
    };
    let salt = match SaltString::from_b64(salt) {
        Ok(s) => s,
        Err(e) => return Err(Errors::DbStoredSaltParsingError(e)),
    };
    debug!("got salt({}) for username: {}", salt, username);
    let hash = match get_hash(password, &salt) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };

    let stmt = Statement::with_args(
        "SELECT id FROM users WHERE username=? AND hash=?;",
        args!(username, hash.to_string()),
    );
    debug!("stmt: {}", stmt);
    let rows = match db.execute(stmt).await {
        Ok(rs) => rs.rows,
        Err(e) => return Err(Errors::DbFetchError(e)),
    };
    if rows.len() != 1 {
        return Err(Errors::DbNoHashMatch(username.to_string()));
    }

    let id: &str = rows
        .first()
        .ok_or(Errors::DbNoHashMatch(username.to_string()))?
        .try_column("id")
        .map_err(Errors::DbStoredUuidParsingError)?;

    let id = Uuid::parse_str(id).map_err(Errors::UuidParsingError)?;

    Ok(id)
}

pub(super) fn get_hash<'a>(
    pw: &str,
    salt: &'a SaltString,
) -> Result<argon2::PasswordHash<'a>, Errors> {
    match argon2::Argon2::default().hash_password(pw.as_bytes(), salt) {
        Ok(h) => Ok(h),
        Err(e) => Err(Errors::GetHashError(e)),
    }
}

pub(super) fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}
