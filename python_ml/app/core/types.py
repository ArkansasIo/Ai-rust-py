from enum import Enum
from pydantic import BaseModel, Field
from typing import Any

class AgentClass(str, Enum):
    ASSISTANT='assistant'; RESEARCHER='researcher'; CODER='coder'; ANALYST='analyst'; PLANNER='planner'; EXECUTOR='executor'; CUSTOM='custom'
class MessageRole(str, Enum): SYSTEM='system'; USER='user'; ASSISTANT='assistant'; TOOL='tool'
class MemoryType(str, Enum): SEMANTIC='semantic'; EPISODIC='episodic'; PROCEDURAL='procedural'; PREFERENCE='preference'; SUMMARY='summary'
class ToolRisk(str, Enum): READ_ONLY='read_only'; LOW='low'; MEDIUM='medium'; HIGH='high'; CRITICAL='critical'

class ChatMessage(BaseModel):
    role: MessageRole
    content: str

class AgentSpec(BaseModel):
    name: str
    agent_class: AgentClass = AgentClass.ASSISTANT
    model: str
    system_prompt: str = ''
    max_steps: int = Field(default=8, ge=1, le=128)

class ToolSpec(BaseModel):
    name: str
    description: str
    risk: ToolRisk = ToolRisk.READ_ONLY
    input_schema: dict[str, Any] = Field(default_factory=dict)

class MemoryRecord(BaseModel):
    content: str
    memory_type: MemoryType = MemoryType.SEMANTIC
    importance: float = Field(default=0.5, ge=0, le=1)
    metadata: dict[str, Any] = Field(default_factory=dict)
