# Rust workspace instructions

These instructions extend the repository root `AGENTS.md` for `crates/**`.

## Scope

- Keep crate responsibilities aligned with `docs/architecture.md`.
- `lsharp-frame-contract` contains stable semantic data only and must not depend on GUI
  toolkits, renderer crates, async runtimes, PTY crates, or OS bindings.
- `lsharp-frame-spi` defines ports in terms of L#frame-owned types.
- `lsharp-frame-core` orchestrates ports and contracts without selecting concrete adapters.
- Adapter crates may depend on external implementations, but their public API must return
  L#frame-owned types.

## Required checks

Run the narrow crate test while iterating, then:

```bash
make fmt-check
make lint
make test
make harness
```

For public contract changes, update WIT/L# projections and add compatibility evidence in one
change. Do not hide a breaking change behind a broad `non_exhaustive` or opaque payload
without documenting the migration model.
