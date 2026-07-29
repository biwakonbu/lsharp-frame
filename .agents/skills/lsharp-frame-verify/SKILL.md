---
name: lsharp-frame-verify
description: Verify an L#frame working tree or branch without changing product behavior. Use before handoff, review, commit, or pull-request readiness decisions.
---

# Verify an L#frame change

1. Read `AGENTS.md` and scoped instructions.
2. Inspect `git status` and the complete diff.
3. Run `make doctor`, `make harness`, and `make ci` when the required tools are available.
4. Run `make wit-check` for WIT changes and L# checks when `lsharp` is available.
5. Map failures to root causes; do not weaken lint, compatibility, or safety gates.
6. Report exact commands, results, unavailable tools, and residual risk.
