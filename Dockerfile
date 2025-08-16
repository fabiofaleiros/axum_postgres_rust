FROM rust:1.88.0-bookworm as builder

WORKDIR /app

# Copy over the manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy file and copy the real source code
RUN rm -rf src
COPY src ./src
COPY tests ./tests
COPY .sqlx ./.sqlx
COPY .env ./.env

# Build the application with cached dependencies
RUN cargo build --release

# Test stage - runs all tests
FROM builder as test

# Install additional test dependencies if needed
RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for testing
ENV DATABASE_URL=postgres://test_user:test_pass@localhost:5432/test_db
ENV RUST_LOG=debug

# Run all tests
RUN cargo test --verbose

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/axum_postgres_rust .
COPY --from=builder /app/.env ./.env

EXPOSE 7878

CMD ["./axum_postgres_rust"]
