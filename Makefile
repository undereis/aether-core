SHELL := /bin/zsh

UV ?= uv
PNPM ?= pnpm
PYTHON ?= python3.12
COMPOSE ?= docker compose -f compose.yaml
BACKEND_HOST ?= 127.0.0.1
BACKEND_PORT ?= 18000
export UV_CACHE_DIR ?= $(CURDIR)/.uv-cache
export PRE_COMMIT_HOME ?= $(CURDIR)/.pre-commit-cache

.PHONY: help setup lint format type-check test build docker-up docker-down docker-validate backend-dev backend-smoke backend-validate validate

help:
	@printf "Aether engineering commands\n"
	@printf "  make setup             Install local development dependencies\n"
	@printf "  make lint              Run Python and Rust linters\n"
	@printf "  make format            Format Python and Rust code\n"
	@printf "  make type-check        Run Python static typing checks\n"
	@printf "  make test              Run Python and Rust tests\n"
	@printf "  make build             Build/check Phase 0 artifacts\n"
	@printf "  make docker-up         Start infrastructure containers\n"
	@printf "  make docker-validate   Validate PostgreSQL, Redis, and Qdrant\n"
	@printf "  make backend-dev       Run the FastAPI backend locally\n"
	@printf "  make backend-smoke     Validate backend endpoints without binding a port\n"
	@printf "  make backend-validate  Validate backend health/version endpoints\n"
	@printf "  make validate          Run the full local validation suite\n"

setup:
	$(UV) sync --all-packages --all-groups --python $(PYTHON)
	$(PNPM) install
	$(UV) run pre-commit install

lint:
	$(UV) run ruff check backend scripts
	$(UV) run black --check backend scripts
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

format:
	$(UV) run ruff check backend scripts --fix
	$(UV) run ruff format backend scripts
	$(UV) run black backend scripts
	cargo fmt --all

type-check:
	$(UV) run mypy backend/src backend/tests scripts

test:
	$(UV) run pytest
	cargo test --workspace

build:
	cargo check --workspace
	$(UV) build backend --out-dir dist
	$(PNPM) exec tauri --version

docker-up:
	$(COMPOSE) up -d postgres redis qdrant

docker-down:
	$(COMPOSE) down

docker-validate:
	$(COMPOSE) ps
	$(COMPOSE) exec -T postgres pg_isready -U $${POSTGRES_USER:-aether} -d $${POSTGRES_DB:-aether}
	$(COMPOSE) exec -T redis redis-cli ping
	curl -fsS http://localhost:$${QDRANT_HTTP_PORT:-6333}/healthz

backend-dev:
	./scripts/run_backend.sh

backend-smoke:
	$(UV) run python scripts/smoke_backend.py

backend-validate:
	curl -fsS http://$(BACKEND_HOST):$(BACKEND_PORT)/health
	curl -fsS http://$(BACKEND_HOST):$(BACKEND_PORT)/version

validate: lint type-check test build docker-validate backend-smoke
