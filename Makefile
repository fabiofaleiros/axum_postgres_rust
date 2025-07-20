run:
	cargo clean
	docker compose up postgres -d
	cargo run --release

docker:
	docker compose up -d

stop:
	docker-compose stop
	