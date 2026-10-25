from .base import BaseAgent

class ResearchAgent(BaseAgent):
    async def run(self, messages):
        return {'mode':'research','response': await self.client.chat(self.spec.model, messages)}
