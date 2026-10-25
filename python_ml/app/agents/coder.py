from .base import BaseAgent

class CoderAgent(BaseAgent):
    async def run(self, messages):
        return {'mode':'coding','response': await self.client.chat(self.spec.model, messages)}
