use rusqlite::params;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

use crate::DB_LOCATION;

use super::db::Database;

#[derive(Clone)]
pub struct Keys {
    db: Database,
}

impl Keys {
    pub async fn new() -> Self {
        Self {
            db: Database::new(DB_LOCATION).await.unwrap(),
        }
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

    pub async fn add_key(&self, uuid: Uuid, key: Passkey) -> Result<(), rusqlite::Error> {
        self.db
            .connection
            .call(move |conn| {
                conn.execute(
                    include_str!("db/add_key.sql"),
                    params![uuid, serde_json::to_string(&key).unwrap()],
                )
            })
            .await
            .expect("error adding key to db");
        Ok(())
    }
}
