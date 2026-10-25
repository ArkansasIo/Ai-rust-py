# Agent Platform

The repository now contains the foundation for a durable agent runtime.

## Agent roles

- `general` — general-purpose reasoning
- `researcher` — evidence and research analysis
- `coder` — software engineering tasks
- `reviewer` — correctness and security review
- `synthesizer` — combines multi-agent results

## Runtime layers

```text
API -> Agent Registry -> Workflow State -> Tool Registry -> Ollama
                         |                    |
                         v                    v
                     Checkpoints          Approval Gate
                         |
                         v
                       MySQL
```

## Security defaults

Shell execution is disabled by default. Network tools are disabled by default. Write and commit operations require explicit approval. Tool risk is recorded for auditing.

## Next layers

The planned runtime extensions are streaming generation, persistent checkpoints, isolated workspaces, Git/GitHub adapters, background task queues, vector retrieval, authentication, and human-in-the-loop approval APIs.
