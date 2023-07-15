use axum::{response::Html, Extension};
use axum_sessions::extractors::WritableSession;

use crate::{state::AppState, views};

pub mod auth;

pub async fn index(
    Extension(app): Extension<AppState>,
    mut session: WritableSession,
) -> Html<String> {
    let logged_in = false; // TODO
    if logged_in {
        views::homepage(app.templates)
    } else {
        views::login(app.templates)
    }
}
