Read `AGENTS.md`, the closest scoped `AGENTS.md`, and the relevant architecture/ADR files.
Inspect the current implementation before writing a plan.

Return a task plan containing:

1. objective and user-visible outcome;
2. explicit non-goals;
3. affected contracts and architectural boundaries;
4. acceptance criteria and evidence;
5. implementation steps ordered by dependency;
6. tests and commands to run;
7. risks, migration concerns, and open questions.

For cross-cutting or multi-session work, create a record with
`make new-task SLUG=<short-kebab-case-name>` and fill it before implementation.
Do not change production code unless the user also asked for implementation.
