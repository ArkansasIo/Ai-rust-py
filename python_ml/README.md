# Python AI/ML Layer

The Python layer contains high-level model experimentation and Ollama integration. It is intentionally separate from the Rust cognitive runtime.

## Ollama

Run Ollama locally, pull a model, then use `OllamaLearningModel` from `ollama_learning.py`.

This layer supports generation and feedback storage for experiments. Feedback is not automatically treated as permanent model training; durable learning requires an explicit training/fine-tuning pipeline.
