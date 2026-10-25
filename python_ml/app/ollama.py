import httpx
class OllamaClient:
    def __init__(self, base_url: str): self.base_url=base_url.rstrip('/')
    async def chat(self, model: str, message: str):
        async with httpx.AsyncClient(timeout=120) as c:
            r=await c.post(f"{self.base_url}/api/chat",json={"model":model,"stream":False,"messages":[{"role":"user","content":message}]}); r.raise_for_status(); return r.json()
    async def embed(self, model: str, text: str):
        async with httpx.AsyncClient(timeout=120) as c:
            r=await c.post(f"{self.base_url}/api/embed",json={"model":model,"input":text}); r.raise_for_status(); return r.json()
