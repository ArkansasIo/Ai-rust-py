from typing import Any
from fastapi import FastAPI
from pydantic import BaseModel
from .models import ModelRegistry
from .ml.supervised import demo_classification
from .ml.unsupervised import demo_clustering

app = FastAPI(title="AI/ML Python Service", version="0.1.0")
registry = ModelRegistry()

class PredictRequest(BaseModel):
    model: str
    input: Any

@app.get("/health")
def health():
    return {"service": "python-ml", "status": "ok"}

@app.get("/models")
def models():
    return {"models": registry.list_models()}

@app.post("/predict")
def predict(request: PredictRequest):
    return {
        "model": request.model,
        "prediction": {"class": "demo", "confidence": 0.5},
        "input": request.input,
    }

@app.post("/demo/supervised")
def supervised_demo():
    return demo_classification()

@app.post("/demo/unsupervised")
def unsupervised_demo():
    return demo_clustering()
