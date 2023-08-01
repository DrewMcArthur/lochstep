use axum::{
    routing::{get, post},
    Router,
};
use log::info;

use crate::{
    controllers::auth::{create_password_registration, login},
    errors::Errors,
    routes::{self, proposals},
};

pub async fn init() -> Result<Router, Errors> {
    info!("intializing router");
    let router = Router::new()
        .route("/", get(routes::root))
        .nest("/auth", auth_router())
        .nest("/proposals", proposals::router());
    info!("done initializing router.");
    Ok(router)
}

#[allow(clippy::let_and_return)]
fn auth_router() -> Router {
    let router = Router::new()
        .route("/password/register", post(create_password_registration))
        .route("/password/login", post(login));

    #[cfg(passkey)]
    router.nest("/passkey", passkey_auth::get_router());

    #[cfg(not(passkey))]
    router
}

#[cfg(passkey)]
fn get_router() -> Router {
    Router::new()
        .route(
            "/registration/options",
            post(get_passkey_registration_options),
        )
        .route("/registration/create", post(create_passkey_registration))
}
