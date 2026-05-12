# Mockyard

A fast, self-hostable mock data generator. Open-source alternative to [Mockaroo](https://mockaroo.com).

Built with Rust for speed and low memory usage. Includes a web UI and a REST API. Generates CSV or JSON.

## Run it

```bash
docker build -t mockyard .
docker run -p 8080:8080 mockyard
```

Open [http://localhost:8080](http://localhost:8080) to use the UI, or [http://localhost:8080/docs](http://localhost:8080/docs) for API docs.

## API

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

Full interactive API docs with a try-it-out panel are available at `/docs`. The [OpenAPI 3.1 spec](static/openapi.yaml) is included.

## Field Types

35+ types across these categories:

| Category | Types |
|----------|-------|
| Person | `first_name`, `last_name`, `full_name`, `email`, `username`, `phone` |
| Address | `city`, `state`, `country`, `zip_code`, `street_address`, `latitude`, `longitude` |
| Business | `company_name`, `job_title`, `credit_card` |
| Internet | `domain_name`, `ipv4`, `ipv6`, `mac_address`, `user_agent` |
| Date & Time | `date`, `date_time`, `time` |
| Text | `paragraph`, `sentence`, `word` |
| Number | `integer`, `decimal`, `percentage`, `currency` |
| Basic | `row_number`, `uuid`, `color`, `hex_color`, `boolean`, `enum` |

## Field Options

Every field supports `blank_percentage` (0-100) to control how often a value is null.

Type-specific options:

| Option | Applies to | Description |
|--------|-----------|-------------|
| `min`, `max` | `integer`, `decimal`, `percentage`, `currency` | Number range (supports negatives) |
| `decimals` | `decimal`, `currency` | Decimal places (default: 2) |
| `true_percentage` | `boolean` | % of values that are `true` (default: 50) |
| `values` | `enum` | List of `{ "value": "...", "weight": N }` entries. Weights are percentages. Unweighted values split the remainder equally. |

## Contributing

```bash
cargo run            # start dev server on :8080
cargo test           # run tests
```

## License

MIT
