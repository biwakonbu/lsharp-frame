# WIT contract instructions

These instructions extend the repository root `AGENTS.md` for `wit/**`.

WIT is a public compatibility boundary.

- Use versioned packages and small, capability-oriented interfaces.
- Prefer L#frame semantic records, variants, lists, and resources.
- Never expose crate names, Rust layouts, OS handles, GPU objects, or trait objects.
- Avoid per-byte, per-glyph, or per-widget boundary calls; define batch operations.
- Document compatibility and migration behavior for every changed public shape.
- Keep baseline portable APIs separate from backend-specific extensions.
