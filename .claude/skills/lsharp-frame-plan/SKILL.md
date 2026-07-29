---
name: lsharp-frame-plan
description: Plan an L#frame repository change before implementation. Use for cross-cutting work, contract changes, migrations, or any task that needs explicit acceptance criteria and validation.
---

# Plan an L#frame change

1. Read `AGENTS.md`, the closest scoped `AGENTS.md`, and relevant architecture/ADR files.
2. Inspect the current implementation and tests before proposing changes.
3. State the objective, observable outcome, and explicit non-goals.
4. Identify affected Rust, WIT, L#, capability, resource-lifecycle, and performance boundaries.
5. Define deterministic acceptance criteria and evidence.
6. Order implementation steps by dependency and list exact validation commands.
7. Record compatibility, migration, and residual risks.
8. For cross-cutting or multi-session work, run `make new-task SLUG=<short-kebab-case-name>` and fill the task record.

Do not modify product code unless implementation was also requested.
