import numpy as np

def chunk_text(text: str, size: int = 800, overlap: int = 100):
    if overlap >= size: raise ValueError("overlap must be smaller than size")
    out=[]; start=0
    while start < len(text):
        out.append(text[start:start+size]); start += size-overlap
    return out

def cosine_similarity(a, b):
    x=np.asarray(a,dtype=float); y=np.asarray(b,dtype=float)
    denom=np.linalg.norm(x)*np.linalg.norm(y)
    return float(np.dot(x,y)/denom) if denom else 0.0
