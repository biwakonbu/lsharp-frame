# L# kernel instructions

These instructions extend the repository root `AGENTS.md` for `kernel/**`.

The L#frame Kernel owns product behavior: state transitions, commands, keymaps, hooks,
workflows, UI composition, and plugin lifecycle policy.

- Native mechanisms are requested as typed effects; do not encode OS-specific behavior.
- Keep event handling deterministic where possible.
- Do not move PTY byte processing, terminal rendering, or other high-volume loops into L#.
- Preserve coarse-grained `EventBatch -> FrameUpdate` calls.
- Update WIT and Rust projections when changing a public kernel contract.
- Report L# compile/test steps as not run when the compiler is unavailable.
