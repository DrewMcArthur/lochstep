use sqlx::PgPool;
use uuid::Uuid;
use webauthn_rs::prelude::Passkey;

pub async fn update_user_password(
    db: &PgPool,
    userid: Uuid,
    password: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("db/update_user_password.sql"))
        .bind(userid)
        .bind(password)
        .execute(db)
        .await
        .expect("error updating user password");
    Ok(())
}

pub async fn add_key(db: &PgPool, uuid: Uuid, key: Passkey) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("db/add_key.sql"))
        .bind(uuid)
        .bind(serde_json::to_string(&key).unwrap())
        .execute(db)
        .await
        .expect("error adding key to db");
    Ok(())
}
