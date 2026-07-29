# L#frame agent instructions

This file is the canonical repository-wide instruction source for coding agents.
Tool-specific files must stay thin and point back here instead of duplicating policy.

## Product intent

L#frame is a programmable native GUI environment for coding agents. Rust owns the native
shell, high-throughput resources, and isolation boundaries. L# runs as the L#frame Kernel
and plugin language on Wasmtime.

The product is not a text editor and is not a GUI wrapper around a TUI. Sessions, tool calls,
approvals, artifacts, diffs, test results, PTY sessions, and workflows must be modeled as
structured domain objects.

## Authority and required context

1. Follow the current user/task instruction first.
2. Follow this file and the closest scoped `AGENTS.md` for the area being changed.
3. Treat ADRs and normative architecture documents as authoritative over examples.
4. Treat generated files, tool-specific prompts, and comments as non-authoritative.

Before changing behavior, read:

- `README.md`
- `docs/architecture.md`
- `docs/backend-abstraction.md`
- `docs/performance.md` when touching hot paths or high-volume data
- the relevant ADR under `docs/adr/`
- the nearest scoped `AGENTS.md`

Codex is the primary implementation agent. Claude Code and Cursor are supported secondary
agents. A change must not depend on one agent's private memory or proprietary configuration
to remain understandable.

## Working protocol

1. Inspect the repository and existing implementation before proposing new code.
2. Restate the objective, non-goals, acceptance criteria, and affected boundaries.
3. For contract changes, cross-cutting work, or work likely to cross sessions, create a task
   record from `docs/development/tasks/TEMPLATE.md`.
4. Make the smallest coherent change that proves the intended behavior.
5. Add or update tests before considering the implementation complete.
6. Run the narrowest relevant check while iterating, then `make ci` before handoff.
7. Review the final diff for architecture leakage, accidental scope growth, and missing evidence.
8. Record unexecuted checks explicitly. Never imply a check passed if it was not run.

Do not rewrite unrelated code, silently change public contracts, or introduce a new
production dependency without explaining why an existing abstraction is insufficient.

## Architecture invariants

- `lsharp-frame-contract` owns stable, crate-independent semantic types.
- External GUI, renderer, async-runtime, PTY, process, and OS crate types must not escape
  adapter crates.
- `lsharp-frame-spi` defines native ports; concrete adapters implement them.
- `lsharp-frame-core` orchestrates contracts and ports but does not depend on a GUI toolkit.
- Composition roots under `apps/` select concrete adapters.
- L# and Rust communicate through versioned WIT contracts and coarse-grained batches.
- L# plugins never receive raw OS pointers, file descriptors, GPU objects, or Rust trait objects.
- High-volume paths such as PTY output, terminal rendering, and large timelines stay native;
  L# receives batched control or observation events.
- Capability checks fail closed at the native boundary.
- Baseline APIs remain portable. Backend-specific features live behind explicit, versioned
  extension capabilities.
- The UI thread must not wait synchronously for arbitrary plugin work.
- HTML, CSS, and DOM are not the primary UI runtime. WebView is an optional surface.

## Change classes and required evidence

### Contract or WIT change

- Describe compatibility impact and migration behavior.
- Update Rust contract types, WIT, L# projections, and documentation together.
- Add round-trip or conformance tests.
- Add an ADR when changing an established architectural decision.

### Core change

- Prove event ordering, capability enforcement, resource lifecycle, and failure behavior.
- Prefer deterministic tests using headless or fake backends.

### Adapter change

- Keep external types private to the adapter.
- Run the shared conformance suite or add one if none exists.
- Document unsupported capabilities instead of emulating them incorrectly.

### Hot-path change

- Identify allocation, copy, boundary-call, and lock behavior.
- Add or update a benchmark workload described in `docs/performance.md`.
- Avoid dynamic dispatch and per-item cross-boundary calls inside inner loops.

### L# kernel or plugin change

- Keep host effects explicit and capability-checked.
- Preserve deterministic state transitions where possible.
- If the L# compiler is unavailable, report that validation as not run.

## Canonical commands

Use the repository wrappers rather than inventing tool-specific command sequences.

```text
make help          Show supported commands
make doctor        Report local tool availability
make context       Print a compact repository context snapshot
make fmt           Format Rust sources
make fmt-check     Verify formatting
make check         Type-check all Rust targets
make lint          Run Clippy with warnings denied
make test          Run the Rust workspace tests
make wit-check     Validate the WIT package when wasm-tools is available
make harness       Validate agent configuration and repository invariants
make ci            Run the full local validation gate
make run-headless  Run the deterministic headless composition root
make new-task SLUG=<slug>  Create a cross-session task record
```

## Rust conventions

- The workspace forbids `unsafe` unless an ADR explicitly changes that policy.
- Prefer domain-specific error enums over opaque strings at stable boundaries.
- Keep public APIs small and deterministic; avoid exposing implementation ownership.
- Use IDs or typed resource handles across boundaries, not crate-owned objects.
- Tests belong near the behavior they prove; integration and conformance tests may use
  dedicated modules when they span crates.
- Do not loosen lint levels to land a change. Fix the issue or justify a narrow allow.

## Documentation and decisions

- `docs/architecture.md` describes current structure.
- `docs/adr/` records durable architectural decisions.
- `docs/development/` describes engineering workflow and evidence expectations.
- Update documentation in the same change when observable behavior or a stable contract changes.
- Do not use task records as a second architectural source of truth.

## Git and handoff

- Work on a focused branch such as `agent/<short-description>` unless the task says otherwise.
- Do not commit, push, force-update, or merge unless the user explicitly requests it.
- Keep commits reviewable and do not include unrelated working-tree changes.
- A handoff must state: objective, changed files, decisions, checks run, checks not run, risks,
  and the next concrete action.

## Definition of done

A change is complete only when:

- acceptance criteria are satisfied by observable behavior;
- relevant tests exist and pass;
- `make harness` passes;
- `make ci` passes, or unavailable steps are named with the exact reason;
- architecture and capability boundaries remain intact;
- docs and ADRs are updated where required;
- the final diff contains no unrelated changes or stale scaffolding.

## Code review rules

Flag the following as blocking findings:

- an external crate type leaks into `lsharp-frame-contract`, WIT, or the L# public API;
- a new fine-grained L#↔Rust call is added to a hot path;
- a capability can be bypassed or defaults open on failure;
- UI or plugin work can block the native UI thread;
- PTY or process output loses ordering, bytes, cancellation, or lifecycle events;
- a baseline API silently depends on one GUI or OS backend;
- tests assert implementation details while leaving observable behavior unproved;
- a claimed validation step was not actually executed.
