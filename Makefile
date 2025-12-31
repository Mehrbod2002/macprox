SHELL := /bin/bash

APP_NAME := macprox

.PHONY: help deps deps-src check-sshuttle run build release fmt clippy test clean

help:
	@echo "Targets:"
	@echo "  make deps        - Install required tools (sshuttle) using brew or pip (no sudo)"
	@echo "  make deps-src    - Show advanced source install (make install) instructions"
	@echo "  make run         - Run in development"
	@echo "  make build       - Debug build"
	@echo "  make release     - Release build"
	@echo "  make fmt         - Rust format check"
	@echo "  make clippy      - Rust clippy (deny warnings)"
	@echo "  make test        - Run tests"
	@echo "  make clean       - Clean target dir"

check-sshuttle:
	@command -v sshuttle >/dev/null 2>&1 || ( \
		echo "sshuttle is NOT installed."; \
		exit 1; \
	)
	@echo "sshuttle is installed: $$(command -v sshuttle)"

deps:
	@echo "Checking sshuttle..."
	@if command -v sshuttle >/dev/null 2>&1; then \
		echo "✅ sshuttle already installed: $$(command -v sshuttle)"; \
		exit 0; \
	fi
	@echo "sshuttle not found. Installing..."
	@if command -v brew >/dev/null 2>&1; then \
		echo "Using Homebrew..."; \
		brew install sshuttle; \
	elif command -v pip3 >/dev/null 2>&1; then \
		echo "Using pip3 (user install)..."; \
		pip3 install --user sshuttle; \
		echo ""; \
		echo "NOTE: If your PATH doesn't include Python user bin, you may need:"; \
		echo "  export PATH=\"$$HOME/Library/Python/3.x/bin:$$PATH\""; \
	else \
		echo "❌ No brew or pip3 found."; \
		echo "Install Homebrew OR Python3/pip3, then run: make deps"; \
		echo "Or use: make deps-src (advanced)"; \
		exit 1; \
	fi
	@echo "Done. Verifying..."
	@command -v sshuttle >/dev/null 2>&1 && echo "✅ sshuttle ready" || (echo "❌ sshuttle still not found in PATH"; exit 1)

deps-src:
	@echo "Advanced (from source) install commands:"
	@echo "  git clone https://github.com/sshuttle/sshuttle.git"
	@echo "  cd sshuttle"
	@echo "  sudo make install"
	@echo ""
	@echo "We do NOT run this automatically because it uses sudo."

run:
	@cargo run

build:
	@cargo build

release:
	@cargo build --release

fmt:
	@cargo fmt --all -- --check

clippy:
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	@cargo test --all

clean:
	@cargo clean
