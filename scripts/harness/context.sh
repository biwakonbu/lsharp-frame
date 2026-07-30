#!/usr/bin/env bash
set -euo pipefail

printf '# L#frame リポジトリコンテキスト\n\n'
printf '## Git\n'
printf 'ブランチ: %s\n' "$(git branch --show-current 2>/dev/null || printf detached)"
printf 'HEAD: %s\n' "$(git rev-parse --short HEAD 2>/dev/null || printf none)"
printf '状態:\n'
git status --short || true

printf '\n## Workspace member\n'
python3 - <<'PY'
from pathlib import Path
import tomllib
with Path("Cargo.toml").open("rb") as stream:
    cargo = tomllib.load(stream)
for member in cargo.get("workspace", {}).get("members", []):
    print(f"- {member}")
PY

printf '\n## 正本ドキュメント\n'
for path in \
  AGENTS.md \
  docs/architecture.md \
  docs/backend-abstraction.md \
  docs/performance.md \
  docs/development/agent-harness.md \
  docs/development/definition-of-done.md; do
  [[ -f "$path" ]] && printf -- '- %s\n' "$path"
done

printf '\n## 進行中のタスク記録\n'
find docs/development/tasks -maxdepth 1 -type f -name '*.md' \
  ! -name README.md ! -name TEMPLATE.md -print | sort | sed 's#^#- #' || true
