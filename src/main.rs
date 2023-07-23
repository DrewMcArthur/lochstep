use axum::{response::ErrorResponse, Extension, Router};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use errors::Errors;
use hyper::StatusCode;
use log::{error, info};
use rand::prelude::*;
use simple_logger::SimpleLogger;
use state::AppState;
use std::path::{Path, PathBuf};
use tera::Tera;
use tower_http::services::ServeDir;

use crate::{config::{Config, Stage}, state::get_app_port};

#[cfg(passkey)]
use crate::state::init_webauthn;

mod config;
mod constants;
mod controllers;
mod errors;
mod models;
mod routes;
mod state;
mod views;

#[cfg(test)]
mod tests;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Config::from_env();
    init_logger(&config).expect("error initializing logger");

    let ui_dir = Path::new("src").join("ui");
    info!("ui dir exists? {}", ui_dir.exists());

    let db_client = init_db_client(&config)
        .await
        .expect("error initializing db client");

    info!("intializing appstate");
    let templates: Tera = match init_templates(&ui_dir) {
        Ok(templates) => templates,
        Err(e) => return Err(e),
    };
    let static_dir: PathBuf = ui_dir.join("static");

    models::init_db(&db_client).await.unwrap();

    let state: AppState = AppState::new(db_client, templates);
    info!("done intializing appstate");

    let router = routes::init_router()
        .await
        .expect("error initializing router")
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer(&config))
        .layer(Extension(state));

    let port = get_app_port();
    serve(router, port)
        .await
        .expect("error serving router to port");
    Ok(())
}

fn init_session_layer(config: &Config) -> SessionLayer<MemoryStore> {
    info!("initializing session memorystore");
    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();

    SessionLayer::new(store, &secret)
        .with_cookie_name("sid")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(config.stage == Stage::Prod)
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

async fn init_db_client(config: &Config) -> Result<libsql_client::Client, Error> {
    let db_url: &str = config.db_url.as_str();
    let auth_token: Option<String> = config.db_token.clone();

    let url = url::Url::parse(db_url).expect("error parsing turso db url");
    let config = libsql_client::Config { url, auth_token };

    match libsql_client::Client::from_config(config).await {
        Ok(client) => Ok(client),
        Err(e) => Err(format!("error initializing db client: {}", e).into()),
    }
}

fn init_logger(config: &Config) -> Result<(), log::SetLoggerError> {
    SimpleLogger::new()
        .with_level(config.log_level.to_level_filter())
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