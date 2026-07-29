---
name: lsharp-frame-implement
description: Implement a scoped L#frame change from agreed acceptance criteria. Use when editing Rust, WIT, L#, adapters, plugins, tests, or engineering documentation.
---

# Implement an L#frame change

1. Read `AGENTS.md`, the closest scoped `AGENTS.md`, and any active task record.
2. Inspect existing code and tests; do not infer unseen behavior.
3. Implement the smallest coherent slice that satisfies the acceptance criteria.
4. Preserve contract/adapter boundaries and keep host effects capability-checked.
5. Add or update observable-behavior tests with the implementation.
6. Run narrow checks while iterating, then `make harness` and applicable `make` validation targets.
7. Review the final diff for unrelated changes, architecture leakage, and missing evidence.
8. Report every unexecuted check and its exact reason.
