# ADR 0003: Capability and resource boundary

- Status: Accepted
- Date: 2026-07-29

## Context

Plugins need broad access to PTY, processes, streams, filesystem, network, clipboard, and OS services. Direct pointer, file descriptor, or crate object access is unsafe and non-portable.

## Decision

Plugins issue typed effects and operate on L#frame-owned stable IDs/resources. Rust maps those logical handles to native objects and enforces capability policy.

## Consequences

- "Not direct" does not mean "not available".
- Trusted plugins can receive broad capabilities equivalent to arbitrary user-level execution.
- All effects are auditable.
- High-throughput data can remain host-side and be observed through ranges or semantic events.
