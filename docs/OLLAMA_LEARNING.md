# Ollama Learning Architecture

Ollama is the local model/inference layer. It is not the entire learning system.

```text
Sensors / input
      |
      v
Rust cognitive state -----> memory / goals / personality
      |
      v
Python ML adapter
      |
      v
Ollama model inference
      |
      v
candidate response / prediction
      |
      v
Rust evaluation + safety + reward
      |
      +----> episodic memory
      +----> training dataset
      +----> metrics / evaluation
```

## Learning modes

- **Inference:** ask the Ollama model for a prediction or response.
- **Feedback learning:** record reward and evaluation metadata.
- **Retrieval learning:** provide relevant memories and knowledge as context.
- **Fine-tuning:** optionally create curated datasets for an external training pipeline.
- **Continual learning:** maintain persistent cognitive state without modifying model weights on every interaction.

The architecture deliberately distinguishes model weights from the agent's persistent memory and identity. This makes experiments reproducible and prevents accidental self-modification of the underlying model.
