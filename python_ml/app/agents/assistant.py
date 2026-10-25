from .base import BaseAgent

class AssistantAgent(BaseAgent):
    async def run(self, messages):
        return await self.client.chat(self.spec.model, messages)
