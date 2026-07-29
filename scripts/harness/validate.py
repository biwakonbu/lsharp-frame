#!/usr/bin/env python3
"""Validate agent harness files and stable L#frame repository boundaries."""

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
        error(f"invalid TOML {relative}: {exc}")
        return {}


def load_json(relative: str) -> dict:
    path = ROOT / relative
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        error(f"invalid JSON {relative}: {exc}")
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
            error(f"required harness file missing: {relative}")

    for name in SKILL_NAMES:
        for root in (".agents/skills", ".claude/skills"):
            relative = f"{root}/{name}/SKILL.md"
            if not (ROOT / relative).is_file():
                error(f"required workflow skill missing: {relative}")


def validate_instruction_hierarchy() -> None:
    root_agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    if "canonical repository-wide instruction source" not in root_agents:
        error("AGENTS.md must identify itself as the canonical instruction source")

    root_claude = (ROOT / "CLAUDE.md").read_text(encoding="utf-8")
    if "@AGENTS.md" not in root_claude:
        error("CLAUDE.md must import @AGENTS.md")
    if len(root_claude.splitlines()) > 12:
        error("CLAUDE.md is no longer a thin adapter; keep policy in AGENTS.md")

    cursor_project = (ROOT / ".cursor/rules/00-project.mdc").read_text(encoding="utf-8")
    if "alwaysApply: true" not in cursor_project or "@AGENTS.md" not in cursor_project:
        error("Cursor project rule must always apply and reference @AGENTS.md")

    for area in SCOPED_AREAS:
        agents = ROOT / area / "AGENTS.md"
        claude = ROOT / area / "CLAUDE.md"
        if not agents.is_file():
            error(f"scoped instructions missing: {area}/AGENTS.md")
        if not claude.is_file():
            error(f"scoped Claude adapter missing: {area}/CLAUDE.md")
        elif "@AGENTS.md" not in claude.read_text(encoding="utf-8"):
            error(f"{area}/CLAUDE.md must import its sibling AGENTS.md")

    if (ROOT / ".cursorrules").exists():
        error("legacy .cursorrules is not allowed; use .cursor/rules/*.mdc")
    if (ROOT / ".claude/commands").exists():
        error("legacy .claude/commands is not allowed; use .claude/skills/*/SKILL.md")


def validate_skills() -> None:
    for name in SKILL_NAMES:
        codex_path = ROOT / ".agents/skills" / name / "SKILL.md"
        claude_path = ROOT / ".claude/skills" / name / "SKILL.md"
        if not codex_path.is_file() or not claude_path.is_file():
            continue

        codex_text = codex_path.read_text(encoding="utf-8")
        claude_text = claude_path.read_text(encoding="utf-8")
        if codex_text != claude_text:
            error(f"Codex/Claude skill drift detected: {name}")

        if not codex_text.startswith("---\n"):
            error(f"skill is missing YAML frontmatter: {name}")
            continue
        try:
            frontmatter = codex_text.split("---", 2)[1]
        except IndexError:
            error(f"skill has malformed YAML frontmatter: {name}")
            continue
        if f"name: {name}" not in frontmatter:
            error(f"skill frontmatter name mismatch: {name}")
        if "description:" not in frontmatter:
            error(f"skill description missing: {name}")


def validate_configuration() -> None:
    cargo = load_toml("Cargo.toml")
    toolchain = load_toml("rust-toolchain.toml")
    load_toml(".codex/config.toml")
    settings = load_json(".claude/settings.json")

    channel = toolchain.get("toolchain", {}).get("channel")
    if channel != "1.97.1":
        error(f"rust-toolchain.toml must pin 1.97.1, found {channel!r}")

    rust_version = cargo.get("workspace", {}).get("package", {}).get("rust-version")
    if rust_version != "1.88":
        error(f"workspace rust-version must be 1.88, found {rust_version!r}")

    permissions = settings.get("permissions", {})
    if not isinstance(permissions.get("allow"), list):
        error(".claude/settings.json permissions.allow must be a list")
    if not isinstance(permissions.get("deny"), list):
        error(".claude/settings.json permissions.deny must be a list")

    members = cargo.get("workspace", {}).get("members", [])
    for member in members:
        if not (ROOT / member / "Cargo.toml").is_file():
            error(f"workspace member has no Cargo.toml: {member}")


def validate_contract_independence() -> None:
    contract_root = ROOT / "crates/lsharp-frame-contract"
    for path in iter_text_files(contract_root, {".rs", ".toml"}):
        text = path.read_text(encoding="utf-8")
        for pattern in FORBIDDEN_CONTRACT_PATTERNS:
            if pattern in text:
                error(f"external implementation type leaked into {path.relative_to(ROOT)}: {pattern}")

    cargo = load_toml("crates/lsharp-frame-contract/Cargo.toml")
    dependencies = set(cargo.get("dependencies", {})) | set(cargo.get("dev-dependencies", {}))
    leaked = sorted(dependencies & FORBIDDEN_CONTRACT_DEPENDENCIES)
    if leaked:
        error(f"lsharp-frame-contract depends on implementation crates: {', '.join(leaked)}")

    wit_text = "\n".join(
        path.read_text(encoding="utf-8")
        for path in iter_text_files(ROOT / "wit", {".wit"})
    )
    for pattern in FORBIDDEN_CONTRACT_PATTERNS:
        crate_name = pattern.removesuffix("::")
        if re.search(rf"\b{re.escape(crate_name)}\b", wit_text, re.IGNORECASE):
            error(f"implementation crate name leaked into WIT: {crate_name}")


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
                error(f"markdown link escapes repository in {path.relative_to(ROOT)}: {raw_target}")
                continue
            if not candidate.exists():
                error(f"broken markdown link in {path.relative_to(ROOT)}: {raw_target}")


def validate_task_templates() -> None:
    task = (ROOT / "docs/development/tasks/TEMPLATE.md").read_text(encoding="utf-8")
    for heading in (
        "## Objective",
        "## Non-goals",
        "## Acceptance criteria",
        "## Validation",
        "## Handoff state",
    ):
        if heading not in task:
            error(f"task template missing heading: {heading}")


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
        print("harness validation failed:", file=sys.stderr)
        for item in ERRORS:
            print(f"- {item}", file=sys.stderr)
        return 1

    print("harness validation: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
