from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Any
from .ollama import OllamaClient
from .rag import chunk_text, cosine_similarity
import os

app = FastAPI(title="Ai-rust-py ML Service", version="0.2.0")
ollama = OllamaClient(os.getenv("OLLAMA_URL", "http://ollama:11434"))

class PredictRequest(BaseModel):
    model: str
    input: Any
class ChatRequest(BaseModel):
    message: str
    model: str | None = None
class EmbedRequest(BaseModel):
    input: str
    model: str = "nomic-embed-text"

@app.get("/health")
def health(): return {"service":"python-ml","status":"ok"}
@app.get("/info")
def info(): return {"version":"0.2.0","features":["supervised","unsupervised","rl-foundation","ollama","rag"]}
@app.post("/predict")
def predict(req: PredictRequest): return {"model":req.model,"prediction":{"echo":req.input}}
@app.post("/chat")
async def chat(req: ChatRequest):
    try: return await ollama.chat(req.model or os.getenv("OLLAMA_MODEL","llama3.2"), req.message)
    except Exception as e: raise HTTPException(502, str(e))
@app.post("/embed")
async def embed(req: EmbedRequest):
    try: return await ollama.embed(req.model, req.input)
    except Exception as e: raise HTTPException(502, str(e))
@app.post("/rag/chunk")
def rag_chunk(text: str): return {"chunks":chunk_text(text)}
