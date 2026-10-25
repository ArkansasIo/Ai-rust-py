from abc import ABC, abstractmethod
from ..core.types import AgentSpec, ChatMessage

class BaseAgent(ABC):
    def __init__(self, spec: AgentSpec, client): self.spec, self.client = spec, client
    @abstractmethod
    async def run(self, messages: list[ChatMessage]) -> dict: ...

    def system_message(self) -> ChatMessage:
        return ChatMessage(role='system', content=self.spec.system_prompt)
