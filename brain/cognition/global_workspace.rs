#[derive(Debug, Clone, Default)]
pub struct WorkspaceItem {
    pub source: String,
    pub content: String,
    pub salience: f32,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct GlobalWorkspace {
    items: Vec<WorkspaceItem>,
    capacity: usize,
}

impl GlobalWorkspace {
    pub fn new(capacity: usize) -> Self { Self { items: Vec::new(), capacity: capacity.max(1) } }

    pub fn publish(&mut self, item: WorkspaceItem) {
        self.items.push(item);
        self.items.sort_by(|a, b| b.salience.total_cmp(&a.salience));
        self.items.truncate(self.capacity);
    }

    pub fn active(&self) -> &[WorkspaceItem] { &self.items }
}
