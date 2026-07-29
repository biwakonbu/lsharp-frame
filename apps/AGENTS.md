# Composition-root instructions

These instructions extend the repository root `AGENTS.md` for `apps/**`.

Application crates are composition roots. They may select concrete adapters and runtime
implementations, but must not become a second location for domain rules.

- Wire dependencies; do not duplicate core orchestration.
- Keep startup, shutdown, and resource ownership explicit.
- Translate configuration into L#frame-owned options before passing it inward.
- Add an integration or smoke test for each new composition root.
- Backend selection must remain replaceable without changing L# plugins or baseline WIT.
