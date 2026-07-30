#!/usr/bin/env bash
set -euo pipefail

slug="${1:-}"
if [[ -z "$slug" ]]; then
  printf '使用方法: make new-task SLUG=<kebab-case>\n' >&2
  exit 2
fi
if [[ ! "$slug" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  printf 'SLUG は小文字の kebab-case で指定してください: %s\n' "$slug" >&2
  exit 2
fi

target="docs/development/tasks/${slug}.md"
if [[ -e "$target" ]]; then
  printf 'タスク記録は既に存在します: %s\n' "$target" >&2
  exit 1
fi

cp docs/development/tasks/TEMPLATE.md "$target"
python3 - "$target" "$slug" <<'PY'
from datetime import date
from pathlib import Path
import sys

path = Path(sys.argv[1])
slug = sys.argv[2]
text = path.read_text(encoding="utf-8")
text = text.replace("<task-title>", slug)
text = text.replace("<task-slug>", slug)
text = text.replace("<yyyy-mm-dd>", date.today().isoformat())
path.write_text(text, encoding="utf-8")
PY

printf '作成しました: %s\n' "$target"
