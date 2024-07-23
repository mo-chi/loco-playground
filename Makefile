init: ## Run initial setting
	cargo install loco-cli sea-orm-cli cargo-watch
	bash tools/create_env.sh development

up: ## Run docker compose up
	docker compose up -d

down: ## Run docker compose down
	docker compose down

run: ## Run loco start
	cargo watch -x check -s "cargo loco start"

ping: ## Run ping check
	curl http://localhost:5150/api/_ping

health: ## Run health check
	curl http://localhost:5150/api/_health

psql: ## postgres login
	docker compose exec postgres psql -U loco -d loco_playground_development

test: ## Run test
	bash tools/create_env.sh test
	docker compose --env-file .env.test up -d

	LOCO_ENV=test cargo loco db migrate
	LOCO_ENV=test cargo loco task seed_data

	INSTA_UPDATE=no cargo test

	docker compose down

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-10s\033[0m %s\n", $$1, $$2}'

.PHONY: help init up down ping health psql test
.DEFAULT_GOAL := help
