use axum::{
    debug_handler,
    response::{ErrorResponse, Html},
    routing::get,
    Router,
};

use crate::{controllers::proposals::proposals_homepage, handle_error};

pub fn router() -> Router {
    Router::new().route("/", get(proposals))
}

#[debug_handler]
async fn proposals() -> Result<Html<String>, ErrorResponse> {
    match proposals_homepage().await {
        Ok(html) => Ok(html),
        Err(e) => Err(handle_error("error rendering proposals homepage", e)),
    }
}
