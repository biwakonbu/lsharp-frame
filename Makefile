SHELL := /usr/bin/env bash
.DEFAULT_GOAL := help

.PHONY: help doctor context fmt fmt-check check lint test wit-check harness ci run-headless new-task

help: ## 利用可能なコマンドを表示
	@awk 'BEGIN {FS = ":.*## "; printf "L#frame 開発コマンド\n\n"} /^[a-zA-Z0-9_-]+:.*## / {printf "  %-16s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

doctor: ## ローカル開発ツールの利用可否を表示
	@bash scripts/harness/doctor.sh

context: ## 機密情報を含まないリポジトリコンテキストを表示
	@bash scripts/harness/context.sh

fmt: ## Rust ソースを整形
	@cargo fmt --all

fmt-check: ## Rust の整形状態を検証
	@cargo fmt --all -- --check

check: ## Rust workspace の全 target を型検査
	@cargo check --workspace --all-targets

lint: ## warning を拒否して Clippy を実行
	@cargo clippy --workspace --all-targets -- -D warnings

test: ## Rust workspace のテストを実行
	@cargo test --workspace

wit-check: ## wasm-tools がある場合に WIT package を検証
	@if command -v wasm-tools >/dev/null 2>&1; then \
		wasm-tools component wit wit/frame.wit --json >/dev/null; \
	else \
		echo 'wasm-tools が利用できないため、WIT 検証を実行していません' >&2; \
		exit 2; \
	fi

harness: ## エージェント設定とリポジトリ不変条件を検証
	@python3 scripts/harness/validate.py
	@find scripts -type f -name '*.sh' -print0 | xargs -0 -n1 bash -n
	@git diff --check

ci: harness fmt-check check lint test ## ローカルの完全検証を実行

run-headless: ## 決定的な headless composition root を実行
	@cargo run -p lsharp-frame-headless

new-task: ## タスク記録を作成: make new-task SLUG=<kebab-case>
	@bash scripts/harness/new-task.sh "$(SLUG)"
