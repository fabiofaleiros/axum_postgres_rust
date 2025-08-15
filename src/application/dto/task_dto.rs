use serde::{Deserialize, Serialize};
use crate::domain::{Task, TaskId};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: i32,
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub priority: Option<i32>,
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.value(),
            name: task.name,
            priority: task.priority,
        }
    }
}

impl TryFrom<TaskDto> for Task {
    type Error = String;

    fn try_from(dto: TaskDto) -> Result<Self, Self::Error> {
        Task::new(TaskId::new(dto.id), dto.name, dto.priority)
    }
}