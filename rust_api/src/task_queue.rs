use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub kind: String,
    pub payload: String,
}

#[derive(Clone, Default)]
pub struct TaskQueue {
    inner: Arc<Mutex<VecDeque<Task>>>,
}

impl TaskQueue {
    pub fn new() -> Self { Self::default() }

    pub async fn push(&self, task: Task) {
        self.inner.lock().await.push_back(task);
    }

    pub async fn pop(&self) -> Option<Task> {
        self.inner.lock().await.pop_front()
    }

    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }
}
