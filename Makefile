.DEFAULT_GOAL := help

CARGO ?= cargo
NPM ?= npm
PORT ?= 8080
FRONTEND_DIR ?= frontend
FRONTEND_PORT ?= 5173

.PHONY: help setup install build build-backend build-frontend check test test-backend test-frontend fmt fmt-check clippy lint run dev dev-backend dev-frontend api clean clean-frontend docker-build docker-up docker-down

help:
	@echo "Rust + React AI PPT engine"
	@echo ""
	@echo "  make setup          Install backend and frontend dependencies"
	@echo "  make install        Alias for setup"
	@echo "  make build         Build backend and frontend"
	@echo "  make build-backend Build Rust release binary"
	@echo "  make build-frontend Build React production bundle"
	@echo "  make check         Check Rust code"
	@echo "  make test          Run backend and frontend tests"
	@echo "  make test-backend  Run Rust tests"
	@echo "  make test-frontend Run frontend tests if configured"
	@echo "  make fmt            Format Rust code"
	@echo "  make fmt-check      Check Rust formatting"
	@echo "  make clippy         Run Rust Clippy"
	@echo "  make lint           Run backend and frontend lint checks"
	@echo "  make run            Start backend"
	@echo "  make dev            Start backend and frontend together"
	@echo "  make dev-backend    Start backend only"
	@echo "  make dev-frontend   Start frontend only"
	@echo "  make api            Check backend health"
	@echo "  make clean          Remove backend and frontend build artifacts"
	@echo "  make docker-build   Build Docker image"
	@echo "  make docker-up      Start Docker Compose services"
	@echo "  make docker-down    Stop Docker Compose services"

setup:
	$(CARGO) fetch
	cd $(FRONTEND_DIR) && $(NPM) install

install: setup

build: build-backend build-frontend

build-backend:
	$(CARGO) build --release

build-frontend:
	cd $(FRONTEND_DIR) && $(NPM) run build

check:
	$(CARGO) check --all-targets --all-features

test: test-backend test-frontend

test-backend:
	$(CARGO) test --all-targets --all-features

test-frontend:
	@cd $(FRONTEND_DIR) && if $(NPM) run | grep -qE '(^|[[:space:]])test([[:space:]]|$$)'; then $(NPM) test; else echo "No frontend test script configured; skipping."; fi

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

lint: clippy
	@cd $(FRONTEND_DIR) && if $(NPM) run | grep -qE '(^|[[:space:]])lint([[:space:]]|$$)'; then $(NPM) run lint; else echo "No frontend lint script configured; skipping."; fi

run:
	$(CARGO) run

dev: dev-backend dev-frontend

# Run both development servers. Requires a POSIX shell with background-job support.
dev-backend:
	@RUST_LOG=$${RUST_LOG:-debug} RUST_BACKTRACE=$${RUST_BACKTRACE:-1} $(CARGO) run & \
	backend_pid=$$!; \
	trap 'kill $$backend_pid 2>/dev/null || true' INT TERM EXIT; \
	cd $(FRONTEND_DIR) && $(NPM) run dev -- --host 0.0.0.0 --port $(FRONTEND_PORT)

dev-frontend:
	cd $(FRONTEND_DIR) && $(NPM) run dev -- --host 0.0.0.0 --port $(FRONTEND_PORT)

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
