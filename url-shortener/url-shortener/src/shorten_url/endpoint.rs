use std::ops::Deref;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use url_shortener_lib::{shorten_url, Url, UrlError};

pub fn endpoint() -> Router {
    Router::new().route("/", post(shorten_url_controller))
}

#[derive(Debug, Deserialize)]
struct ShortenUrlRequest {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ShortenUrlResponse {
    url: String,
}

impl IntoResponse for ShortenUrlResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(json!({"url": self.url}))).into_response()
    }
}

enum UrlErrorRequest {
    InvalidUrl,
}

impl IntoResponse for UrlErrorRequest {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, "Invalid url").into_response()
    }
}

#[axum::debug_handler]
async fn shorten_url_controller(
    Json(payload): Json<ShortenUrlRequest>,
) -> Result<ShortenUrlResponse, UrlErrorRequest> {
    let url = Url::from(payload.url.deref());

    match url {
        Ok(u) => Ok(ShortenUrlResponse {
            url: shorten_url(u),
        }),
        Err(_) => Err(UrlErrorRequest::InvalidUrl),
    }
}
