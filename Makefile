.PHONY: dev.server
dev.server:
	cargo watch -x 'run -p server --bin main'

.PHONY: db.up
db.up:
	docker-compose up -d

.PHONY: db.down
db.down:
	docker-compose down

.PHONY: db.attach
db.attach:
	docker-compose exec db /bin/bash -c "su - postgres"