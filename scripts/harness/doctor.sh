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
    printf '  %-8s %-12s %s\n' "$class" "$tool" "${version:-available}"
  else
    printf '  %-8s %-12s %s\n' "$class" "$tool" 'missing'
    if [[ "$class" == required ]]; then
      missing_required=1
    fi
  fi
}

printf 'L#frame development environment\n\n'
for tool in "${required[@]}"; do
  print_tool required "$tool"
done
for tool in "${optional[@]}"; do
  print_tool optional "$tool"
done

printf '\nPinned Rust toolchain: '
python3 - <<'PY'
from pathlib import Path
import tomllib
with Path("rust-toolchain.toml").open("rb") as stream:
    print(tomllib.load(stream)["toolchain"]["channel"])
PY

if command -v rustup >/dev/null 2>&1; then
  printf 'Active Rust toolchain: '
  rustup show active-toolchain 2>/dev/null || printf 'unknown\n'
fi

if (( missing_required )); then
  printf '\nOne or more required tools are unavailable. `make ci` will not complete.\n'
else
  printf '\nRequired local tools are available.\n'
fi

# Diagnostic only: report the entire environment rather than failing at the first missing tool.
exit 0
