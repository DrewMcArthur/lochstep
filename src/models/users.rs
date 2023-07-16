use rusqlite::params;
use uuid::Uuid;

use crate::DB_LOCATION;

use super::db::Database;

#[derive(Clone)]
pub struct Users {
    db: Database,
}

impl Users {
    pub async fn new() -> Self {
        Self {
            db: Database::new(DB_LOCATION).await.unwrap(),
        }
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
