use axum::{
    response::{ErrorResponse, Html},
    Extension,
};
use axum_sessions::extractors::ReadableSession;
use http::Request;
use hyper::Body;

use crate::{
    constants::session_keys::AUTH_STATE, controllers::auth::AuthState, errors::Errors,
    handle_error, models, state::AppState, views,
};

pub mod auth;

pub async fn index(
    Extension(app): Extension<AppState>,
    session: ReadableSession,
    req: Request<Body>,
) -> Result<Html<String>, ErrorResponse> {
    match get_index(app, session, req).await {
        Ok(html) => Ok(html),
        Err(e) => Err(handle_error("error rendering index", e)),
    }
}

// todo: rename
async fn get_index(
    app: AppState,
    session: ReadableSession,
    req: Request<Body>,
) -> Result<Html<String>, Errors> {
    log::debug!("handling request: 'GET /': {:?}", req);
    log::debug!("session: {:?}", session);

    let reg_state = session
        .get_raw(AUTH_STATE)
        .map(|val| serde_json::from_str::<AuthState>(&val))
        .map(|res| res.map_err(Errors::SessionError));

    log::debug!("got reg_state: {:?}", reg_state);
    match reg_state {
        Some(Err(err)) => Err(err),
        Some(Ok(auth)) => homepage(app, auth.username).await,
        None => Ok(views::login(app.templates)),
    }
}

async fn homepage(app: AppState, name: String) -> Result<Html<String>, Errors> {
    match models::users::all_users(&app.db).await {
        Ok(all_users) => views::homepage(app.templates, name, all_users),
        Err(e) => Err(e),
    }
}
