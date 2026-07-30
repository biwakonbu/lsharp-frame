#!/usr/bin/env python3
"""エージェントハーネスと L#frame の安定境界を検証する。"""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[2]
ERRORS: list[str] = []

SKILL_NAMES = (
    "lsharp-frame-plan",
    "lsharp-frame-implement",
    "lsharp-frame-verify",
    "lsharp-frame-review",
    "lsharp-frame-handoff",
)
REQUIRED_FILES = (
    "AGENTS.md",
    "CLAUDE.md",
    ".codex/config.toml",
    ".claude/settings.json",
    ".cursor/rules/00-project.mdc",
    ".cursor/commands/plan.md",
    "Makefile",
    "scripts/harness/doctor.sh",
    "scripts/harness/context.sh",
    "scripts/harness/new-task.sh",
    "scripts/harness/validate.py",
    "docs/development/agent-harness.md",
    "docs/development/definition-of-done.md",
    "docs/development/tasks/TEMPLATE.md",
    "docs/development/handoffs/TEMPLATE.md",
)
SCOPED_AREAS = ("crates", "apps", "kernel", "plugins", "wit", "docs")
FORBIDDEN_CONTRACT_PATTERNS = (
    "iced::",
    "wgpu::",
    "winit::",
    "gpui::",
    "tokio::",
    "portable_pty::",
)
FORBIDDEN_CONTRACT_DEPENDENCIES = {
    "iced",
    "wgpu",
    "winit",
    "gpui",
    "tokio",
    "portable-pty",
    "portable_pty",
}
MARKDOWN_LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")


def error(message: str) -> None:
    ERRORS.append(message)


def load_toml(relative: str) -> dict:
    path = ROOT / relative
    try:
        with path.open("rb") as stream:
            return tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        error(f"TOML が不正です {relative}: {exc}")
        return {}


def load_json(relative: str) -> dict:
    path = ROOT / relative
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        error(f"JSON が不正です {relative}: {exc}")
        return {}


def iter_text_files(base: Path, suffixes: Iterable[str]) -> Iterable[Path]:
    if not base.exists():
        return []
    return (
        path
        for path in base.rglob("*")
        if path.is_file() and path.suffix.lower() in suffixes
    )


def validate_required_files() -> None:
    for relative in REQUIRED_FILES:
        if not (ROOT / relative).is_file():
            error(f"必須ハーネスファイルがありません: {relative}")

    for name in SKILL_NAMES:
        for root in (".agents/skills", ".claude/skills"):
            relative = f"{root}/{name}/SKILL.md"
            if not (ROOT / relative).is_file():
                error(f"必須ワークフロー Skill がありません: {relative}")


def validate_instruction_hierarchy() -> None:
    root_agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    if "リポジトリ全体の指示の正本" not in root_agents:
        error("AGENTS.md は自身をリポジトリ全体の指示の正本として明記してください")
    if "## 出力言語" not in root_agents or "日本語" not in root_agents:
        error("AGENTS.md に AI の日本語出力規則がありません")

    root_claude = (ROOT / "CLAUDE.md").read_text(encoding="utf-8")
    if "@AGENTS.md" not in root_claude:
        error("CLAUDE.md は @AGENTS.md を import してください")
    if "日本語" not in root_claude:
        error("CLAUDE.md に日本語出力の指示がありません")
    if len(root_claude.splitlines()) > 12:
        error("CLAUDE.md が薄い adapter ではなくなっています。方針は AGENTS.md に置いてください")

    cursor_project = (ROOT / ".cursor/rules/00-project.mdc").read_text(encoding="utf-8")
    if "alwaysApply: true" not in cursor_project or "@AGENTS.md" not in cursor_project:
        error("Cursor の project rule は常時適用し、@AGENTS.md を参照してください")
    if "日本語" not in cursor_project:
        error("Cursor の project rule に日本語出力の指示がありません")

    for area in SCOPED_AREAS:
        agents = ROOT / area / "AGENTS.md"
        claude = ROOT / area / "CLAUDE.md"
        if not agents.is_file():
            error(f"スコープ別指示がありません: {area}/AGENTS.md")
        if not claude.is_file():
            error(f"スコープ別 Claude adapter がありません: {area}/CLAUDE.md")
        elif "@AGENTS.md" not in claude.read_text(encoding="utf-8"):
            error(f"{area}/CLAUDE.md は同じディレクトリの AGENTS.md を import してください")

    if (ROOT / ".cursorrules").exists():
        error("旧形式の .cursorrules は使用できません。.cursor/rules/*.mdc を使用してください")
    if (ROOT / ".claude/commands").exists():
        error("旧形式の .claude/commands は使用できません。.claude/skills/*/SKILL.md を使用してください")


def validate_skills() -> None:
    for name in SKILL_NAMES:
        codex_path = ROOT / ".agents/skills" / name / "SKILL.md"
        claude_path = ROOT / ".claude/skills" / name / "SKILL.md"
        if not codex_path.is_file() or not claude_path.is_file():
            continue

        codex_text = codex_path.read_text(encoding="utf-8")
        claude_text = claude_path.read_text(encoding="utf-8")
        if codex_text != claude_text:
            error(f"Codex／Claude Skill に差分があります: {name}")
        if "出力は日本語" not in codex_text:
            error(f"Skill に日本語出力の指示がありません: {name}")

        if not codex_text.startswith("---\n"):
            error(f"Skill に YAML frontmatter がありません: {name}")
            continue
        try:
            frontmatter = codex_text.split("---", 2)[1]
        except IndexError:
            error(f"Skill の YAML frontmatter が不正です: {name}")
            continue
        if f"name: {name}" not in frontmatter:
            error(f"Skill frontmatter の name が一致しません: {name}")
        if "description:" not in frontmatter:
            error(f"Skill に description がありません: {name}")


def validate_configuration() -> None:
    cargo = load_toml("Cargo.toml")
    toolchain = load_toml("rust-toolchain.toml")
    load_toml(".codex/config.toml")
    settings = load_json(".claude/settings.json")

    channel = toolchain.get("toolchain", {}).get("channel")
    if channel != "1.97.1":
        error(f"rust-toolchain.toml は 1.97.1 に固定してください。現在値: {channel!r}")

    rust_version = cargo.get("workspace", {}).get("package", {}).get("rust-version")
    if rust_version != "1.88":
        error(f"workspace の rust-version は 1.88 にしてください。現在値: {rust_version!r}")

    permissions = settings.get("permissions", {})
    if not isinstance(permissions.get("allow"), list):
        error(".claude/settings.json の permissions.allow は list である必要があります")
    if not isinstance(permissions.get("deny"), list):
        error(".claude/settings.json の permissions.deny は list である必要があります")

    members = cargo.get("workspace", {}).get("members", [])
    for member in members:
        if not (ROOT / member / "Cargo.toml").is_file():
            error(f"workspace member に Cargo.toml がありません: {member}")


def validate_contract_independence() -> None:
    contract_root = ROOT / "crates/lsharp-frame-contract"
    for path in iter_text_files(contract_root, {".rs", ".toml"}):
        text = path.read_text(encoding="utf-8")
        for pattern in FORBIDDEN_CONTRACT_PATTERNS:
            if pattern in text:
                error(f"外部実装型が漏洩しています {path.relative_to(ROOT)}: {pattern}")

    cargo = load_toml("crates/lsharp-frame-contract/Cargo.toml")
    dependencies = set(cargo.get("dependencies", {})) | set(cargo.get("dev-dependencies", {}))
    leaked = sorted(dependencies & FORBIDDEN_CONTRACT_DEPENDENCIES)
    if leaked:
        error(f"lsharp-frame-contract が実装 crate に依存しています: {', '.join(leaked)}")

    wit_text = "\n".join(
        path.read_text(encoding="utf-8")
        for path in iter_text_files(ROOT / "wit", {".wit"})
    )
    for pattern in FORBIDDEN_CONTRACT_PATTERNS:
        crate_name = pattern.removesuffix("::")
        if re.search(rf"\b{re.escape(crate_name)}\b", wit_text, re.IGNORECASE):
            error(f"実装 crate 名が WIT に漏洩しています: {crate_name}")


def validate_markdown_links() -> None:
    for path in iter_text_files(ROOT, {".md", ".mdc"}):
        if ".git" in path.parts:
            continue
        text = path.read_text(encoding="utf-8")
        for raw_target in MARKDOWN_LINK_RE.findall(text):
            target = raw_target.strip().split(maxsplit=1)[0].strip("<>")
            if not target or target.startswith(("#", "http://", "https://", "mailto:")):
                continue
            target = target.split("#", 1)[0]
            if not target:
                continue
            candidate = (path.parent / target).resolve()
            try:
                candidate.relative_to(ROOT.resolve())
            except ValueError:
                error(f"Markdown link がリポジトリ外を参照しています {path.relative_to(ROOT)}: {raw_target}")
                continue
            if not candidate.exists():
                error(f"Markdown link が壊れています {path.relative_to(ROOT)}: {raw_target}")


def validate_task_templates() -> None:
    task = (ROOT / "docs/development/tasks/TEMPLATE.md").read_text(encoding="utf-8")
    for heading in (
        "## 目的",
        "## 非目標",
        "## 受入条件",
        "## 検証",
        "## 引き継ぎ状態",
    ):
        if heading not in task:
            error(f"タスクテンプレートに見出しがありません: {heading}")

    handoff = (ROOT / "docs/development/handoffs/TEMPLATE.md").read_text(encoding="utf-8")
    for heading in ("## 目的", "## 現在の状態", "## 実行した検証", "## 未実行の検証"):
        if heading not in handoff:
            error(f"引き継ぎテンプレートに見出しがありません: {heading}")


def main() -> int:
    validate_required_files()
    if not ERRORS:
        validate_instruction_hierarchy()
        validate_skills()
        validate_configuration()
        validate_contract_independence()
        validate_markdown_links()
        validate_task_templates()

    if ERRORS:
        print("ハーネス検証に失敗しました:", file=sys.stderr)
        for item in ERRORS:
            print(f"- {item}", file=sys.stderr)
        return 1

    print("ハーネス検証: 成功")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
