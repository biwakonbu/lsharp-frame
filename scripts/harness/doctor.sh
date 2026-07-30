#!/usr/bin/env bash
set -uo pipefail

required=(git make python3 cargo rustc rustfmt)
optional=(wasm-tools lsharp codex claude cursor gh)
missing_required=0

print_tool() {
  local class="$1"
  local tool="$2"
  if command -v "$tool" >/dev/null 2>&1; then
    local version
    version="$($tool --version 2>/dev/null | head -n 1 || true)"
    printf '  %-8s %-12s %s\n' "$class" "$tool" "${version:-利用可能}"
  else
    printf '  %-8s %-12s %s\n' "$class" "$tool" '不足'
    if [[ "$class" == 必須 ]]; then
      missing_required=1
    fi
  fi
}

printf 'L#frame 開発環境\n\n'
for tool in "${required[@]}"; do
  print_tool 必須 "$tool"
done
for tool in "${optional[@]}"; do
  print_tool 任意 "$tool"
done

printf '\n固定 Rust toolchain: '
python3 - <<'PY'
from pathlib import Path
import tomllib
with Path("rust-toolchain.toml").open("rb") as stream:
    print(tomllib.load(stream)["toolchain"]["channel"])
PY

if command -v rustup >/dev/null 2>&1; then
  printf '有効な Rust toolchain: '
  rustup show active-toolchain 2>/dev/null || printf '不明\n'
fi

if (( missing_required )); then
  printf '\n必須ツールが1つ以上不足しています。`make ci` は完走しません。\n'
else
  printf '\n必須のローカルツールを利用できます。\n'
fi

# 診断専用。最初の不足ツールで停止せず、環境全体を表示する。
exit 0
