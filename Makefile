SHELL := /bin/bash
TAILWIND_BIN := ./tailwindcss
DAISY_PLUGIN := ./daisyui.mjs
DAISY_THEME_PLUGIN := ./daisyui-theme.mjs

.PHONY: help setup setup-css serve build fmt check clippy test clean prettier

help:
	@echo "Targets:"
	@echo "  make setup    - install rust target, trunk, and daisyUI css tooling"
	@echo "  make setup-css - download tailwind standalone and daisyUI plugin files"
	@echo "  make serve    - run local dev server with Trunk"
	@echo "  make build    - production build into dist/"
	@echo "  make fmt      - format Rust and web assets"
	@echo "  make check    - cargo check"
	@echo "  make clippy   - run clippy lints (warnings denied)"
	@echo "  make test     - run cargo tests"
	@echo "  make clean    - remove build outputs"
	@echo "  make prettier - format HTML/CSS only if prettier exists"

setup:
	rustup target add wasm32-unknown-unknown
	@if ! command -v trunk >/dev/null 2>&1; then cargo install trunk; fi
	@$(MAKE) setup-css

setup-css:
	@if [ ! -f "$(TAILWIND_BIN)" ]; then \
		ARCH=$$(uname -m); \
		if [ "$$ARCH" = "arm64" ] || [ "$$ARCH" = "aarch64" ]; then \
			URL="https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64"; \
		else \
			URL="https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-x64"; \
		fi; \
		curl -sSL "$$URL" -o "$(TAILWIND_BIN)"; \
		chmod +x "$(TAILWIND_BIN)"; \
	fi
	@if [ ! -f "$(DAISY_PLUGIN)" ]; then \
		curl -sSLO "https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs"; \
	fi
	@if [ ! -f "$(DAISY_THEME_PLUGIN)" ]; then \
		curl -sSLO "https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs"; \
	fi

serve: setup-css
	trunk serve

build: setup-css
	trunk build --release

fmt:
	cargo fmt
	$(MAKE) prettier

check:
	cargo check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

clean:
	cargo clean

prettier:
	@if command -v prettier >/dev/null 2>&1; then \
		prettier --write index.html input.css README.md; \
	else \
		echo "prettier not found; skipping HTML/CSS formatting."; \
	fi
