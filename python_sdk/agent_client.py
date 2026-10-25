import os
from dataclasses import dataclass
from typing import Any
import httpx

@dataclass
class AgentClient:
    base_url: str = os.getenv("RUST_API_URL", "http://rust-api:8080")

    async def run(self, task: str, agent: str = "general", model: str | None = None) -> Any:
        payload = {"task": task, "agent": agent}
        if model:
            payload["model"] = model
        async with httpx.AsyncClient(timeout=180) as client:
            response = await client.post(f"{self.base_url.rstrip('/')}/api/v1/agents/run", json=payload)
            response.raise_for_status()
            return response.json()

    async def chat(self, message: str, model: str | None = None) -> Any:
        payload = {"message": message}
        if model:
            payload["model"] = model
        async with httpx.AsyncClient(timeout=180) as client:
            response = await client.post(f"{self.base_url.rstrip('/')}/api/v1/chat", json=payload)
            response.raise_for_status()
            return response.json()
