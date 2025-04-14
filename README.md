# Rust Fullstack App

A fullstack web application with Rust backend (Axum) and Docker deployment.

## Features

- 🚀 Axum web server
- 📦 Dockerized development and production environments
- 📊 Request logging middleware
- 🧪 Integration and concurrency tests
- 🔄 Hot-reloading in development

## Prerequisites

- Rust 1.75+
- Docker 20.10+
- Make (optional)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-app-fullstack.git
cd rust-app-fullstack

# Start in development mode
make dev

# Or for production:
make prod
```

## Project Structure

```
rust-app-fullstack/
├── backend/       # Axum web server
├── docs/          # Documentation
├── Makefile       # Development commands
└── docker-compose.yml
```

## Development

```bash
# Build and start containers
make dev

# Run tests
make test

# View logs
make logs
```

## Deployment

```bash
# Production build
make prod

# Push to Docker registry
docker