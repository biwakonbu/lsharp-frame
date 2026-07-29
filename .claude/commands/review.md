Review the current diff against `AGENTS.md`, scoped instructions, architecture documents,
and acceptance criteria.

Prioritize blocking findings:

- external implementation types leaking into stable contracts;
- fine-grained L#/Rust calls in hot paths;
- fail-open capability behavior;
- UI-thread blocking;
- PTY/process ordering or lifecycle loss;
- backend assumptions in baseline APIs;
- missing observable tests or false validation claims.

Return findings first, ordered by severity, with file/line evidence. Then list assumptions,
missing checks, and a concise change summary.
