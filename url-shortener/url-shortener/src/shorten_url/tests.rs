use axum_test::TestServer;
use serde_json::json;

use crate::app;

#[tokio::test]
async fn when_shorten_invalid_url_then_should_return_error() {
    let app = app().into_make_service();
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/")
        .content_type("application/json")
        .json(&json!({
            "url": "teste",
        }))
        .await;

    response.assert_status_bad_request();
    response.assert_text("Invalid url")
}

#[tokio::test]
async fn when_shorten_google_then_should_return_encoded_url() {
    let app = app().into_make_service();
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/")
        .content_type("application/json")
        .json(&json!({
            "url": "https://google.com.br",
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "url": "teste",
    }))
}
