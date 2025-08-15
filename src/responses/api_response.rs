use serde::{Serialize, Deserialize};
use crate::application::dto::TaskDto;

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

/// Response structure for task lists
#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskDto>,
}

/// Response structure for task creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCreatedResponse {
    pub task_id: i32,
    pub message: String,
}

