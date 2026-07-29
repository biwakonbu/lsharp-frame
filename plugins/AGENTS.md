# L# plugin instructions

These instructions extend the repository root `AGENTS.md` for `plugins/**`.

- Declare required and optional capabilities explicitly.
- Keep host interaction in typed effects.
- Prefer named commands and inspectable extension points over hidden global mutation.
- Provide deterministic fixtures for state transitions and UI transactions.
- Avoid raw high-volume stream subscriptions unless the plugin actually needs the bytes.
- A trusted capability such as arbitrary process execution must be documented as equivalent
  to user-level arbitrary code execution.
