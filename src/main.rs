use axum::{response::ErrorResponse, routing::get, Extension, Router};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use errors::Errors;
use hyper::StatusCode;
use log::{error, info};
use rand::prelude::*;
use simple_logger::SimpleLogger;
use state::AppState;
use std::{
    env,
    path::{Path, PathBuf},
};
use tera::Tera;
use tower_http::services::ServeDir;

use crate::state::get_app_port;

#[cfg(passkey)]
use crate::state::init_webauthn;

#[cfg(test)]
mod tests;

mod constants;
mod controllers;
mod errors;
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_logger().expect("error initializing logger");

    let ui_dir = Path::new("src").join("ui");
    info!("ui dir exists? {}", ui_dir.exists());

    let db_client = init_db_client()
        .await
        .expect("error initializing db client");

    let router = init_router(db_client, &ui_dir)
        .await
        .expect("error initializing router");

    let port = get_app_port();
    serve(router, port)
        .await
        .expect("error serving router to port");
}

async fn init_router(db_client: libsql_client::Client, ui_dir: &Path) -> Result<Router, Error> {
    info!("intializing appstate");
    let templates: Tera = match init_templates(ui_dir) {
        Ok(templates) => templates,
        Err(e) => return Err(e),
    };
    let static_dir: PathBuf = ui_dir.join("static");

    models::init_db(&db_client).await?;

    let state: AppState = AppState::new(db_client, templates);
    info!("done intializing appstate");

    info!("intializing router");
    let router = Router::new()
        .route("/", get(controllers::index))
        .nest("/auth", controllers::auth::get_router())
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer())
        .layer(Extension(state));

    info!("done initializing router.");
    Ok(router)
}

fn init_session_layer() -> SessionLayer<MemoryStore> {
    info!("initializing session memorystore");
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();

    SessionLayer::new(store, &secret)
        .with_cookie_name("sid")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true) // TODO: set this to true iff prod
}

fn init_templates(ui_dir: &Path) -> Result<Tera, Error> {
    info!("initializing templates...");
    let templates_dir = ui_dir.join("templates");
    let templates_pattern = format!("{}/**/*.html", templates_dir.display());
    let mut templates =
        Tera::parse(templates_pattern.as_str()).expect("Error parsing templates directory");
    templates
        .build_inheritance_chains()
        .expect("Error building tera inheritance chains");
    info!("done initializing templates.");
    Ok(templates)
}

async fn init_db_client() -> Result<libsql_client::Client, Error> {
    let db_url = "libsql://choice-shredder-drewmcarthur.turso.io";
    let token = env::var("DB_TURSO_TOKEN").expect("error loading env.DB_TURSO_TOKEN");
    let config = libsql_client::Config {
        url: url::Url::parse(db_url).expect("error parsing turso db url"),
        auth_token: Some(token),
    };

    match libsql_client::Client::from_config(config).await {
        Ok(client) => Ok(client),
        Err(e) => Err(format!("error initializing db client: {}", e).into()),
    }
}

fn init_logger() -> Result<(), log::SetLoggerError> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .with_module_level("hyper", log::LevelFilter::Info)
        .with_module_level("h2", log::LevelFilter::Info)
        .with_module_level("rustls", log::LevelFilter::Info)
        .init()
}

async fn serve(router: Router, port: String) -> Result<(), Error> {
    info!("router initialized, listening on :{}", port);
    match axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(router.into_make_service())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn handle_error(err_msg: &str, e: Errors) -> ErrorResponse {
    let err_msg = format!("{}: {}", err_msg, e);
    error!("{}", err_msg);
    (StatusCode::INTERNAL_SERVER_ERROR, err_msg).into()
}
