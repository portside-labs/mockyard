pub mod api;
pub mod generator;
pub mod schema;

mod routes {
    use axum::{
        Router,
        http::{StatusCode, header},
        response::{Html, IntoResponse, Response},
        routing::{get, post},
    };
    use rust_embed::Embed;
    use tower_http::cors::CorsLayer;

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

    async fn serve_docs() -> Response {
        match StaticAssets::get("docs.html") {
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

    pub fn build_router() -> Router {
        Router::new()
            .route("/", get(serve_index))
            .route("/docs", get(serve_docs))
            .route("/v1/generate", post(super::api::generate))
            .route("/static/{*path}", get(serve_static))
            .layer(CorsLayer::permissive())
    }
}

pub fn build_router() -> axum::Router {
    routes::build_router()
}
