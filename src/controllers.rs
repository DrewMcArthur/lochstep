use axum::{response::Html, Extension};

use crate::{state::AppState, views};

pub mod auth;

pub async fn index(Extension(app): Extension<AppState>) -> Html<String> {
    let logged_in = &false; // TODO
    views::render_homepage(app.templates, logged_in)
}
