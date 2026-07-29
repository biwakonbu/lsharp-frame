#!/usr/bin/env bash
set -euo pipefail

slug="${1:-}"
if [[ -z "$slug" ]]; then
  printf 'usage: make new-task SLUG=<kebab-case>\n' >&2
  exit 2
fi
if [[ ! "$slug" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  printf 'SLUG must be lowercase kebab-case: %s\n' "$slug" >&2
  exit 2
fi

target="docs/development/tasks/${slug}.md"
if [[ -e "$target" ]]; then
  printf 'task record already exists: %s\n' "$target" >&2
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
text = text.replace("<task-title>", slug.replace("-", " ").title())
text = text.replace("<task-slug>", slug)
text = text.replace("<yyyy-mm-dd>", date.today().isoformat())
path.write_text(text, encoding="utf-8")
PY

printf 'created %s\n' "$target"
