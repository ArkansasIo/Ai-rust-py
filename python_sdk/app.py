import os
from fastapi import FastAPI
from pydantic import BaseModel
from .agent_client import AgentClient

app = FastAPI(title="Ai-rust-py Agent SDK", version="0.3.0")
client = AgentClient(os.getenv("RUST_API_URL", "http://rust-api:8080"))

class AgentRequest(BaseModel):
    task: str
    agent: str = "general"
    model: str | None = None

@app.get("/health")
def health():
    return {"status": "ok", "service": "agent-sdk"}

@app.post("/run")
async def run(req: AgentRequest):
    return await client.run(req.task, req.agent, req.model)
