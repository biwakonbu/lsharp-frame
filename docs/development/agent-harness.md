# Development Agent Harness

## Purpose

L#frame is developed with Codex, Claude Code, and Cursor. Codex is the default implementation
agent, but repository knowledge must remain portable between all three. The harness therefore
keeps one tool-neutral policy source and treats tool-specific files as adapters.

## Instruction hierarchy

```text
AGENTS.md                         repository-wide canonical policy
<area>/AGENTS.md                  scoped policy for crates/apps/kernel/plugins/wit/docs
CLAUDE.md                         thin import adapter for Claude Code
<area>/CLAUDE.md                  scoped Claude import adapter
.cursor/rules/*.mdc               Cursor activation/glob adapters
.codex/config.toml                project execution defaults, not architecture policy
```

The current task, repository files, tests, and recorded decisions are authoritative. Private
chat memory is not an acceptable dependency for implementation or handoff.

## Canonical workflow

```text
inspect
  -> define objective/non-goals/acceptance
  -> create task record when cross-cutting
  -> implement smallest coherent slice
  -> run narrow checks
  -> make harness
  -> make ci
  -> review diff
  -> handoff with evidence
```

Use `make new-task SLUG=<slug>` when work changes a stable contract, spans multiple subsystems,
is expected to cross sessions, or needs explicit migration/benchmark evidence. Small localized
fixes do not require a task record.

## Tool roles

### Codex

Codex is the primary coding path. It reads the root and nearest scoped `AGENTS.md` and uses
`.codex/config.toml` for project-local execution defaults. Model/provider/authentication
settings are intentionally not committed.

### Claude Code

`CLAUDE.md` imports the canonical instructions. `.claude/settings.json` grants a narrow set of
read-only Git and repository validation commands while denying destructive Git/shell commands
and likely secret files. User-local permission changes belong in
`.claude/settings.local.json`, which is ignored.

### Cursor

`.cursor/rules/*.mdc` selects the same canonical instructions by path. Commands under
`.cursor/commands/` mirror the shared planning, implementation, verification, review, and
handoff workflow without redefining architecture.

## Repository commands

All agents use `make` targets so validation does not drift between tools:

```text
make doctor
make context
make harness
make fmt-check
make check
make lint
make test
make wit-check
make ci
```

`make doctor` is diagnostic and reports unavailable optional tools without failing.
`make harness` is independent of the Rust compiler and validates configuration, instruction
hierarchy, Markdown links, workspace membership, and implementation-type leakage into stable
contracts.

## Agent handoff

Use [the handoff template](handoffs/TEMPLATE.md) when another agent or session must continue
work. A valid handoff names exact checks and unexecuted steps; it does not merely summarize
intent.

## Local-only configuration

Do not commit:

```text
.claude/settings.local.json
.cursor/mcp.json
.mcp.json
.env*
.codex-log/
```

Secrets, personal model preferences, MCP credentials, and machine-specific paths stay out of
the repository.
