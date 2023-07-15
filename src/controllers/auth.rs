use axum::{response::Html, Extension, Json};
use axum_sessions::extractors::WritableSession;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::{CreationChallengeResponse, WebauthnError};

use crate::{
    models::{db::Database, passwords::Passwords, users::Users},
    state::AppState,
    views,
};

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BeginRegistrationResponse {
    id: Uuid,
    username: String,
    authenticated: bool,
    registration_result: Option<CreationChallengeResponse>,
}

pub async fn begin_register(
    Extension(app): Extension<AppState>,
    Extension(db): Extension<Database>,
    mut session: WritableSession,
    Json(login): Json<Login>,
) -> Result<Html<String>, String> {
    let users = Users::new(&db);
    let userid = users
        .create_user(&login.username)
        .await
        .expect("error creating username");
    if let Some(pw) = login.password {
        if pw.len() > 0 {
            return Err("not supported".to_string());
            // register_with_password(&db, &userid, login.username, pw)
            //     .await
            //     .unwrap();
        }
    }

    let challenge = register_with_passkey(&app, &db, &mut session, &userid, &login.username)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(views::complete_registration_challenge(
        &app.templates,
        challenge,
    ))
}

// pub async fn finish_register(
//     Extension(db): Extension<Database>,
//     Json(registration): Json<RegistrationRequest>,
// ) -> Json<CompletedRegistrationResponse> {
//     WEBAUTHN.finish_passkey_registration()
// }

async fn register_with_password(
    db: &Database,
    userid: &Uuid,
    username: String,
    password: String,
) -> Result<Json<BeginRegistrationResponse>, rusqlite::Error> {
    let passwords = Passwords::new(db);
    passwords
        .update_user_password(userid.clone(), password)
        .await
        .unwrap();
    Ok(Json(BeginRegistrationResponse {
        id: userid.clone(),
        username: username,
        authenticated: true,
        registration_result: None,
    }))
}

async fn register_with_passkey(
    app: &AppState,
    _db: &Database,
    session: &mut WritableSession,
    userid: &Uuid,
    username: &str,
) -> Result<BeginRegistrationResponse, WebauthnError> {
    // log::info!("Start register");
    // We get the username from the URL, but you could get this via form submission or
    // some other process. In some parts of Webauthn, you could also use this as a "display name"
    // instead of a username. Generally you should consider that the user *can* and *will* change
    // their username at any time.

    // Since a user's username could change at anytime, we need to bind to a unique id.
    // We use uuid's for this purpose, and you should generate these randomly. If the
    // username does exist and is found, we can match back to our unique id. This is
    // important in authentication, where presented credentials may *only* provide
    // the unique id, and not the username!

    // TODO: is this a better way to do state/db?
    // let user_unique_id = {
    //     let users_guard = app_state.users.lock().await;
    //     users_guard
    //         .name_to_id
    //         .get(&username)
    //         .copied()
    //         .unwrap_or_else(Uuid::new_v4)
    // };

    // Remove any previous registrations that may have occured from the session.
    session.remove("reg_state");

    // If the user has any other credentials, we exclude these here so they can't be duplicate registered.
    // It also hints to the browser that only new credentials should be "blinked" for interaction.
    // TODO: exclude existing credentials
    // let exclude_credentials = {
    //     let users_guard = app_state.users.lock().await;
    //     users_guard
    //         .keys
    //         .get(&user_unique_id)
    //         .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
    // };

    let res = match app.webauthn.start_passkey_registration(
        userid.clone(),
        &username,
        &username,
        None,
        // exclude_credentials,
    ) {
        Ok((ccr, reg_state)) => {
            // Note that due to the session store in use being a server side memory store, this is
            // safe to store the reg_state into the session since it is not client controlled and
            // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
            session
                .insert("reg_state", (username, userid, reg_state))
                .expect("Failed to insert");
            log::info!("Registration Successful!");
            ccr
        }
        Err(e) => {
            log::debug!("challenge_register -> {:?}", e);
            return Err(e);
        }
    };

    // TODO: instead of returning JSON, return the script that the browser will execute.
    Ok(BeginRegistrationResponse {
        id: userid.clone(),
        username: username.to_string(),
        authenticated: false,
        registration_result: Some(res),
    })
}
