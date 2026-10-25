# Agent Runtime

The runtime is designed around explicit agents, workflows, tools, policy, and durable checkpoints.

```text
AgentRuntime
├── AgentRegistry
├── WorkflowState
├── ToolRegistry
├── SecurityPolicy
├── ApprovalGate
└── Ollama
```

No privileged tool should execute solely because an LLM requested it. Tool authorization is a separate policy decision.
