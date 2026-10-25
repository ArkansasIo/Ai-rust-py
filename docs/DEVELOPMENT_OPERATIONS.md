# Development and Operations Guide

This document describes a practical local development workflow for Ai-rust-py.

## Components

- **Rust API** (`rust_api/`): Axum gateway, agent runtime, workflow/task APIs, and MySQL persistence.
- **Python ML service** (`python_ml/`): FastAPI service for Ollama chat, embeddings, RAG utilities, and ML helpers.
- **Ollama**: local model inference. Model availability is separate from service health.
- **MySQL** (`database/`): relational persistence and versioned SQL migrations.

## Start locally with Docker Compose

1. Copy `.env.example` to `.env`.
2. Replace all example passwords with unique local secrets. Do not commit `.env`.
3. Start the stack with `docker compose up --build -d`.
4. Pull the configured chat model: `docker compose exec ollama ollama pull llama3.2`.
5. Check both service health endpoints:

   ```sh
   curl --fail http://127.0.0.1:8080/health
   curl --fail http://127.0.0.1:8001/health
   ```

The first MySQL initialization applies `database/schema.sql`; the Rust API also runs SQLx migrations from `database/migrations`. For an existing named volume, changes to the MySQL init script are not automatically reapplied.

## Development checks

From the repository root:

```sh
cargo fmt --manifest-path rust_api/Cargo.toml --check
cargo check --manifest-path rust_api/Cargo.toml
python -m compileall -q python_ml python_sdk scripts
```

Python dependency installation:

```sh
python -m venv .venv
# macOS/Linux
. .venv/bin/activate
python -m pip install -r python_ml/requirements.txt
```

Run the Python API outside Docker with `uvicorn python_ml.app.main:app --host 127.0.0.1 --port 8001` from the repository root after installing dependencies.

## API smoke checks

```sh
curl --fail http://127.0.0.1:8080/health
curl --fail http://127.0.0.1:8080/api/v1/agents
curl --fail http://127.0.0.1:8080/api/v1/tools
curl --fail http://127.0.0.1:8001/info
```

An HTTP health response does not guarantee that Ollama has downloaded the requested model or that MySQL migrations completed. Check container logs when startup fails:

```sh
docker compose logs --tail=200 rust-api python-ml ollama mysql
```

## Operational safety

- Keep credentials in environment variables or a secret manager; never commit real credentials.
- Change permissive development defaults before exposing the services to a network.
- Keep model-generated tool requests separate from execution. Require explicit approval for writes, commits, shell access, and other privileged actions.
- Do not expose Ollama, MySQL, or the API directly to the public internet without authentication, network restrictions, and TLS.
- Back up the MySQL volume before schema or migration changes.
- Treat prompts, retrieved documents, model output, and external tool output as untrusted input.

## CI

The GitHub Actions workflow runs formatting and Rust compile checks, Python syntax checks, and SQL/JSON configuration validation on pushes and pull requests. It does not download large language models or require production credentials.
