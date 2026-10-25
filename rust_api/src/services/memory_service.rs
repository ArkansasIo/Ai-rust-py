use crate::domain::memory::{MemoryQuery, MemoryRecord};

pub struct MemoryService;
impl MemoryService {
    pub fn validate_query(query: &MemoryQuery) -> Result<(), String> {
        if query.query.trim().is_empty() { return Err("memory query is empty".into()); }
        if query.top_k == 0 || query.top_k > 100 { return Err("top_k must be between 1 and 100".into()); }
        Ok(())
    }
    pub fn rank(records: &mut [MemoryRecord]) { records.sort_by(|a,b| b.importance.partial_cmp(&a.importance).unwrap_or(std::cmp::Ordering::Equal)); }
}
