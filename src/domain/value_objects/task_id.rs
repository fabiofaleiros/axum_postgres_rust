#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(i32);

impl TaskId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl From<i32> for TaskId {
    fn from(id: i32) -> Self {
        Self::new(id)
    }
}

impl From<TaskId> for i32 {
    fn from(task_id: TaskId) -> Self {
        task_id.value()
    }
}