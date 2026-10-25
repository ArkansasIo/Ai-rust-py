from dataclasses import dataclass
import math

@dataclass
class VectorDocument:
    document_id: str
    text: str
    embedding: list[float]
    metadata: dict

class InMemoryVectorStore:
    def __init__(self) -> None:
        self.documents: list[VectorDocument] = []

    def upsert(self, document: VectorDocument) -> None:
        self.documents = [d for d in self.documents if d.document_id != document.document_id]
        self.documents.append(document)

    def search(self, query: list[float], top_k: int = 5) -> list[tuple[float, VectorDocument]]:
        scored = [(self._cosine(query, d.embedding), d) for d in self.documents]
        return sorted(scored, key=lambda item: item[0], reverse=True)[:top_k]

    @staticmethod
    def _cosine(a: list[float], b: list[float]) -> float:
        if not a or not b or len(a) != len(b):
            return 0.0
        dot = sum(x * y for x, y in zip(a, b))
        na = math.sqrt(sum(x * x for x in a))
        nb = math.sqrt(sum(y * y for y in b))
        return dot / (na * nb) if na and nb else 0.0
