SHELL := /bin/sh

RUSTUP := $(shell command -v rustup 2>/dev/null)
CARGO := $(shell if command -v rustup >/dev/null 2>&1; then rustup which cargo; else command -v cargo; fi)
RUSTC := $(shell if command -v rustup >/dev/null 2>&1; then rustup which rustc; else command -v rustc; fi)
TRUNK := $(shell command -v trunk 2>/dev/null)

export PATH := $(HOME)/.cargo/bin:$(PATH)
export RUSTC := $(RUSTC)

.PHONY: all assets build check clippy doctor e2e ensure-toolchain fmt serve

all: check

ensure-toolchain:
	@if [ -z "$(RUSTUP)" ]; then \
		echo "rustup is required for this project. Install it from https://rustup.rs/"; \
		exit 1; \
	fi
	@rustup target list --installed | grep -q '^wasm32-unknown-unknown$$' || rustup target add wasm32-unknown-unknown
	@rustup component list --installed | grep -q '^clippy-' || rustup component add clippy
	@rustup component list --installed | grep -q '^rustfmt-' || rustup component add rustfmt
	@if [ -z "$(TRUNK)" ]; then \
		echo "trunk is required. Install it with: cargo install trunk"; \
		exit 1; \
	fi

doctor: ensure-toolchain
	@echo "cargo: $(CARGO)"
	@echo "rustc: $(RUSTC)"
	@echo "trunk: $(TRUNK)"
	@$(CARGO) --version
	@$(RUSTC) --version
	@$(TRUNK) --version
	@rustup target list --installed | grep -q '^wasm32-unknown-unknown$$'
	@echo "toolchain ok"

fmt: ensure-toolchain
	@$(CARGO) fmt --all -- --check

clippy: ensure-toolchain
	@$(CARGO) clippy --target wasm32-unknown-unknown -- -D warnings -D clippy::pedantic

build: ensure-toolchain
	@env -u NO_COLOR RUSTC="$(RUSTC)" $(TRUNK) build --release --public-url /opt-out/

serve: ensure-toolchain
	@env -u NO_COLOR RUSTC="$(RUSTC)" $(TRUNK) serve --port 8080 --public-url /opt-out/

e2e: ensure-toolchain
	@RUSTC="$(RUSTC)" $(CARGO) test --test e2e_menu

assets:
	@node scripts/generate_seo_assets.mjs

check: fmt clippy build
