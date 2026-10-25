from .base import BaseAgent

class PlannerAgent(BaseAgent):
    async def run(self, messages):
        return {'mode':'planning','response': await self.client.chat(self.spec.model, messages)}
