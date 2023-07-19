use axum::{debug_handler, Extension, Json};
use axum_sessions::extractors::WritableSession;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::{
    CreationChallengeResponse, PasskeyRegistration, RegisterPublicKeyCredential, WebauthnError,
};

use crate::{models, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct SessionRegistrationState {
    pub username: String,
    pub userid: Uuid,
    reg_state: PasskeyRegistration,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PasskeyRegistrationOptionsRequest {
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasskeyRegistrationOptions {
    id: Uuid,
    username: String,
    registration_result: CreationChallengeResponse,
}

pub async fn get_passkey_registration_options(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
    Json(req): Json<PasskeyRegistrationOptionsRequest>,
) -> Json<CreationChallengeResponse> {
    let userid = models::users::create_user(&app.db, &req.username)
        .await
        .expect("error creating username");

    let challenge =
        generate_passkey_registration_challenge(&app, &mut session, &userid, &req.username)
            .await
            .map_err(|e| e.to_string())
            .expect("error generating passkey challenge");

    Json(challenge)
}

async fn generate_passkey_registration_challenge(
    app: &AppState,
    session: &mut WritableSession,
    userid: &Uuid,
    username: &str,
) -> Result<CreationChallengeResponse, WebauthnError> {
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

    let res =
        match app
            .webauthn
            .start_passkey_registration(userid.clone(), &username, &username, None)
        {
            Ok((ccr, reg_state)) => {
                // Note that due to the session store in use being a server side memory store, this is
                // safe to store the reg_state into the session since it is not client controlled and
                // not open to replay attacks. If this was a cookie store, this would be UNSAFE.
                session
                    .insert(
                        "reg_state",
                        SessionRegistrationState {
                            username: username.to_string(),
                            userid: *userid,
                            reg_state,
                        },
                    )
                    .expect("Failed to insert");
                log::info!("Registration Successful!");
                ccr
            }
            Err(e) => {
                log::debug!("challenge_register -> {:?}", e);
                return Err(e);
            }
        };

    Ok(res)
}

#[debug_handler]
pub async fn create_passkey_registration(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> (StatusCode, String) {
    let session_res = session
        .get("reg_state")
        .ok_or(WebauthnError::CredentialPersistenceError);

    if let Err(e) = session_res {
        log::debug!("challenge_register -> {:?}", e);
        return (StatusCode::BAD_REQUEST, e.to_string());
    }

    let (username, user_unique_id, reg_state): (String, Uuid, PasskeyRegistration) =
        session_res.unwrap();

    session.remove("reg_state");

    let res = match app.webauthn.finish_passkey_registration(&reg, &reg_state) {
        Ok(sk) => {
            //TODO: This is where we would store the credential in a db, or persist them in some other way.
            // users_guard
            //     .keys
            //     .entry(user_unique_id)
            //     .and_modify(|keys| keys.push(sk.clone()))
            //     .or_insert_with(|| vec![sk.clone()]);

            // save key to db
            if let Err(e) = models::keys::add_key(&app.db, user_unique_id, sk).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("error saving passkey to db: {}", e),
                );
            };

            log::info!("saved new key for user {:?}", username);
            StatusCode::OK
        }
        Err(e) => {
            log::debug!("challenge_register -> {:?}", e);
            StatusCode::BAD_REQUEST
        }
    };

    (res, "OK".to_string())
}

pub(crate) async fn create_password_registration() {
    todo!();
}

// async fn register_with_password(
//     db: &Database,
//     userid: &Uuid,
//     username: String,
//     password: String,
// ) -> Result<Json<PasskeyRegistrationOptions>, rusqlite::Error> {
//     let passwords = Passwords::new(db);
//     passwords
//         .update_user_password(userid.clone(), password)
//         .await
//         .unwrap();
//     Ok(Json( {
//         id: userid.clone(),
//         username: username,
//     }))
// }
