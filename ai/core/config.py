import json
from pathlib import Path

def load_config(path="ai/config/default.json"):
    return json.loads(Path(path).read_text(encoding="utf-8"))
