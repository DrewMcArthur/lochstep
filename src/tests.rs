use std::path::{Path, PathBuf};

use axum::Extension;
use http::{Method, Request};
use hyper::Body;
use libsql_client::Client;
use log::info;
use tera::Tera;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use crate::{
    config::{Config, Stage},
    controllers::auth::Login,
    errors::Errors,
    init_session_layer, init_templates, models, routes,
    state::AppState,
    Error,
};

fn get_test_requests() -> Vec<Request<Body>> {
    vec![
        Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap(),
        Request::builder()
            .method(Method::POST)
            .uri("/auth/password/register")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&Login {
                    username: "test".to_string(),
                    password: "test".to_string(),
                })
                .unwrap(),
            ))
            .unwrap(),
        Request::builder()
            .method(Method::POST)
            .uri("/auth/password/login")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&Login {
                    username: "test".to_string(),
                    password: "test".to_string(),
                })
                .unwrap(),
            ))
            .unwrap(),
        Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap(),
    ]
}

fn test_config() -> Config {
    Config {
        db_url: ":memory:".to_string(),
        db_token: None,
        stage: Stage::Test,
        log_level: log::Level::Debug,
    }
}

async fn init_test_db_client() -> Result<Client, Errors> {
    Client::in_memory().map_err(Errors::DbInitializationError)
}

#[tokio::test]
async fn happy_path() -> Result<(), Error> {
    let config = test_config();
    let ui_dir = Path::new("src").join("ui");

    info!("intializing appstate");
    let templates: Tera = match init_templates(&ui_dir) {
        Ok(templates) => templates,
        Err(e) => return Err(e),
    };
    let static_dir: PathBuf = ui_dir.join("static");

    let db_client = init_test_db_client().await.unwrap();
    models::init_db(&db_client).await.unwrap();

    let state: AppState = AppState::new(db_client, templates);
    info!("done intializing appstate");
    let router = routes::router::init()
        .await
        .unwrap()
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer(&config))
        .layer(Extension(state));

    for req in get_test_requests() {
        let path = req.uri().path().to_string();
        let response = router.clone().oneshot(req).await.unwrap();
        assert!(
            response.status().is_success(),
            "route: {}, response status: {}",
            path,
            response.status()
        );
    }
    Ok(())
}
