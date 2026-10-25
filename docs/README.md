# Documentation Index

Documentation for the Ai-rust-py repository, also described as Synthetic Brain Architecture.

## Architecture and design

- [Synthetic Brain Architecture](../SYNTHETIC_BRAIN_ARCHITECTURE.md) — system scope, regional software/hardware architecture, and hardware maturity status.
- [Repository Architecture Review](REPOSITORY_REVIEW_2026-10-25.md) — current repository layout, risk priorities, recommended sequencing, and validation boundaries.

## Operations

- [Operations Runbook](OPERATIONS_RUNBOOK.md) — local startup, health checks, diagnostics, CI commands, shutdown, and pre-deployment security checklist.

## Testing

- [Agent Runtime Test Plan](../tests/AGENT_RUNTIME_TEST_PLAN.md) — high-level runtime test plan.
- [Security Policy Test Cases](../tests/SECURITY_POLICY_TEST_CASES.md) — expected security-policy behavior and invariants.

## Source locations

- `rust_api/src` — Rust API, runtime, tools, workflows and service modules.
- `python_ml/app` — Python ML, model, RAG, memory and worker modules.
- `python_sdk` — Python client package.
- `database` — schema and migrations.
- `workflows` — declarative workflow definitions.
- `.github/workflows` — CI definitions.

## Accuracy note

Architecture documentation describes intent and design constraints. A feature is considered implemented only after code exists, tests exercise it, and the relevant build or integration checks pass. Hardware documentation is not a substitute for validated schematics, board layouts or manufacturing review.
