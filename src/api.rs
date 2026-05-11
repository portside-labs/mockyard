use axum::{
    extract::Json,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::generator::Generator;
use crate::schema::{OutputFormat, Schema};

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    #[serde(flatten)]
    pub schema: Schema,
}

pub async fn generate(Json(request): Json<GenerateRequest>) -> Response {
    let schema = request.schema;

    // Validate
    if schema.fields.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "At least one field is required"
            })),
        )
            .into_response();
    }

    if schema.num_rows == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "num_rows must be greater than 0"
            })),
        )
            .into_response();
    }

    if schema.num_rows > 100_000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "num_rows must not exceed 100,000"
            })),
        )
            .into_response();
    }

    let mut generator = Generator::new();
    let data = generator.generate(&schema);

    match schema.format {
        OutputFormat::Csv => match data.to_csv() {
            Ok(csv_string) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/csv; charset=utf-8")],
                csv_string,
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to generate CSV: {}", e)
                })),
            )
                .into_response(),
        },
        OutputFormat::Json => match data.to_json() {
            Ok(json_string) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
                json_string,
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to generate JSON: {}", e)
                })),
            )
                .into_response(),
        },
    }
}
