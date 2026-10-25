# Operations Runbook

This runbook describes local development operations for Ai-rust-py / Synthetic Brain Architecture. Do not use the sample credentials or development configuration for a public deployment.

## 1. Local startup

1. Copy `.env.example` to `.env`.
2. Replace all development credentials with local-only secrets.
3. Build and start the services:

   ```sh
   docker compose build
   docker compose up -d
   docker compose ps
   ```

4. Pull the configured language model into Ollama:

   ```sh
   docker compose exec ollama ollama pull llama3.2
   ```

5. Check service endpoints:

   ```sh
   curl --fail http://127.0.0.1:8080/health
   curl --fail http://127.0.0.1:8001/health
   curl --fail http://127.0.0.1:11434/api/tags
   ```

If the configured model differs from `llama3.2`, pull the model specified by `OLLAMA_MODEL`.

## 2. Common diagnostics

### Rust API does not start

- Inspect logs: `docker compose logs --tail=200 rust-api`.
- Confirm `DATABASE_URL` points to the Compose hostname `mysql`, not `127.0.0.1`, from inside the container.
- Confirm the migration files exist and the database user can create/read schema objects.
- Check whether the configured port is already occupied.

### MySQL is not ready

- Inspect: `docker compose logs --tail=200 mysql`.
- Wait for MySQL initialization to finish before restarting dependents.
- For a disposable development instance only, inspect container health and volume state before considering data resets. Never remove `mysql_data` if its data must be retained.

### Ollama requests fail

- Inspect: `docker compose logs --tail=200 ollama`.
- Verify the Ollama base URL from service containers is `http://ollama:11434`.
- Confirm the requested model is present with `docker compose exec ollama ollama list`.
- Model downloads require network access and sufficient storage.

### Python ML service fails

- Inspect: `docker compose logs --tail=200 python-ml`.
- Verify `python_ml/requirements.txt` matches the runtime environment.
- Run `python -m compileall -q python_ml python_sdk scripts` from the repository root.
- Confirm the ML service reads the expected environment variables and that the model volume is writable when model artifacts are generated.

## 3. CI checks

Run the same baseline checks locally:

```sh
cargo fmt --manifest-path rust_api/Cargo.toml --check
cargo check --manifest-path rust_api/Cargo.toml
python -m compileall -q python_ml python_sdk scripts
python -c "import json, pathlib; [json.loads(p.read_text()) for p in pathlib.Path('workflows').glob('*.json')]"
```

These are baseline syntax and compile checks; they do not replace integration tests against a running MySQL instance and model service.

## 4. Security checklist before deployment

- Configure authentication and authorization at the API boundary.
- Replace sample database credentials and manage secrets outside source control.
- Restrict CORS to known origins.
- Keep shell execution and network tools disabled unless there is a defined, audited need.
- Require a separate trusted approval mechanism for privileged operations.
- Do not expose MySQL directly to untrusted networks.
- Apply request, context, concurrency and timeout limits.
- Define audit logging and retention rules for prompts, outputs, embeddings and memory.
- Use TLS and a reverse proxy for externally accessible deployments.
- Back up the database and test restore procedures before production use.

## 5. Shutdown

```sh
docker compose down
```

This stops containers but preserves named volumes. Do not use `docker compose down -v` unless deleting the local database and model data is explicitly intended.
