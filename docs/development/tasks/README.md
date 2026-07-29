# Task Records

Task records are execution state for cross-cutting or multi-session work. They are not a
second source of architectural truth.

Create one with:

```bash
make new-task SLUG=short-kebab-case-name
```

Keep the record current while work is active. On completion, retain concise outcome and
evidence links or replace durable decisions with an ADR and close the task state.
