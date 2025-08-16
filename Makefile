# Makefile for Axum Postgres Rust API

.PHONY: help run-local run-docker run-stop test-local test-unit test-integration test-domain test-docker test-all coverage coverage-json coverage-report clean build

# Default target
help:
	@echo "Available commands:"
	@echo "  run-local      - Run application locally with PostgreSQL in Docker"
	@echo "  run-docker     - Run everything in Docker"
	@echo "  run-stop       - Stop all services"
	@echo "  test-local     - Run all tests locally"
	@echo "  test-unit      - Run unit tests only"
	@echo "  test-integration - Run integration tests with Docker database"
	@echo "  test-domain    - Run domain tests only"
	@echo "  test-docker    - Run all tests in Docker container"
	@echo "  test-all       - Run comprehensive test suite"
	@echo "  build          - Build the application"
	@echo "  clean          - Clean build artifacts"
	@echo "  coverage       - Generate HTML coverage report"
	@echo "  coverage-json  - Generate JSON coverage report"
	@echo "  coverage-report - Generate both HTML and JSON coverage reports"

# Build commands
build:
	cargo build

build-release:
	cargo build --release

clean:
	cargo clean

# Run commands
run-local:
	cargo clean
	docker compose up postgres -d
	cargo run --release

run-docker:
	docker compose up -d

run-stop:
	docker-compose stop

# Test commands
test-local:
	cargo test

test-unit:
	cargo test unit

test-integration:
	docker compose up postgres -d
	cargo test integration
	docker-compose stop

test-domain:
	cargo test domain

test-docker:
	@echo "Building and running all tests in Docker container..."
	docker build --target test -t axum-postgres-test .
	@echo "All tests completed successfully in Docker!"

test-all: test-local test-integration
	@echo "All test suites completed successfully!"

# Coverage commands
coverage:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Html --output-dir coverage

coverage-json:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Json --output-dir coverage

coverage-report:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Html --out Json --output-dir coverage
	