class MemorySummarizer:
    def summarize(self, messages: list[dict], max_items: int = 12) -> str:
        recent = messages[-max_items:]
        return '\n'.join(f"{m.get('role','unknown')}: {m.get('content','')}" for m in recent)
