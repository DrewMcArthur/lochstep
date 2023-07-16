use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{
    async_session::{self, MemoryStore},
    SameSite, SessionLayer,
};
use rand::prelude::*;
use shuttle_axum::ShuttleAxum;
use shuttle_service::SecretStore;
use sqlx::PgPool;
use state::AppState;
use std::path::PathBuf;
use tower_http::services::ServeDir;

mod controllers;
mod models;
mod state;
mod views;

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_turso::Turso(
        addr = "libsql://choice-shredder-drewmcarthur.turso.io",
        token = "{secrets.DB_TURSO_TOKEN}"
    )]
    turso: libsql_client::Client,
    // #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "src/ui/static")] static_dir: PathBuf,
) -> ShuttleAxum {
    init_db(&turso).await.expect("DB initialization failed :(");

    log::info!("intializing appstate and router");
    let state = AppState::new(turso);

    let router = Router::new()
        .route("/", get(controllers::index))
        // .route(
        //     "/auth/passkey/registration/options",
        //     post(controllers::auth::get_passkey_registration_options),
        // )
        // .route(
        //     "/auth/passkey/registration/create",
        //     post(controllers::auth::create_passkey_registration),
        // )
        // .route(
        //     "/auth/password/registration/create",
        //     post(controllers::auth::create_password_registration),
        // )
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer())
        .layer(Extension(state));

    log::info!("done initializing.");
    Ok(router.into())
}

async fn init_db(client: &libsql_client::Client) -> Result<(), async_session::Error> {
    let create_users_table = "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT,
            pw TEXT
          );";
    let create_keys_table = "CREATE TABLE IF NOT EXISTS keys (
            id INT PRIMARY KEY,
            userid TEXT,
            key TEXT
          );";
    client.execute(create_users_table).await.unwrap();
    client.execute(create_keys_table).await.unwrap();
    Ok(())
}

fn init_session_layer() -> SessionLayer<MemoryStore> {
    log::info!("initializing session memorystore");
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();

    SessionLayer::new(store, &secret)
        .with_cookie_name("webauthnrs")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true)
}
