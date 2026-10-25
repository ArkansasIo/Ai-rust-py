from ..core.types import MemoryRecord

class MemoryStore:
    def __init__(self): self._records: list[MemoryRecord] = []
    def add(self, record: MemoryRecord) -> MemoryRecord: self._records.append(record); return record
    def all(self) -> list[MemoryRecord]: return list(self._records)
    def clear(self) -> None: self._records.clear()
