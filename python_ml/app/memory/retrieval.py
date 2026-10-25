from ..core.types import MemoryRecord

class MemoryRetriever:
    def search(self, records: list[MemoryRecord], query: str, top_k: int = 5) -> list[MemoryRecord]:
        terms = set(query.lower().split())
        scored = []
        for record in records:
            overlap = len(terms.intersection(set(record.content.lower().split())))
            scored.append((overlap + record.importance, record))
        scored.sort(key=lambda item: item[0], reverse=True)
        return [record for _, record in scored[:top_k]]
