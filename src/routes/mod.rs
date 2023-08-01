use axum::{
    response::{ErrorResponse, Html},
    Extension,
};
use axum_sessions::extractors::ReadableSession;
use http::Request;
use hyper::Body;

use crate::{controllers, handle_error, state::AppState};

mod proposals;
pub mod router;

pub async fn root(
    Extension(app): Extension<AppState>,
    session: ReadableSession,
    req: Request<Body>,
) -> Result<Html<String>, ErrorResponse> {
    match controllers::get_index(app, session, req).await {
        Ok(html) => Ok(html),
        Err(e) => Err(handle_error("error rendering index", e)),
    }
}
