# Development Harness

- Slug: `development-harness`
- Created: `2026-07-29`
- Status: implemented; awaiting CI
- Owner/agent: Codex-compatible harness bootstrap

## Objective

Provide a repository-owned development harness that lets Codex, Claude Code, Cursor, and
ordinary shell users share one architecture policy, one validation command surface, and an
evidence-based handoff workflow. Codex is the primary implementation path.

## Non-goals

- Implement the native GUI, Wasmtime runtime, PTY backend, or agent protocol.
- Design the L# version manager or package distribution system.
- Commit personal model choices, credentials, MCP endpoints, or machine-specific settings.

## Context and authority

- Relevant architecture documents: `docs/architecture.md`, `docs/backend-abstraction.md`,
  `docs/performance.md`
- Relevant ADRs: `docs/adr/0001-native-host-lsharp-kernel.md`,
  `docs/adr/0002-stable-ui-ir.md`, `docs/adr/0003-capability-resource-boundary.md`
- Existing implementation/tests: initial L#frame Rust/L#/WIT scaffold on `main`

## Affected boundaries

- Rust contract/core/adapter: no product contract change; adds static architecture-leak checks.
- WIT/L#: no public contract change; adds scoped agent instructions.
- Capability/resource lifecycle: no runtime change.
- Performance/hot path: no runtime change.

## Acceptance criteria

- [x] `AGENTS.md` is the canonical policy with scoped area instructions.
- [x] Claude Code and Cursor use thin adapters rather than duplicated architecture policy.
- [x] Codex project defaults are committed without personal model/provider/auth settings.
- [x] All tools share `make` targets for context, validation, tests, and handoff preparation.
- [x] The harness validates configuration syntax, links, workspace membership, and stable
      contract independence without requiring a Rust compiler.
- [x] CI runs the harness before the pinned Rust workspace gate.
- [x] PR, task, and handoff templates require exact evidence and unexecuted-check disclosure.

## Implementation plan

1. Establish root and scoped canonical instruction files.
2. Add thin Codex, Claude Code, and Cursor adapters and reusable commands.
3. Add Makefile wrappers and deterministic harness validation scripts.
4. Add task/handoff/definition-of-done documentation and PR template.
5. Pin the Rust toolchain and run harness checks locally and Rust checks in CI.

## Validation

```text
make harness                                      PASS in assembled repository snapshot
make doctor                                       PASS (diagnostic; reports missing tools)
git diff --check                                  PASS
cargo fmt/check/clippy/test                        pending GitHub Actions
wasm-tools component wit wit/frame.wit --json     pending GitHub Actions/local tool
L# compile/test                                    NOT RUN: lsharp unavailable
```

## Decisions and risks

- Decision: repository policy lives in `AGENTS.md`; tool-specific files stay thin.
- Decision: `make` is the canonical command surface for agents and humans.
- Decision: Rust is pinned to `1.97.1` until an explicit upgrade change is reviewed.
- Risk: Claude/Cursor permission and command-file schemas may evolve; the adapters are kept
  small so such changes do not affect product policy.

## Handoff state

- Current state: harness implementation and static validation complete; CI is the next gate.
- Changed files: agent configs, Makefile/scripts, development docs/templates, CI/toolchain.
- Checks run: assembled-repository `make harness`, shell syntax, JSON/TOML/link checks.
- Checks not run and reason: Rust/L# checks unavailable in the current execution environment.
- Next concrete action: inspect draft-PR CI and fix any toolchain/runtime discrepancy.
