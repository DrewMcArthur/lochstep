use axum::{
    response::{ErrorResponse, Html},
    Extension,
};
use axum_sessions::extractors::ReadableSession;
use http::Request;
use hyper::Body;

use crate::{
    constants::session_keys::AUTH_STATE, controllers::auth::AuthState, errors::Errors,
    handle_error, state::AppState, views,
};

pub mod auth;

pub async fn index(
    Extension(app): Extension<AppState>,
    session: ReadableSession,
    req: Request<Body>,
) -> Result<Html<String>, ErrorResponse> {
    log::debug!("handling request: 'GET /': {:?}", req);
    log::debug!("session: {:?}", session);

    let reg_state = session
        .get_raw(AUTH_STATE)
        .map(|val| serde_json::from_str::<AuthState>(&val))
        .map(|res| {
            res.map_err(|err| {
                handle_error(
                    "error parsing session registration state",
                    Errors::SessionError(err),
                )
            })
        });

    log::debug!("got reg_state: {:?}", reg_state);
    match reg_state {
        Some(Err(err)) => Err(err),
        Some(Ok(auth)) => Ok(views::homepage(app.templates, auth.username)),
        None => Ok(views::login(app.templates)),
    }
}
