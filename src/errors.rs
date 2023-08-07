use std::fmt::Display;

#[derive(Debug)]
pub enum Errors {
    DbStoredSaltParsingError(argon2::password_hash::Error),
    DbStoredUuidParsingError(anyhow::Error),
    DbStoredUuidWrongTypeError(),
    DbStoredUsernameWrongTypeError(),
    DbFetchLatestMigrationError(anyhow::Error),
    UuidParsingError(uuid::Error),
    GetHashError(argon2::password_hash::Error),
    DbFetchError(anyhow::Error),
    DbUserNotFound(String),
    DbNoHashMatch(String),
    DbInsertError(anyhow::Error),
    DbMissingUuid(String),
    DbInitializationError(anyhow::Error),
    LoginErrorUsernameOrPasswordMissing,
    RenderingError(String, tera::Error),
    SessionError(serde_json::Error),
    UserAlreadyExists(String),
    StageParseError,
    Default,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}
