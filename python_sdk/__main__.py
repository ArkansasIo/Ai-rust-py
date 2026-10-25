from .agent_client import AgentClient

async def run_agent(task: str, agent: str = "general", model: str | None = None):
    return await AgentClient().run(task, agent, model)
