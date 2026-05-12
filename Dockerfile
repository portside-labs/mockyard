FROM rust:1.86-slim AS builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs && cargo build --release && rm -rf src

# Build the actual app
COPY src ./src
COPY static ./static
RUN touch src/main.rs src/lib.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mockyard /usr/local/bin/mockyard

ENV PORT=8080
EXPOSE 8080

CMD ["mockyard"]
