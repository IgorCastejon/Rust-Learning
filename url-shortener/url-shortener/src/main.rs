use std::net::SocketAddr;

mod shorten_url;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("->> LISTENING ON {addr}");

    let app = app(); //.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub fn app() -> Router {
    shorten_url::endpoint::endpoint()
}

/*
#[derive(OpenApi)]
#[openapi(paths(hello_world))]
pub struct ApiDoc;*/
