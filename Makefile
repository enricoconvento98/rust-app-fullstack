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
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

logs:
	docker-compose logs -f

test:
	cd backend && cargo test

clean: down
	docker-compose rm -f
	docker volume prune -f

prod:
	ENV=prod docker-compose up --build -d

dev:
	ENV=dev docker-compose up --build -d