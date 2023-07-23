use std::path::Path;

use dotenv::dotenv;
use http::{Method, Request};
use hyper::Body;
use tower::ServiceExt;

use crate::{init_db_client, init_router};

struct TestRoute {
    method: Method,
    uri: String,
    body: Body,
}

fn get_test_routes() -> Vec<TestRoute> {
    vec![TestRoute {
        method: Method::GET,
        uri: "/".to_string(),
        body: Body::empty(),
    }]
}

#[tokio::test]
async fn happy_path() {
    dotenv().unwrap();
    let ui_dir = Path::new("src").join("ui");
    let db = init_db_client().await.unwrap();
    let router = init_router(db, &ui_dir).await.unwrap();

    for route in get_test_routes() {
        let req = Request::builder()
            .method(route.method)
            .uri(route.uri)
            .body(route.body)
            .unwrap();
        let response = router.clone().oneshot(req).await.unwrap();
        assert!(response.status().is_success());
    }
}
