# ADR 0002: Stable L#frame UI IR

- Status: Accepted
- Date: 2026-07-29

## Context

The initial GUI toolkit may be replaced as the Rust ecosystem evolves. Exposing toolkit widgets to L# would bind all plugins to that implementation.

## Decision

L# produces an L#frame-owned, retained UI document and atomic UI transactions. Desktop adapters translate this IR into concrete toolkit structures.

## Consequences

- HTML/DOM and toolkit-specific widget types are not public contracts.
- A toolkit replacement requires a new adapter but not plugin rewrites.
- Backend-specific functions are isolated in versioned extensions.
- UI conformance is semantic, not only pixel-perfect.
