use libsql_client::Client;
use log::debug;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub async fn add_key(db: &Client, uuid: Uuid, key: Passkey) -> Result<(), crate::Error> {
    let stmt = format!(
        "INSERT INTO keys (userid, pubkey) VALUES (\"{}\", \"{}\");",
        uuid,
        serde_json::to_string(&key).expect("error serializing PassKey json")
    );
    debug!("stmt: {}", stmt);
    db.execute(stmt).await.expect("error adding key to db");
    Ok(())
}
