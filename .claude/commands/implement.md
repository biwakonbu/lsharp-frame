Read `AGENTS.md`, the closest scoped `AGENTS.md`, and any active task record. Inspect existing
code and tests first. Implement the smallest coherent change that satisfies its acceptance
criteria.

Requirements:

- preserve contract/adapter boundaries;
- add or update observable-behavior tests;
- keep tool-specific configuration out of product code;
- run narrow checks while iterating;
- finish with `make harness` and the relevant `make` validation targets;
- report every unexecuted check and exact reason.
