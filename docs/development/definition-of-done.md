# Definition of Done

A change is complete only when all applicable conditions below are met.

## Behavior

- The objective and observable acceptance criteria are satisfied.
- Explicit non-goals remain out of scope.
- Failure, cancellation, ordering, and resource-lifecycle behavior are defined where relevant.

## Architecture

- Stable contracts contain only L#frame-owned semantic types.
- Concrete crate/OS/toolkit types remain inside adapters or composition roots.
- Capability checks fail closed.
- No fine-grained Component boundary calls are introduced into a hot path.
- Product behavior remains in L# or core policy rather than drifting into a concrete adapter.

## Evidence

- Relevant tests exist and pass.
- `make harness` passes.
- `make ci` passes, or each unavailable step is named with the exact environmental reason.
- Benchmarks are updated for hot-path changes.
- Conformance evidence exists for adapter changes.
- Compatibility and migration evidence exists for WIT or stable contract changes.

## Documentation and review

- Documentation and ADRs are updated where the observable contract changed.
- The final diff contains no unrelated changes, generated noise, credentials, or machine paths.
- Review found no blocking issue under the root `AGENTS.md` criteria.
- Handoff state is recorded when work is incomplete or crosses sessions.
