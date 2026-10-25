from dataclasses import dataclass, asdict
from pathlib import Path

@dataclass
class ModelInfo:
    name: str
    model_type: str
    framework: str
    version: str
    artifact_path: str | None = None

class ModelRegistry:
    def __init__(self, directory: str = "./models"):
        self.directory = Path(directory)
        self.directory.mkdir(parents=True, exist_ok=True)
        self._models: dict[str, ModelInfo] = {}

    def register(self, info: ModelInfo) -> None:
        self._models[info.name] = info

    def list_models(self):
        return [asdict(m) for m in self._models.values()]
