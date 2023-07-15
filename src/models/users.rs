use rusqlite::params;
use uuid::Uuid;

use super::db::Database;

pub struct Users<'db> {
    db: &'db Database,
}

impl<'db> Users<'db> {
    pub fn new(db: &'db Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: &str) -> Result<Uuid, rusqlite::Error> {
        let id = Uuid::new_v4();
        let username = username.to_string();
        self.db
            .connection
            .call(move |conn| {
                conn.execute(include_str!("db/create_user.sql"), params![id, username])
            })
            .await
            .expect("error executing create_users statement");
        Ok(id)
    }
}
