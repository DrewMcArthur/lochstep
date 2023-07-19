use axum::{response::Html, Extension};
use axum_sessions::extractors::WritableSession;

use crate::{state::AppState, views};

use self::auth::SessionRegistrationState;

pub mod auth;

pub async fn index(Extension(app): Extension<AppState>, session: WritableSession) -> Html<String> {
    let reg_state: Option<SessionRegistrationState> = session.get("reg_state");
    match reg_state {
        Some(_) => views::homepage(app.templates),
        None => views::login(app.templates),
    }
}
