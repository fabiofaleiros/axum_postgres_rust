use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a task record from the database
#[derive(Debug, Serialize, FromRow)]
pub struct Task {
    pub task_id: i32,
    pub name: String,
    pub priority: Option<i32>,
}

/// Request payload for creating a new task
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub priority: Option<i32>,
}

/// Request payload for updating an existing task
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub priority: Option<i32>,
}

/// Response when creating a task (returns the new task_id)
#[derive(Debug, Serialize, FromRow)]
pub struct CreateTaskResponse {
    pub task_id: i32,
}

impl Task {
    /// Validates task data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        if let Some(priority) = self.priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        Ok(())
    }
}

impl CreateTaskRequest {
    /// Validates create request data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        if let Some(priority) = self.priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        Ok(())
    }
}