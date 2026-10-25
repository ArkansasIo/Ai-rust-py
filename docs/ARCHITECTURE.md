# Ai-rust-py Architecture v0.2

Ai-rust-py is a Rust-centered AI platform combining an Axum gateway, Ollama model service, Python ML service, MySQL state, agent workflows and RAG foundations.

## Runtime

Client -> Rust API -> Agent/Workflow Runtime -> Ollama or Python ML -> Memory/RAG -> response.

Rust owns orchestration, API boundaries, security policy and persistence. Python owns classical ML/training/evaluation. Ollama supplies local LLM and embedding inference.

## Agent model

Agents are specialized roles: planner, researcher, coder, critic, reviewer and synthesizer. Workflows can chain agents and later add fan-out/fan-in and human approval.

## Safety

Tool execution must pass a permission/risk policy. High and critical operations should require explicit human approval. Production deployments should add authentication, secrets management, sandboxing, rate limits and audit retention.

## Ollama

The gateway calls Ollama's HTTP API for chat and embeddings. The model name is configured with `OLLAMA_MODEL`. Pull models separately before production use.

## RAG

The Python service provides deterministic text chunking and the Rust gateway exposes embedding access. A production vector store can be added without changing the agent API.
