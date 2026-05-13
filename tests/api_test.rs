use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn app() -> axum::Router {
    mockyard::build_router()
}

async fn post_generate(body: &str) -> (StatusCode, String) {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    (status, text)
}

fn error_message(body: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    v["error"].as_str().unwrap().to_string()
}

// ── Validation Tests ──

#[tokio::test]
async fn rejects_empty_fields() {
    let (status, body) = post_generate(r#"{"fields": [], "num_rows": 10, "format": "csv"}"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(error_message(&body), "At least one field is required");
}

#[tokio::test]
async fn rejects_zero_rows() {
    let (status, body) = post_generate(r#"{
        "fields": [{"name": "id", "type": "row_number", "options": {}}],
        "num_rows": 0,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(error_message(&body), "num_rows must be greater than 0");
}

#[tokio::test]
async fn rejects_too_many_rows() {
    let (status, body) = post_generate(r#"{
        "fields": [{"name": "id", "type": "row_number", "options": {}}],
        "num_rows": 10000001,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(error_message(&body), "num_rows must not exceed 10,000,000");
}

#[tokio::test]
async fn accepts_max_rows() {
    let (status, _) = post_generate(r#"{
        "fields": [{"name": "id", "type": "row_number", "options": {}}],
        "num_rows": 100000,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn accepts_one_row() {
    let (status, _) = post_generate(r#"{
        "fields": [{"name": "id", "type": "row_number", "options": {}}],
        "num_rows": 1,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn rejects_enum_weights_over_100() {
    let (status, body) = post_generate(r#"{
        "fields": [{
            "name": "role",
            "type": "enum",
            "options": {
                "values": [
                    {"value": "A", "weight": 60},
                    {"value": "B", "weight": 50}
                ]
            }
        }],
        "num_rows": 10,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let msg = error_message(&body);
    assert!(msg.contains("role"), "Error should mention field name: {}", msg);
    assert!(msg.contains("110"), "Error should mention total weight: {}", msg);
}

#[tokio::test]
async fn accepts_enum_weights_at_100() {
    let (status, _) = post_generate(r#"{
        "fields": [{
            "name": "role",
            "type": "enum",
            "options": {
                "values": [
                    {"value": "A", "weight": 50},
                    {"value": "B", "weight": 50}
                ]
            }
        }],
        "num_rows": 5,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn accepts_enum_with_unweighted_values() {
    let (status, _) = post_generate(r#"{
        "fields": [{
            "name": "role",
            "type": "enum",
            "options": {
                "values": [
                    {"value": "A", "weight": 20},
                    {"value": "B"},
                    {"value": "C"}
                ]
            }
        }],
        "num_rows": 5,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn rejects_invalid_json() {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from("not json"))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert!(
        resp.status() == StatusCode::BAD_REQUEST || resp.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Expected 400 or 422 for invalid JSON, got {}", resp.status()
    );
}

#[tokio::test]
async fn rejects_invalid_field_type() {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "fields": [{"name": "x", "type": "nonexistent", "options": {}}],
            "num_rows": 1,
            "format": "csv"
        }"#))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// ── CSV Response Tests ──

#[tokio::test]
async fn csv_response_has_correct_content_type() {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "fields": [{"name": "id", "type": "row_number", "options": {}}],
            "num_rows": 1,
            "format": "csv"
        }"#))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(ct.contains("text/csv"), "Content-Type should be text/csv: {}", ct);
}

#[tokio::test]
async fn csv_response_has_header_and_data() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "id", "type": "row_number", "options": {}},
            {"name": "name", "type": "first_name", "options": {}}
        ],
        "num_rows": 3,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let lines: Vec<&str> = body.trim().split('\n').collect();
    assert_eq!(lines.len(), 4); // header + 3 rows
    assert_eq!(lines[0], "id,name");
    assert!(lines[1].starts_with("1,"));
    assert!(lines[2].starts_with("2,"));
    assert!(lines[3].starts_with("3,"));
}

// ── JSON Response Tests ──

#[tokio::test]
async fn json_response_has_correct_content_type() {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "fields": [{"name": "id", "type": "row_number", "options": {}}],
            "num_rows": 1,
            "format": "json"
        }"#))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(ct.contains("application/json"), "Content-Type should be application/json: {}", ct);
}

#[tokio::test]
async fn json_response_is_valid_array() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "id", "type": "row_number", "options": {}},
            {"name": "email", "type": "email", "options": {}}
        ],
        "num_rows": 5,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed.len(), 5);
    for row in &parsed {
        assert!(row["id"].is_number());
        assert!(row["email"].is_string());
    }
}

// ── Format defaults to CSV ──

#[tokio::test]
async fn format_defaults_to_csv() {
    let req = Request::builder()
        .method("POST")
        .uri("/v1/generate")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "fields": [{"name": "id", "type": "row_number", "options": {}}],
            "num_rows": 1
        }"#))
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(ct.contains("text/csv"), "Default format should be CSV: {}", ct);
}

// ── Complex Schema ──

#[tokio::test]
async fn complex_schema_generates_all_types() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "id", "type": "row_number", "options": {}},
            {"name": "name", "type": "full_name", "options": {}},
            {"name": "email", "type": "email", "options": {"blank_percentage": 10}},
            {"name": "role", "type": "enum", "options": {"values": [{"value": "Admin", "weight": 10}, {"value": "User"}]}},
            {"name": "active", "type": "boolean", "options": {"true_percentage": 80}},
            {"name": "score", "type": "integer", "options": {"min": -100, "max": 100}},
            {"name": "price", "type": "currency", "options": {"min": 0.99, "max": 99.99, "decimals": 2}},
            {"name": "ip", "type": "ipv4", "options": {}},
            {"name": "date", "type": "date", "options": {}},
            {"name": "uuid", "type": "uuid", "options": {}}
        ],
        "num_rows": 50,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed.len(), 50);

    // Check first row has all fields
    let first = &parsed[0];
    assert!(first["id"].is_number());
    assert!(first["name"].is_string());
    assert!(first["active"].is_boolean());
    assert!(first["ip"].is_string());
    assert!(first["date"].is_string());
    assert!(first["uuid"].is_string());

    // Verify score is in range
    for row in &parsed {
        if let Some(score) = row["score"].as_i64() {
            assert!((-100..=100).contains(&score));
        }
    }

    // Verify price is in range
    for row in &parsed {
        if let Some(price) = row["price"].as_f64() {
            assert!((0.99..=99.99).contains(&price));
        }
    }
}

// ── Static Routes ──

#[tokio::test]
async fn index_returns_html() {
    let req = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(body.contains("mockyard"), "Index should contain mockyard");
}

#[tokio::test]
async fn docs_returns_html() {
    let req = Request::builder()
        .uri("/docs")
        .body(Body::empty())
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(body.contains("API"), "Docs should contain API reference");
}

#[tokio::test]
async fn static_openapi_spec_served() {
    let req = Request::builder()
        .uri("/static/openapi.yaml")
        .body(Body::empty())
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(body.contains("openapi:"), "Should serve OpenAPI spec");
}

#[tokio::test]
async fn static_missing_file_returns_404() {
    let req = Request::builder()
        .uri("/static/nonexistent.txt")
        .body(Body::empty())
        .unwrap();

    let resp = app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── Lookup Tests ──

#[tokio::test]
async fn lookup_produces_correlated_output() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "", "type": "lookup", "options": {
                "columns": ["city", "state"],
                "data": [
                    ["Miami", "Florida"],
                    ["Toronto", "Ontario"]
                ]
            }}
        ],
        "num_rows": 20,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed.len(), 20);
    for row in &parsed {
        let city = row["city"].as_str().unwrap();
        let state = row["state"].as_str().unwrap();
        match city {
            "Miami" => assert_eq!(state, "Florida"),
            "Toronto" => assert_eq!(state, "Ontario"),
            _ => panic!("Unexpected city: {}", city),
        }
    }
}

#[tokio::test]
async fn lookup_with_prefix() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "", "type": "lookup", "options": {
                "prefix": "hq_",
                "columns": ["city", "country"],
                "data": [["Berlin", "DE"]]
            }}
        ],
        "num_rows": 2,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    for row in &parsed {
        assert_eq!(row["hq_city"].as_str().unwrap(), "Berlin");
        assert_eq!(row["hq_country"].as_str().unwrap(), "DE");
    }
}

#[tokio::test]
async fn lookup_csv_has_expanded_columns() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "id", "type": "row_number", "options": {}},
            {"name": "", "type": "lookup", "options": {
                "columns": ["city", "state"],
                "data": [["NYC", "NY"]]
            }}
        ],
        "num_rows": 1,
        "format": "csv"
    }"#).await;
    assert_eq!(status, StatusCode::OK);
    let lines: Vec<&str> = body.trim().split('\n').collect();
    assert_eq!(lines[0], "id,city,state");
}

#[tokio::test]
async fn lookup_rejects_empty_columns() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "loc", "type": "lookup", "options": {
                "columns": [],
                "data": []
            }}
        ],
        "num_rows": 1,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let msg = error_message(&body);
    assert!(msg.contains("columns"), "Error should mention columns: {}", msg);
}

#[tokio::test]
async fn lookup_rejects_mismatched_row_length() {
    let (status, body) = post_generate(r#"{
        "fields": [
            {"name": "loc", "type": "lookup", "options": {
                "columns": ["a", "b"],
                "data": [["x", "y", "z"]]
            }}
        ],
        "num_rows": 1,
        "format": "json"
    }"#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let msg = error_message(&body);
    assert!(msg.contains("3 values"), "Error should mention value count: {}", msg);
    assert!(msg.contains("expected 2"), "Error should mention expected count: {}", msg);
}
