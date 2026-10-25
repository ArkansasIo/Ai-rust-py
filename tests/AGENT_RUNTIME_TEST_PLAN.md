# Agent Runtime Tests

Unit coverage should verify:

1. Agent registry resolves built-in agents.
2. Unknown agents are rejected.
3. Read-only tools do not require approval.
4. Write and commit tools require approval.
5. Shell execution remains disabled by default.
6. Workflow state checkpoints increment iterations.
7. Workflow definitions reject invalid graph edges.
8. Security policy denies unauthorized writes.

Integration tests should run against a disposable Ollama instance and MySQL database in CI.
