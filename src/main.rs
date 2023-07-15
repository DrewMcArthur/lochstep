use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use rand::prelude::*;

use simple_logger::SimpleLogger;
use state::AppState;
use tower_http::services::ServeDir;

use models::db::Database;

mod controllers;
mod models;
mod state;
mod views;

static PORT: u16 = 3000;
static DB_LOCATION: &'static str = "src/models/db/db.sqlite";

#[tokio::main]
async fn main() {
    init_logger();
    let router = routes().await;
    start_server(router).await;
}

fn init_logger() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
}

async fn routes() -> Router {
    let db = Database::new(DB_LOCATION).await.expect("error creating db");

    let store = MemoryStore::new();
    let secret1 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret2 = thread_rng().gen::<[u8; 32]>(); // MUST be at least 64 bytes!
    let secret = [secret1, secret2].concat();
    let session_layer = SessionLayer::new(store, &secret)
        .with_cookie_name("webauthnrs")
        .with_same_site_policy(SameSite::Lax)
        .with_secure(true);

    let state = AppState::new();

    Router::new()
        .route("/", get(controllers::index))
        .route(
            "/auth/begin-register",
            post(controllers::auth::begin_register),
        )
        .nest_service("/static", ServeDir::new("ui/static"))
        .layer(db)
        .layer(session_layer)
        .layer(Extension(state))
}

async fn start_server(app: Router) -> () {
    let host = "0.0.0.0";
    log::info!("Starting server on http://{}:{}!", host, PORT);

    let addr = format!("{}:{}", host, PORT)
        .parse()
        .expect("failed to parse address");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
