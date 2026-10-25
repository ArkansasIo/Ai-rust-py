use crate::domain::task::{Task, TaskStatus};

pub struct TaskService;
impl TaskService {
    pub fn can_run(task: &Task) -> bool { matches!(task.status, TaskStatus::Queued) && task.attempts < task.max_attempts }
}
