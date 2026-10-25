# Repository Architecture Review — 2026-10-25

## Current architecture

The repository currently contains a Rust/Axum API gateway, a Python ML service, Ollama integration, MySQL persistence, workflow/task modules, a Python SDK, and a synthetic-brain hardware architecture draft.

Primary entry points:
- `rust_api/src/main.rs` — gateway routes, database initialization, migrations, middleware and server startup.
- `rust_api/src/config.rs` — environment-based service configuration.
- `rust_api/src/security.rs` — default-deny shell/network flags and approval gating for writes.
- `python_ml/app/main.py` — Python ML HTTP service.
- `database/schema.sql` and `database/migrations/` — persistence schema and migrations.
- `.github/workflows/ci.yml` — Rust formatting/compile, Python syntax and workflow JSON checks.
- `docker-compose.yml` — local multi-service setup.

## Priority engineering findings

### P0 — Validate configuration and avoid unsafe production defaults
1. Fail fast on invalid port and connection-pool settings rather than silently falling back after parse failures.
2. Require non-placeholder database passwords outside development.
3. Restrict CORS to configured origins for deployed environments; permissive CORS should be development-only.
4. Add a readiness endpoint that checks MySQL and Ollama independently of liveness.

### P1 — Improve service startup and availability
1. Add health checks to Docker Compose for MySQL, Ollama, Python ML and Rust API.
2. Use bounded retries/backoff for database startup and dependency health checks.
3. Add request correlation IDs, structured error responses, and explicit HTTP status checking for Ollama requests.
4. Ensure Python ML health and request routes have bounded timeouts and validate request payload sizes.

### P1 — Test the integration boundaries
1. Add Rust unit tests for security policy behavior, including shell/network denial and approval-required write/commit actions.
2. Add API tests for agent, tool authorization, workflow and task endpoints.
3. Add Python unit tests for request validation, model service error handling, RAG chunking/vector-store contracts and health responses.
4. Add a CI service-integration job using disposable MySQL and a mocked Ollama endpoint where feasible.

### P2 — Operational visibility and durable background jobs
1. Add metrics for request latency, model latency, task queue depth, failures and retry counts.
2. Record workflow/task transitions with timestamps and correlation IDs.
3. Make task claiming atomic and idempotent before using multiple workers.
4. Add retention policies for model prompts, conversation memory, predictions and event records.

### P2 — Security and data governance
1. Keep model output separate from authorization decisions; never let the model approve its own privileged action.
2. Treat tool arguments, retrieved documents and model outputs as untrusted input.
3. Add authentication and role-based access before exposing agent or authorization endpoints beyond a trusted local network.
4. Define data retention, deletion, audit and secret-rotation policies before production use.

## Hardware architecture boundary

The Synthetic Brain Architecture documentation correctly identifies its hardware files as engineering drafts, not manufacturing-qualified PCB layouts. Before fabrication, the hardware project needs selected components, power and thermal budgets, signal-integrity review, connector pinouts, safety analysis and validated board layouts. Software region names should not be interpreted as evidence of biological equivalence.

## Suggested implementation order

1. Configuration validation and safe CORS defaults.
2. Health/readiness checks and dependency-aware Compose health checks.
3. Unit and integration tests for security policy and the service boundary.
4. Authentication, auditability, observability and operational retention.
5. Task-worker concurrency and reliability.
6. Hardware component selection and manufacturing review.

## Validation status

This review was created from repository metadata and source-file inspection on 2026-10-25. It is a code-review document, not a claim that the repository has been built, deployed, or hardware-validated in this environment. Run the repository CI workflow and local Compose smoke tests before release.
