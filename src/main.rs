use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use log::{debug, info};
use rand::prelude::*;
use simple_logger::SimpleLogger;
use state::AppState;
use std::{
    env,
    path::{Path, PathBuf},
};
use tera::Tera;
use tower_http::services::ServeDir;

mod controllers;
mod models;
mod state;
mod views;

type Error = Box<dyn std::error::Error>;

// #[shuttle_runtime::main]
// async fn init_shuttle(
//     #[shuttle_secrets::Secrets] secrets: SecretStore,
//     #[shuttle_turso::Turso(
//         addr = "libsql://choice-shredder-drewmcarthur.turso.io",
//         local_addr = "libsql://choice-shredder-drewmcarthur.turso.io",
//         token = "{secrets.DB_TURSO_TOKEN}"
//     )]
//     turso: libsql_client::Client,
//     #[shuttle_static_folder::StaticFolder(folder = "src/ui/")] ui_dir: PathBuf,
// ) -> ShuttleAxum {
//     Ok(init_router(turso, &ui_dir).await.unwrap().into())
// }

async fn init_router(turso: libsql_client::Client, ui_dir: &PathBuf) -> Result<Router, Error> {
    info!("intializing appstate");
    let templates = init_templates(&ui_dir).unwrap();
    let static_dir = ui_dir.join("static");
    init_db(&turso).await.expect("DB initialization failed :(");
    let state = AppState::new(turso, templates);
    info!("done intializing appstate");

    info!("intializing router");
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
        .route(
            "/auth/password/registration/create",
            post(controllers::auth::create_password_registration),
        )
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer())
        .layer(Extension(state));

    info!("done initializing router.");
    Ok(router)
}

async fn init_db(client: &libsql_client::Client) -> Result<(), Error> {
    info!("initializing db");
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

    client
        .execute(create_users_table)
        .await
        .expect("error creating users table");
    client
        .execute(create_keys_table)
        .await
        .expect("error creating keys table");

    Ok(())
}

fn init_session_layer() -> SessionLayer<MemoryStore> {
    info!("initializing session memorystore");
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();

    SessionLayer::new(store, &secret)
        .with_cookie_name("webauthnrs")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true)
}

fn init_templates(ui_dir: &PathBuf) -> Result<Tera, Error> {
    info!("initializing templates...");
    info!("checking if src/ui exists: {}", ui_dir.exists());
    let templates_dir = ui_dir.join("templates");
    let templates_pattern = format!("{}/**/*.html", templates_dir.display());
    for file in glob::glob(&templates_pattern).unwrap() {
        let file = file.unwrap();
        debug!("{:?}: {}", file, file.exists());
    }
    let mut templates =
        Tera::parse(templates_pattern.as_str()).expect("Error parsing templates directory");
    templates
        .build_inheritance_chains()
        .expect("Error building tera inheritance chains");
    info!("done initializing templates.");
    Ok(templates)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_logger();
    let ui_dir = Path::new("src").join("ui");
    info!("ui dir exists? {}", ui_dir.exists());
    let db_client = init_db_client().await.unwrap();
    let router = init_router(db_client, &ui_dir).await.unwrap();
    let port = env::var("PORT").unwrap_or("8080".to_string());
    serve(router, port).await.unwrap();
}

async fn init_db_client() -> Result<libsql_client::Client, Error> {
    let db_url = "libsql://choice-shredder-drewmcarthur.turso.io";
    // let token = env::var("DB_TURSO_TOKEN").expect("error loading env.DB_TURSO_TOKEN");
    let config = libsql_client::Config {
        url: url::Url::parse(db_url).expect("error parsing turso db url"),
        // auth_token: Some(token),
        auth_token: None,
    };
    let client = libsql_client::Client::from_config(config).await.unwrap();
    Ok(client)
}
fn init_logger() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
}

async fn serve(router: Router, port: String) -> Result<(), Error> {
    info!("router initialized, listening on :{}", port);
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
