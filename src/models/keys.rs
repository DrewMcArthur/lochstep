use libsql_client::Client;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub async fn update_user_password(
    db: &Client,
    userid: Uuid,
    password: String,
) -> Result<(), crate::Error> {
    let stmt = format!(
        "update users set password = \"{}\" where id = \"{}\";",
        password, userid
    );
    db.execute(stmt)
        .await
        .expect("error updating user password");
    Ok(())
}

pub async fn add_key(db: &Client, uuid: Uuid, key: Passkey) -> Result<(), crate::Error> {
    let stmt = format!(
        "INSERT INTO keys (userid, pubkey) VALUES (\"{}\", \"{}\");",
        uuid,
        serde_json::to_string(&key).expect("error serializing PassKey json")
    );
    db.execute(stmt).await.expect("error adding key to db");
    Ok(())
}
