use axum::{
    response::{ErrorResponse, Html},
    Extension, Json,
};
use axum_sessions::extractors::WritableSession;
use hyper::StatusCode;
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::{
    CreationChallengeResponse, PasskeyRegistration, RegisterPublicKeyCredential,
};

use crate::{
    constants::session_keys::AUTH_STATE, errors::Errors, handle_error, models, state::AppState,
    views, Error,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionRegistrationState {
    pub username: String,
    pub userid: Uuid,
    reg_state: PasskeyRegistration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthState {
    pub username: String,
    pub userid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
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
) -> Result<(StatusCode, Json<CreationChallengeResponse>), ErrorResponse> {
    let userid = match models::users::create_user(&app.db, &req.username).await {
        Ok(id) => id,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into()),
    };

    let challenge =
        match generate_passkey_registration_challenge(&app, &mut session, &userid, &req.username)
            .await
        {
            Ok(challenge) => challenge,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("error generating challenge: {}", e),
                )
                    .into())
            }
        };

    Ok((StatusCode::OK, Json(challenge)))
}

async fn generate_passkey_registration_challenge(
    app: &AppState,
    session: &mut WritableSession,
    userid: &Uuid,
    username: &str,
) -> Result<CreationChallengeResponse, Error> {
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
            .start_passkey_registration(*userid, username, username, None)
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
                let err_msg = format!(
                    "error generating registration challenge_register -> {:?}",
                    e
                );
                debug!("{}", err_msg);
                return Err(Box::new(e));
            }
        };

    Ok(res)
}

pub async fn create_passkey_registration(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> Result<(), ErrorResponse> {
    let session_res: SessionRegistrationState = match session.get("reg_state") {
        Some(state) => state,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Credentials were not persisted.".to_string(),
            )
                .into())
        }
    };

    session.remove("reg_state");

    match app
        .webauthn
        .finish_passkey_registration(&reg, &session_res.reg_state)
    {
        Ok(sk) => {
            //TODO: This is where we would store the credential in a db, or persist them in some other way.
            // users_guard
            //     .keys
            //     .entry(user_unique_id)
            //     .and_modify(|keys| keys.push(sk.clone()))
            //     .or_insert_with(|| vec![sk.clone()]);

            // save key to db
            if let Err(e) = models::keys::add_key(&app.db, session_res.userid, sk).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("error saving passkey to db: {}", e),
                )
                    .into());
            };

            log::info!("saved new key for user {:?}", session_res.username);
            StatusCode::OK
        }
        Err(e) => {
            log::debug!("error finalizing registrationg: {:?}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                format!("error finalizing registration: {}", e),
            )
                .into());
        }
    };

    Ok(())
}

pub(crate) async fn create_password_registration(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
    Json(req): Json<Login>,
) -> Result<(StatusCode, String), ErrorResponse> {
    debug!("creating password registration");
    session.remove(AUTH_STATE);
    // check if user exists in db, if so login
    // add user/pw to db
    if let Err(e) =
        models::passwords::create_user_with_password(&app.db, &req.username, &req.password).await
    {
        return Err(handle_error("Error creating password", e));
    }
    Ok((StatusCode::ACCEPTED, "Success! Please login.".to_string()))
}

pub async fn login(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
    Json(req): Json<Login>,
) -> Result<(StatusCode, Html<String>), ErrorResponse> {
    // check that username and password are present
    if (req.username.is_empty()) && (req.password.is_empty()) {
        debug!("login attempt error, empty username or password");
        return Err((
            StatusCode::BAD_REQUEST,
            "Username and password are required".to_string(),
        )
            .into());
    }

    // validate password
    let uuid =
        match models::passwords::validate_password(&app.db, &req.username, &req.password).await {
            Ok(uuid) => uuid,
            Err(e) => return Err(handle_error("Error validating password", e)),
        };

    // create session
    let auth_state = AuthState {
        username: req.username,
        userid: uuid,
    };
    if let Err(e) = session.insert(AUTH_STATE, auth_state) {
        return Err(handle_error(
            "Error creating session",
            Errors::SessionError(e),
        ));
    }
    // return tera login success template
    match views::login_success(app.templates) {
        Ok(res) => Ok((StatusCode::ACCEPTED, res)),
        Err(e) => Err(handle_error("Error rendering login template", e)),
    }
}
