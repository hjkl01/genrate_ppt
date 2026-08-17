.DEFAULT_GOAL := help

CARGO ?= cargo
PNPM ?= pnpm
PORT ?= 8080
FRONTEND_DIR ?= frontend
FRONTEND_PORT ?= 5173

.PHONY: help setup install build build-backend build-frontend check test test-backend test-frontend fmt fmt-check clippy lint run dev dev-backend dev-frontend api clean clean-frontend docker-build docker-up docker-down

help:
	@echo "Rust + React AI PPT engine"
	@echo "  make setup          Install backend and frontend dependencies"
	@echo "  make install        Alias for setup"
	@echo "  make build          Build backend and frontend"
	@echo "  make build-backend  Build Rust release binary"
	@echo "  make build-frontend Build React production bundle"
	@echo "  make check         Check Rust code"
	@echo "  make test          Run backend and frontend tests"
	@echo "  make fmt            Format Rust code"
	@echo "  make fmt-check      Check Rust formatting"
	@echo "  make clippy         Run Rust Clippy"
	@echo "  make lint           Run backend and frontend lint"
	@echo "  make run            Start backend"
	@echo "  make dev            Start backend and frontend together"
	@echo "  make dev-backend    Start backend only"
	@echo "  make dev-frontend   Start frontend only"
	@echo "  make api            Check backend health"
	@echo "  make clean          Remove build artifacts"

setup:
	$(CARGO) fetch
	cd $(FRONTEND_DIR) && $(PNPM) install

install: setup

build: build-backend build-frontend

build-backend:
	$(CARGO) build --release

build-frontend:
	cd $(FRONTEND_DIR) && $(PNPM) build

check:
	$(CARGO) check --all-targets --all-features

test: test-backend test-frontend

test-backend:
	$(CARGO) test --all-targets --all-features

test-frontend:
	cd $(FRONTEND_DIR) && $(PNPM) test

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

lint: clippy
	cd $(FRONTEND_DIR) && $(PNPM) lint

run:
	$(CARGO) run

dev:
	@set -m; \
	$(CARGO) run & backend_pid=$$!; \
	(cd $(FRONTEND_DIR) && $(PNPM) dev --host 0.0.0.0 --port $(FRONTEND_PORT)) & frontend_pid=$$!; \
	trap 'kill $$backend_pid $$frontend_pid 2>/dev/null || true' INT TERM EXIT; \
	wait $$backend_pid $$frontend_pid

dev-backend:
	RUST_LOG=$${RUST_LOG:-debug} RUST_BACKTRACE=$${RUST_BACKTRACE:-1} $(CARGO) run

dev-frontend:
	cd $(FRONTEND_DIR) && $(PNPM) dev --host 0.0.0.0 --port $(FRONTEND_PORT)

api:
	@curl -fsS http://127.0.0.1:$(PORT)/health
	@echo

clean: clean-frontend
	$(CARGO) clean

clean-frontend:
	rm -rf $(FRONTEND_DIR)/dist $(FRONTEND_DIR)/node_modules/.vite

docker-build:
	docker build -t genrate-ppt:latest .

docker-up:
	docker compose up -d

docker-down:
	docker compose down
