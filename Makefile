SHELL := /usr/bin/env bash
.DEFAULT_GOAL := help

.PHONY: help doctor context fmt fmt-check check lint test wit-check harness ci run-headless new-task

help: ## Show supported commands
	@awk 'BEGIN {FS = ":.*## "; printf "L#frame development commands\n\n"} /^[a-zA-Z0-9_-]+:.*## / {printf "  %-16s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

doctor: ## Report local development-tool availability
	@bash scripts/harness/doctor.sh

context: ## Print a compact, non-secret repository context snapshot
	@bash scripts/harness/context.sh

fmt: ## Format Rust sources
	@cargo fmt --all

fmt-check: ## Verify Rust formatting
	@cargo fmt --all -- --check

check: ## Type-check all Rust workspace targets
	@cargo check --workspace --all-targets

lint: ## Run Clippy with warnings denied
	@cargo clippy --workspace --all-targets -- -D warnings

test: ## Run Rust workspace tests
	@cargo test --workspace

wit-check: ## Validate the WIT package when wasm-tools is installed
	@if command -v wasm-tools >/dev/null 2>&1; then \
		wasm-tools component wit wit/frame.wit --json >/dev/null; \
	else \
		echo 'wasm-tools is unavailable; WIT validation was not run' >&2; \
		exit 2; \
	fi

harness: ## Validate agent configuration and repository invariants
	@python3 scripts/harness/validate.py
	@find scripts -type f -name '*.sh' -print0 | xargs -0 -n1 bash -n
	@git diff --check

ci: harness fmt-check check lint test ## Run the full local validation gate

run-headless: ## Run the deterministic headless composition root
	@cargo run -p lsharp-frame-headless

new-task: ## Create a task record: make new-task SLUG=<kebab-case>
	@bash scripts/harness/new-task.sh "$(SLUG)"
