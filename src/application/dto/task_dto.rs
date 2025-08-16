use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::{Task, TaskId, TaskStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: i32,
    pub name: String,
    pub priority: Option<i32>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskStatusDto {
    pub status: TaskStatus,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWithTransitionsDto {
    pub task: TaskDto,
    pub valid_transitions: Vec<TaskStatus>,
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.value(),
            name: task.name,
            priority: task.priority,
            status: task.status,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}

impl TryFrom<TaskDto> for Task {
    type Error = String;

    fn try_from(dto: TaskDto) -> Result<Self, Self::Error> {
        Task::new_with_status(
            TaskId::new(dto.id), 
            dto.name, 
            dto.priority, 
            dto.status, 
            dto.created_at, 
            dto.updated_at
        )
    }
}

