from sklearn.datasets import load_iris
from sklearn.cluster import KMeans
from sklearn.metrics import silhouette_score

def demo_clustering():
    X, _ = load_iris(return_X_y=True)
    model = KMeans(n_clusters=3, random_state=42, n_init=10)
    labels = model.fit_predict(X)
    return {
        "task": "clustering",
        "algorithm": "KMeans",
        "clusters": int(len(set(labels))),
        "silhouette_score": float(silhouette_score(X, labels)),
    }
