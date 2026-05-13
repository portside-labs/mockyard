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

    for field in &schema.fields {
        // Validate enum weights
        if matches!(field.field_type, crate::schema::FieldType::Enum) {
            let total_weight: f64 = field
                .options
                .values
                .iter()
                .filter_map(|v| v.weight)
                .sum();
            if total_weight > 100.0 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": format!(
                            "Field \"{}\": enum weights total {}%, must not exceed 100%",
                            field.name, total_weight
                        )
                    })),
                )
                    .into_response();
            }
        }

        // Validate lookup fields
        if matches!(field.field_type, crate::schema::FieldType::Lookup) {
            if field.options.columns.is_empty() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": format!(
                            "Lookup field \"{}\": columns must not be empty",
                            field.name
                        )
                    })),
                )
                    .into_response();
            }
            let col_count = field.options.columns.len();
            for (i, row) in field.options.data.iter().enumerate() {
                if row.len() != col_count {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "error": format!(
                                "Lookup field \"{}\": data row {} has {} values, expected {} (matching columns)",
                                field.name, i + 1, row.len(), col_count
                            )
                        })),
                    )
                        .into_response();
                }
            }
        }
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
