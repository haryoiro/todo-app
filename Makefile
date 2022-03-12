.PHONY: dev.api
dev.server:
	cargo watch -x 'run -p server --bin main'

.PHONY: dev.fix
dev.fix:
	cargo fmt -p server

.PHONY: cli.run
run.cli:
	cargo run -p server --bin cli



.PHONY: db.up
db.up:
	docker-compose up -d

.PHONY: db.down
db.down:
	docker-compose down

.PHONY: db.attach
db.attach:
	docker-compose exec db /bin/bash -c "su - postgres"

