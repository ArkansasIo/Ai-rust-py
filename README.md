# Ai-rust-py

Rust + Python + MySQL AI/ML platform with Ollama, agent workflows and RAG foundations.

## Services

| Service | Port | Responsibility |
|---|---:|---|
| Rust API | 8080 | Gateway, agents, workflows, persistence |
| Python ML | 8001 | ML training/inference/evaluation/RAG helpers |
| Ollama | 11434 | Local LLM + embeddings |
| MySQL | 3306 | Durable application and ML state |

## Quick start

```bash
cp .env.example .env
docker compose build
docker compose up -d
```

Then install a model in Ollama, for example:

```bash
docker compose exec ollama ollama pull llama3.2
```

Health:

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8001/health
```

Chat:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/chat \
  -H 'content-type: application/json' \
  -d '{"message":"Explain supervised learning in three sentences."}'
```

Agent:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/agents/run \
  -H 'content-type: application/json' \
  -d '{"agent":"researcher","task":"Compare Rust and Python for AI orchestration."}'
```

## Architecture

```text
Client
  -> Rust Axum Gateway
     -> Agent Runtime / Workflow Engine
     -> Ollama
     -> Python ML
     -> MySQL / Memory
     -> RAG / Embeddings
```

The design intentionally keeps model inference separate from orchestration so additional local or hosted model providers can be added later.

## Production roadmap

Authentication and authorization, secret management, isolated tool sandboxes, vector database, streaming chat, durable task queues, observability, model evaluation, GitHub coding agents and human approval gates are the next production layers.
