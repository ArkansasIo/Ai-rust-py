# AI Rust + Python + MySQL

A multi-language AI/ML platform scaffold based on the AI and Machine Learning notes supplied for this project.

## Stack

- **Rust** — API gateway, application orchestration, persistence integration
- **Python** — ML training, inference, evaluation, model registry
- **MySQL 8.4** — datasets, models, training runs, predictions, events
- **Docker Compose** — local multi-service development

## AI/ML scope

- Supervised learning
- Unsupervised learning
- Reinforcement-learning foundation
- Classification and clustering demos
- Model registry
- Training/evaluation pipeline
- Prediction API
- AI agent architecture foundation
- Configuration and database schema

## Architecture

```text
Client
  |
  v
Rust API :8080
  |
  +----> MySQL :3306
  |
  +----> Python ML :8001
              |
              +--> Training
              +--> Inference
              +--> Evaluation
              +--> Model Registry
              +--> RL / Agents
```

## Quick start

```bash
cp .env.example .env
docker compose up --build
```

Rust API: `http://127.0.0.1:8080`
Python ML: `http://127.0.0.1:8001`

Health checks:

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8001/health
```

Demo ML endpoints:

```bash
curl -X POST http://127.0.0.1:8001/demo/supervised
curl -X POST http://127.0.0.1:8001/demo/unsupervised
```

This is a development foundation. Production use should add authentication, authorization, secrets management, database migrations, model artifact signing, observability, rate limiting, input validation, and secure model isolation.
