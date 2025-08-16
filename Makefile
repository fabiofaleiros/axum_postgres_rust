run-local:
	cargo clean
	docker compose up postgres -d
	cargo run --release

run-docker:
	docker compose up -d

run-stop:
	docker-compose stop

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

coverage:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Html --output-dir coverage

coverage-json:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Json --output-dir coverage

coverage-report:
	@if not exist coverage mkdir coverage
	cargo tarpaulin --out Html --out Json --output-dir coverage
	