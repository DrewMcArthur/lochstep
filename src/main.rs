use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use rand::prelude::*;
use shuttle_axum::ShuttleAxum;
use sqlx::PgPool;
use state::AppState;
use tower_http::services::ServeDir;

mod controllers;
mod models;
mod state;
mod views;

#[shuttle_runtime::main]
async fn init(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    log::info!("initializing DB");
    sqlx::migrate!("src/models/db/migrations")
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    log::info!("initializing session memorystore");
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();
    let session_layer = SessionLayer::new(store, &secret)
        .with_cookie_name("webauthnrs")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true);

    log::info!("intializing appstate and router");
    let state = AppState::new();

    let router = Router::new()
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
        .layer(Extension(pool));

    log::info!("done initializing.");
    Ok(router.into())
}
