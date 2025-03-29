.PHONY: help build up down logs test clean

help:
	@echo "Available targets:"
	@echo "  build      - Build the Docker containers"
	@echo "  up         - Start the containers in detached mode"
	@echo "  down       - Stop and remove the containers"
	@echo "  logs       - View container logs"
	@echo "  test       - Run tests"
	@echo "  clean      - Remove all containers and clean up"
	@echo "  prod       - Build and run in production mode"
	@echo "  dev        - Build and run in development mode (default)"

build:
	docker compose build

up:
	docker compose up -d

down:
	docker compose down

logs:
	docker compose logs -f

# Run all tests in running container
test:
	docker compose exec backend cargo test -- --nocapture

# Run specific test file (e.g., concurrency.rs)
test-file:
	docker compose exec backend cargo test --test $(FILE) -- --nocapture

# Example: make test-file FILE=concurrency

clean: down
	docker compose rm -f
	docker volume prune -f

prod:
	ENV=prod docker compose up --build -d

dev:
	ENV=dev docker compose up --build -d

test-server:
	@echo "Testing server health..."
	@if curl -s -f http://localhost:3000/health >/dev/null; then \
		echo "Server is healthy!"; \
		curl -s http://localhost:3000/health | jq . 2>/dev/null || echo "Got response:" && curl -s http://localhost:3000/health; \
	else \
		echo "Server not responding!"; \
		exit 1; \
	fi