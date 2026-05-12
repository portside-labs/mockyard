# Mockyard

A fast, self-hostable mock data generator written in Rust. An open-source alternative to [Mockaroo](https://mockaroo.com).

## Features

- **Fast** — built with Rust and Axum for high-throughput data generation
- **Self-hostable** — single binary, Docker-ready, runs on Cloud Run / AWS Lambda
- **Web UI** — reactive schema builder powered by Alpine.js with localStorage persistence
- **REST API** — versioned endpoint (`POST /v1/generate`) with full [OpenAPI 3.1 spec](/static/openapi.yaml)
- **Interactive API docs** — built-in docs page with try-it-out at `/docs`
- **CSV & JSON export** — download generated data in either format
- **35+ field types** — names, emails, addresses, phone numbers, IPs, UUIDs, dates, credit cards, and more
- **Fine-grained control** — null/blank percentages, boolean true/false distributions, weighted enum values, number ranges with negative support, decimal precision

## Quick Start

### Run locally

```bash
cargo run
# Open http://localhost:8080
```

### Docker

```bash
docker compose up

# or build and run manually
docker build -t mockyard .
docker run -p 8080:8080 mockyard
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT`   | `8080`  | Server listen port |

## API

Full interactive documentation is available at `/docs` when the server is running. The OpenAPI spec is at `/static/openapi.yaml`.

### `POST /v1/generate`

Generate mock data from a schema definition.

```bash
curl -X POST http://localhost:8080/v1/generate \
  -H "Content-Type: application/json" \
  -d '{
    "fields": [
      { "name": "id", "type": "row_number", "options": {} },
      { "name": "name", "type": "full_name", "options": {} },
      { "name": "email", "type": "email", "options": { "blank_percentage": 5 } },
      { "name": "role", "type": "enum", "options": {
          "values": [
            { "value": "Admin", "weight": 5 },
            { "value": "Manager", "weight": 25 },
            { "value": "Viewer" }
          ]
      }},
      { "name": "active", "type": "boolean", "options": { "true_percentage": 80 } },
      { "name": "score", "type": "integer", "options": { "min": -100, "max": 100 } }
    ],
    "num_rows": 1000,
    "format": "json"
  }'
```

### Field Types

| Type | Description | Options |
|------|-------------|---------|
| `row_number` | Sequential row number | — |
| `first_name` | First name | — |
| `last_name` | Last name | — |
| `full_name` | Full name | — |
| `email` | Email address | — |
| `username` | Username | — |
| `phone` | Phone number | — |
| `city` | City name | — |
| `state` | US state | — |
| `country` | Country name | — |
| `zip_code` | Zip code | — |
| `street_address` | Street address | — |
| `latitude` | Latitude coordinate | — |
| `longitude` | Longitude coordinate | — |
| `company_name` | Company name | — |
| `job_title` | Job title | — |
| `credit_card` | Credit card number | — |
| `domain_name` | Domain name | — |
| `ipv4` | IPv4 address | — |
| `ipv6` | IPv6 address | — |
| `mac_address` | MAC address | — |
| `user_agent` | Browser user agent | — |
| `uuid` | UUID v4 | — |
| `color` | Color name | — |
| `hex_color` | Hex color code | — |
| `date` | Date (YYYY-MM-DD) | — |
| `date_time` | DateTime (ISO 8601) | — |
| `time` | Time (HH:MM:SS) | — |
| `paragraph` | Lorem ipsum paragraph | — |
| `sentence` | Lorem ipsum sentence | — |
| `word` | Random word | — |
| `integer` | Integer | `min`, `max` |
| `decimal` | Decimal number | `min`, `max`, `decimals` |
| `boolean` | Boolean | `true_percentage` |
| `enum` | Enum from fixed values | `values` (with optional `weight`) |
| `percentage` | Percentage | `min`, `max`, `decimals` |
| `currency` | Currency amount | `min`, `max`, `decimals` |

### Field Options (all types)

| Option | Type | Description |
|--------|------|-------------|
| `blank_percentage` | `number` | Percentage of values that should be null/blank (0–100) |

## Deployment

### Cloud Run

```bash
gcloud run deploy mockyard \
  --source . \
  --port 8080 \
  --allow-unauthenticated
```

### AWS Lambda

Use [cargo-lambda](https://www.cargo-lambda.info/) or deploy the Docker image with Lambda container image support.

## Tech Stack

- **Rust** with [Axum](https://github.com/tokio-rs/axum) for the HTTP server
- **[fake](https://crates.io/crates/fake)** crate for data generation
- **[Alpine.js](https://alpinejs.dev/)** for the reactive frontend
- **[rust-embed](https://crates.io/crates/rust-embed)** to bundle static assets into the binary

## License

MIT
