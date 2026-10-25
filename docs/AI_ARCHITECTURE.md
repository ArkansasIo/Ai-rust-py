# Ai-rust-py AI Architecture

## Framework layers

1. **API layer** — Axum exposes chat, agents, tools, tasks, workflows, models, embeddings and memory endpoints.
2. **Domain layer** — Rust types define agents, conversations, messages, memories, models, tools, workflows, tasks and users.
3. **Service layer** — validation and orchestration boundaries isolate domain rules from transport and persistence.
4. **Runtime layer** — agent execution, task queues, workflow execution and tool dispatch.
5. **Provider layer** — Ollama and future OpenAI-compatible/local providers.
6. **Python ML layer** — FastAPI, RAG helpers, model adapters and Python agent subclasses.
7. **Persistence layer** — MySQL stores users, agents, conversations, messages, memories, tools, tool calls, workflows, runs, tasks and audit events.
8. **Operations layer** — GitHub Actions, cron jobs, database backups and environment configuration.

## Agent hierarchy

`BaseAgent` is the Python abstract base. Specialized classes currently include `AssistantAgent`, `ResearchAgent`, `CoderAgent`, and `PlannerAgent`. Rust mirrors this with `AgentClass` and `AgentProfile` so the API and Python runtime can share a stable conceptual model.

## Memory hierarchy

Memory is classified as semantic, episodic, procedural, preference, or summary. Retrieval is separated from persistence so an in-memory implementation can be replaced by a database/vector backend without changing agent APIs.

## Tool hierarchy

Every tool has a kind, schema, enabled state, and risk level. Medium, high, and critical actions require approval by default. Shell and arbitrary code execution are disabled in the default seed configuration.

## Workflow hierarchy

A workflow contains ordered/branched steps. Step kinds include prompt, model, tool, condition, transform, memory read/write, Python and Rust. Workflow execution should remain bounded by step count and timeout policies.

## Runtime contract

The AI runtime should follow:

`request -> authenticate -> policy -> context -> memory retrieval -> model generation -> tool approval -> tool execution -> observation -> next step -> response -> persistence -> audit`

Model output is never treated as trusted executable instructions. Tool execution is a separate policy-controlled subsystem.

## Database

`database/schema.sql` contains the original ML tables. `database/migrations/0002_ai_platform.sql` adds the operational AI platform tables. SQLx applies migrations when the Rust service starts.

## Scheduled operations

`cron/crontab.example` defines health checks, task queue checks, and nightly MySQL backups. Production deployments should use the platform's scheduler/systemd/Kubernetes CronJob equivalent rather than copying local paths unchanged.
