FROM rust:1.88.0 as builder

WORKDIR /app

# Copy over the manifest files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy file and copy the real source code
RUN rm -rf src
COPY src ./src
COPY .sqlx ./.sqlx

# Build the application with cached dependencies
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/axum_postgres_rust .
COPY .env ./.env

ENV DATABASE_URL=postgres://root:1234@postgres:5432/axum_postgres
ENV SERVER_ADDRESS=0.0.0.0:7878

EXPOSE 7878

CMD ["./axum_postgres_rust"]
