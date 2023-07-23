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
    init_session_layer, init_templates, models,
    routes::init_router,
    state::AppState,
    Error, errors::Errors,
};

struct TestRoute {
    method: Method,
    uri: String,
    body: Body,
    content_type: String,
}

fn get_test_routes() -> Vec<TestRoute> {
    vec![
        TestRoute {
            method: Method::GET,
            uri: "/".to_string(),
            body: Body::empty(),
            content_type: "text/html; charset=utf-8".to_string(),
        },
        TestRoute {
            method: Method::POST,
            uri: "/auth/password/register".to_string(),
            body: Body::from(
                serde_json::to_string(&Login {
                    username: "test".to_string(),
                    password: "test".to_string(),
                })
                .unwrap(),
            ),
            content_type: "application/json".to_string(),
        },
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
    let router = init_router()
        .await
        .unwrap()
        .nest_service("/static", ServeDir::new(static_dir))
        .layer(init_session_layer(&config))
        .layer(Extension(state));

    for route in get_test_routes() {
        let req = Request::builder()
            .method(route.method)
            .header("Content-Type", route.content_type)
            .uri(&route.uri)
            .body(route.body)
            .unwrap();
        let response = router.clone().oneshot(req).await.unwrap();
        assert!(
            response.status().is_success(),
            "route: {}, response status: {}",
            route.uri,
            response.status()
        );
    }
    Ok(())
}
