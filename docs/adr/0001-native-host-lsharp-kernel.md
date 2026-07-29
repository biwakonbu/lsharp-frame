# ADR 0001: Rust native host and L#frame kernel

- Status: Accepted
- Date: 2026-07-29

## Context

The product requires native GPU rendering, IME, accessibility, PTY, process, filesystem, and plugin isolation while keeping application behavior programmable in L#.

## Decision

Rust owns native mechanisms and Wasmtime supervision. L# runs as a long-lived Component and owns application state, commands, keymaps, workflows, UI composition, and plugin behavior.

## Consequences

- L# is not a restricted UI template language.
- Native capabilities must be exposed through typed WIT contracts.
- Product logic must not drift into concrete Rust adapters.
- Component boundary cost must be measured and batched.
