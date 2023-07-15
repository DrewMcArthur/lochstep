use axum::Extension;
use rusqlite::params;
use tokio_rusqlite::Connection;

#[derive(Clone)]
pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub async fn new(path: &str) -> Result<Extension<Self>, tokio_rusqlite::Error> {
        let connection = Connection::open(path)
            .await
            .expect("error opening sqlite db connection");
        let db = Database { connection };
        db.init().await.expect("error initializing db");
        Ok(Extension(db))
    }

    async fn init(&self) -> Result<(), tokio_rusqlite::Error> {
        self.connection
            .call(|conn| conn.execute(include_str!("db/schema.sql"), params![]))
            .await
            .expect("error initializing db");
        Ok(())
    }
}
