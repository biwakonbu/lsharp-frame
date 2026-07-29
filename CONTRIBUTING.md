# Contributing to L#frame

## Start here

Read the root [`AGENTS.md`](AGENTS.md), the closest scoped `AGENTS.md`, and the relevant
architecture/ADR documents before changing code. Codex is the primary implementation path,
but all committed instructions and validation must remain usable from Claude Code, Cursor,
and ordinary shell workflows.

## Canonical workflow

```bash
make doctor
make context
# For cross-cutting or multi-session work:
make new-task SLUG=short-kebab-case-name

# Iterate with narrow checks, then:
make harness
make ci
```

Do not claim a command passed when it was not executed. Record unavailable checks and the
exact environmental reason in the pull request or handoff.

## Core boundary rule

External crate types must not appear in `lsharp-frame-contract`, WIT, or the L# plugin API.
Crate-specific conversions stay inside adapter crates, and concrete adapter selection stays
inside application composition roots.

## Change categories

- **Contract change**: `lsharp-frame-contract` or `wit/`; requires compatibility, migration,
  round-trip/conformance evidence, and coordinated Rust/L# documentation.
- **Core change**: event routing, capability enforcement, resource lifecycle, or kernel
  execution; requires deterministic observable-behavior tests.
- **Adapter change**: crate/OS-specific implementation; baseline contracts remain unchanged
  and the shared conformance suite must pass.
- **Hot-path change**: boundary calls, PTY/terminal streams, large timelines, rendering, or
  allocation; requires workload and measurement evidence.
- **Plugin/kernel change**: L# behavior or SDK; host effects remain explicit and capability
  checked.

## Pull requests

Use the repository pull-request template. Include objective, non-goals, affected boundaries,
exact validation results, unexecuted checks, compatibility impact, and residual risk.
