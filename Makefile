.DEFAULT_GOAL := help

CARGO ?= cargo
BINARY ?= genrate-ppt
PORT ?= 8080

.PHONY: help build check test fmt fmt-check clippy run dev clean api

help:
	@echo "Rust AI PPT engine"
	@echo ""
	@echo "  make build       Build release binary"
	@echo "  make check       Fast compile check"
	@echo "  make test        Run tests"
	@echo "  make fmt         Format Rust code"
	@echo "  make fmt-check   Check formatting"
	@echo "  make clippy      Run Clippy"
	@echo "  make run         Start API server"
	@echo "  make dev         Start with debug logging"
	@echo "  make api         Check health endpoint"
	@echo "  make clean       Remove build artifacts"

build:
	$(CARGO) build --release

check:
	$(CARGO) check --all-targets --all-features

test:
	$(CARGO) test --all-targets --all-features

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

run:
	$(CARGO) run

dev:
	RUST_LOG=$${RUST_LOG:-debug} RUST_BACKTRACE=$${RUST_BACKTRACE:-1} $(CARGO) run

api:
	@curl -fsS http://127.0.0.1:$(PORT)/health
	@echo

clean:
	$(CARGO) clean
