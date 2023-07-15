use rusqlite::params;
use uuid::Uuid;

use super::db::Database;

pub struct Passwords<'db> {
    db: &'db Database,
}

impl<'db> Passwords<'db> {
    pub fn new(db: &'db Database) -> Self {
        Self { db }
    }

    pub async fn update_user_password(
        &self,
        userid: Uuid,
        password: String,
    ) -> Result<(), rusqlite::Error> {
        self.db
            .connection
            .call(move |conn| {
                conn.execute(
                    include_str!("db/update_user_password.sql"),
                    params![userid, password],
                )
            })
            .await
            .expect("error updating user password");

        Ok(())
    }
}
