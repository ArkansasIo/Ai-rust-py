# Ai-rust-py

Rust + Python + MySQL AI/ML platform with Ollama, an explicit agent runtime, workflow checkpoints, tool authorization, and RAG foundations.

## Services

| Service | Port | Responsibility |
|---|---:|---|
| Rust API | 8080 | Gateway, agents, workflows, tools, security policy, persistence |
| Python ML | 8001 | ML training/inference/evaluation/RAG helpers |
| Ollama | 11434 | Local LLM + embeddings |
| MySQL | 3306 | Durable application and ML state |

## Quick start

```bash
cp .env.example .env
docker compose build
docker compose up -d
docker compose exec ollama ollama pull llama3.2
```

Health:

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8001/health
```

List agents and tools:

```bash
curl http://127.0.0.1:8080/api/v1/agents
curl http://127.0.0.1:8080/api/v1/tools
curl http://127.0.0.1:8080/api/v1/policy
```

Run an agent:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/agents/run \
  -H 'content-type: application/json' \
  -d '{"agent":"researcher","task":"Compare Rust and Python for AI orchestration."}'
```

Authorize a tool:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/tools/authorize \
  -H 'content-type: application/json' \
  -d '{"tool":"filesystem.write","approved":true}'
```

## Agent runtime

```text
Client
  -> Rust Axum Gateway
     -> AgentRuntime
        -> AgentRegistry
        -> WorkflowState / Checkpoints
        -> ToolRegistry
        -> SecurityPolicy / Approval Gate
        -> Ollama
     -> Python ML
     -> MySQL / Memory
     -> RAG / Embeddings
```

Built-in agent roles are `general`, `researcher`, `coder`, `reviewer`, and `synthesizer`.

Privileged actions are deliberately separated from model reasoning. Shell execution and network tools are disabled by default, while write and commit operations require explicit approval.

## Project structure

```text
rust_api/
  src/
    agents.rs
    runtime.rs
    tools.rs
    security.rs
    models.rs
    workflow.rs
    workflows.rs
    workflow_loader.rs
    ollama.rs
    api/

python_ml/
python_sdk/
database/
workflows/
config/
docs/
tests/
```

## Roadmap

Next production layers are streaming Ollama responses, durable task workers, isolated workspaces, Git/GitHub adapters, vector retrieval, authentication/authorization, OpenTelemetry, evaluation pipelines, and human-in-the-loop approval APIs.
