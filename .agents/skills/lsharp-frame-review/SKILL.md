---
name: lsharp-frame-review
description: Review an L#frame diff for correctness, architecture, safety, performance, and missing evidence. Use for pre-PR or pull-request review.
---

# Review an L#frame change

Review the complete diff against `AGENTS.md`, scoped instructions, architecture documents, and acceptance criteria.

Return findings first, ordered by severity, with file and line evidence. Treat these as blocking:

- external implementation types leaking into stable contracts;
- fine-grained L#/Rust calls in hot paths;
- fail-open capability behavior;
- UI-thread blocking by plugin work;
- PTY/process ordering, bytes, cancellation, or lifecycle loss;
- baseline APIs that silently depend on one backend;
- tests that miss observable behavior;
- validation claims for commands that were not executed.

Then list assumptions, missing checks, and a concise change summary.
