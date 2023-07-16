use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use rand::prelude::*;

use shuttle_axum::ShuttleAxum;
use simple_logger::SimpleLogger;
use state::AppState;
use tower_http::services::ServeDir;

use models::Models;

mod controllers;
mod models;
mod state;
mod views;

static PORT: u16 = 3000;
static DB_LOCATION: &'static str = "src/models/db/db.sqlite";

#[shuttle_runtime::main]
async fn init() -> ShuttleAxum {
    // init_logger();
    let router = routes().await;
    Ok(router.into())
    // start_server(router).await
}

fn init_logger() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
}

async fn routes() -> Router {
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();
    let session_layer = SessionLayer::new(store, &secret)
        .with_cookie_name("webauthnrs")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true);

    let state = AppState::new();
    let models = Models::new().await;

    Router::new()
        .route("/", get(controllers::index))
        .route(
            "/auth/passkey/registration/options",
            post(controllers::auth::get_passkey_registration_options),
        )
        .route(
            "/auth/passkey/registration/create",
            post(controllers::auth::create_passkey_registration),
        )
        // .route(
        //     "/auth/password/registration/create",
        //     post(controllers::auth::create_password_registration),
        // )
        .nest_service("/static", ServeDir::new("ui/static"))
        .layer(session_layer)
        .layer(Extension(state))
        .layer(Extension(models))
}

async fn start_server(app: Router) -> Result<(), shuttle_runtime::Error> {
    let host = "0.0.0.0";
    log::info!("Starting server on http://{}:{}!", host, PORT);

    let addr = format!("{}:{}", host, PORT)
        .parse()
        .expect("failed to parse address");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| shuttle_runtime::Error::Custom(e.into()))
}
