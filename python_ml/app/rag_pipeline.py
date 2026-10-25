from .rag import chunk_text
from .vector_store import InMemoryVectorStore, VectorDocument

class RAGPipeline:
    def __init__(self, store: InMemoryVectorStore | None = None):
        self.store = store or InMemoryVectorStore()

    def add_document(self, document_id: str, text: str, embeddings: list[list[float]], metadata: dict | None = None) -> int:
        chunks = chunk_text(text)
        for index, chunk in enumerate(chunks):
            if index >= len(embeddings):
                break
            self.store.upsert(VectorDocument(
                document_id=f"{document_id}:{index}",
                text=chunk,
                embedding=embeddings[index],
                metadata={**(metadata or {}), "chunk": index},
            ))
        return min(len(chunks), len(embeddings))

    def retrieve(self, query_embedding: list[float], top_k: int = 5) -> list[dict]:
        return [
            {"score": score, "text": doc.text, "metadata": doc.metadata}
            for score, doc in self.store.search(query_embedding, top_k)
        ]
