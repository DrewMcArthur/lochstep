use axum::{
    response::{ErrorResponse, Html},
    routing::get,
    Extension, Json, Router,
};
use axum_sessions::extractors::WritableSession;
use hyper::StatusCode;
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    constants::session_keys::AUTH_STATE, errors::Errors, handle_error, models, state::AppState,
    views,
};

#[cfg(passkey)]
mod passkey_auth;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthState {
    pub username: String,
    pub userid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub(crate) username: String,
    pub(crate) password: String,
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
        models::users::create_user_with_password(&app.db, &req.username, &req.password).await
    {
        return Err(handle_error("Error creating user", e));
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
