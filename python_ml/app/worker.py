import asyncio
import os

from .ollama import OllamaClient

class AgentWorker:
    def __init__(self) -> None:
        self.ollama = OllamaClient(os.getenv("OLLAMA_URL", "http://ollama:11434"))
        self.model = os.getenv("OLLAMA_MODEL", "llama3.2")

    async def execute(self, task: dict) -> dict:
        kind = task.get("kind", "general")
        prompt = task.get("prompt", task.get("payload", ""))
        response = await self.ollama.chat(self.model, f"Agent task type: {kind}\n\n{prompt}")
        return {"status": "success", "kind": kind, "result": response}

async def worker_loop() -> None:
    worker = AgentWorker()
    while True:
        await asyncio.sleep(1)
        # Queue broker integration is intentionally externalized.
        # This loop provides the lifecycle boundary for Redis/NATS/RabbitMQ workers.
        _ = worker

if __name__ == "__main__":
    asyncio.run(worker_loop())
