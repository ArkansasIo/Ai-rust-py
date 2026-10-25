# Agent Execution Architecture

## Execution lifecycle

1. Client submits a task.
2. Runtime validates the task and selects an agent.
3. A workflow creates a durable task record.
4. The worker claims the task.
5. The agent invokes Ollama and approved tools.
6. Each important workflow transition creates a checkpoint.
7. Tool calls are evaluated against the security policy.
8. Medium/high-risk operations require approval.
9. Results are persisted and returned to the client.

## Workspace isolation

Each coding task receives an isolated workspace. Relative-path validation prevents `..` traversal and absolute-path escapes.

## Git integration

Read-only Git operations are available through the runtime foundation:

- status
- diff
- log

Mutating Git operations should remain behind an explicit approval gate.

## Streaming

The Rust Ollama streaming adapter consumes newline-delimited JSON responses and exposes generated token chunks to the HTTP layer.

## Production queue

The repository includes an in-memory queue and a Python worker lifecycle. Production deployments should replace the queue boundary with Redis, NATS, RabbitMQ, or another durable broker.

## Safety boundary

Shell execution is disabled by default. File writes and other state-changing operations must pass policy checks and should be paired with human approval in production.
