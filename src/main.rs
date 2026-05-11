mod api;
mod generator;
mod schema;

use axum::{
    Router,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use rust_embed::Embed;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

#[derive(Embed)]
#[folder = "static/"]
struct StaticAssets;

async fn serve_index() -> Response {
    match StaticAssets::get("index.html") {
        Some(content) => {
            let html = String::from_utf8_lossy(content.data.as_ref()).to_string();
            Html(html).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

async fn serve_static(axum::extract::Path(path): axum::extract::Path<String>) -> Response {
    match StaticAssets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data.to_vec(),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("mockyard=info".parse().unwrap()))
        .init();

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/v1/generate", post(api::generate))
        .route("/static/{*path}", get(serve_static))
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Mockyard starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
