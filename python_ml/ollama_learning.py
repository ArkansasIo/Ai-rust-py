"""Ollama-backed learning/inference adapter.

This module keeps model interaction separate from the Rust cognitive runtime.
It provides generation, evaluation hooks, and a small continual-learning
interface without pretending that text generation alone is human learning.
"""

from dataclasses import dataclass, field
from typing import Any
import json
import urllib.request


@dataclass
class LearningSample:
    prompt: str
    response: str
    reward: float = 0.0
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass
class OllamaConfig:
    base_url: str = "http://127.0.0.1:11434"
    model: str = "llama3.2"
    temperature: float = 0.7


class OllamaLearningModel:
    def __init__(self, config: OllamaConfig | None = None) -> None:
        self.config = config or OllamaConfig()
        self.samples: list[LearningSample] = []

    def generate(self, prompt: str, system: str | None = None) -> str:
        payload: dict[str, Any] = {
            "model": self.config.model,
            "prompt": prompt,
            "stream": False,
            "options": {"temperature": self.config.temperature},
        }
        if system:
            payload["system"] = system
        request = urllib.request.Request(
            f"{self.config.base_url.rstrip('/')}/api/generate",
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        with urllib.request.urlopen(request, timeout=120) as response:
            data = json.loads(response.read().decode("utf-8"))
        return str(data.get("response", ""))

    def learn_from_feedback(self, sample: LearningSample) -> None:
        self.samples.append(sample)

    def best_examples(self, limit: int = 10) -> list[LearningSample]:
        return sorted(self.samples, key=lambda item: item.reward, reverse=True)[:limit]

    def build_context(self, limit: int = 5) -> str:
        examples = self.best_examples(limit)
        return "\n\n".join(
            f"Prompt: {item.prompt}\nResponse: {item.response}\nReward: {item.reward}"
            for item in examples
        )
