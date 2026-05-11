# Mockyard

A fast, self-hostable mock data generator written in Rust. An open-source alternative to Mockaroo.

## Features

- **Fast** — built with Rust and Axum for high-throughput data generation
- **Self-hostable** — single binary, Docker-ready, serverless-compatible
- **Web UI** — clean interface for building schemas and generating data
- **REST API** — versioned API for programmatic access
- **Multiple formats** — export to CSV or JSON
- **Rich field types** — 35+ data types including names, emails, addresses, numbers, booleans, enums, dates, and more
- **Fine-grained control** — null/blank percentages, boolean distributions, enum weight distributions, number ranges

## Quick Start

### Run locally

```bash
cargo run
# Open http://localhost:8080
```

### Docker

```bash
docker compose up
# or
docker build -t mockyard .
docker run -p 8080:8080 mockyard
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT`   | `8080`  | Server port |

## API

### `POST /v1/generate`

Generate mock data from a schema definition.

**Request body:**

```json
{
  "fields": [
    {
      "name": "id",
      "type": "row_number",
      "options": {}
    },
    {
      "name": "first_name",
      "type": "first_name",
      "options": {
        "blank_percentage": 5
      }
    },
    {
      "name": "role",
      "type": "enum",
      "options": {
        "values": [
          { "value": "Admin", "weight": 5 },
          { "value": "Manager", "weight": 25 },
          { "value": "Viewer" }
        ]
      }
    },
    {
      "name": "active",
      "type": "boolean",
      "options": {
        "true_percentage": 80
      }
    },
    {
      "name": "score",
      "type": "integer",
      "options": {
        "min": -100,
        "max": 100
      }
    }
  ],
  "num_rows": 1000,
  "format": "csv"
}
```

**Response:** CSV or JSON data depending on the `format` field.

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

### Options (all types)

| Option | Type | Description |
|--------|------|-------------|
| `blank_percentage` | `number` | Percentage of values that should be null (0–100) |

## Deployment

### Cloud Run

```bash
gcloud run deploy mockyard \
  --source . \
  --port 8080 \
  --allow-unauthenticated
```

### AWS Lambda

Use [cargo-lambda](https://www.cargo-lambda.info/) or build the Docker image and deploy to Lambda with container image support.

## License

MIT
