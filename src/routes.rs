use axum::{
    routing::{get, post},
    Router,
};
use log::info;

use crate::{
    controllers::{
        self,
        auth::{create_password_registration, login},
    },
    Error,
};

pub async fn init_router() -> Result<Router, Error> {
    info!("intializing router");
    let router = Router::new()
        .route("/", get(controllers::index))
        .nest("/auth", auth_router());
    info!("done initializing router.");
    Ok(router)
}

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
