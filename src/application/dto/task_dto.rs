use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::{Task, TaskId, TaskStatus, StatusHistory, TaskAnalytics};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusHistoryDto {
    pub id: String,
    pub task_id: i32,
    pub from_status: Option<TaskStatus>,
    pub to_status: TaskStatus,
    pub changed_at: DateTime<Utc>,
    pub changed_by: String,
    pub comment: Option<String>,
    pub user_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskHistoryDto {
    pub task_id: i32,
    pub history: Vec<StatusHistoryDto>,
    pub total_time_in_progress: Option<String>, // Duration as human-readable string
    pub number_of_transitions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalyticsDto {
    pub task_id: i32,
    pub total_time_in_progress: Option<String>,
    pub time_to_completion: Option<String>,
    pub number_of_transitions: usize,
    pub was_approved: bool,
    pub approval_time: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionAnalyticsDto {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_completed_tasks: usize,
    pub average_completion_time: Option<String>,
    pub completion_times_by_priority: Vec<PriorityCompletionDto>,
    pub approval_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityCompletionDto {
    pub priority: i32,
    pub average_time: String,
    pub task_count: usize,
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

impl From<StatusHistory> for StatusHistoryDto {
    fn from(history: StatusHistory) -> Self {
        Self {
            id: history.id,
            task_id: history.task_id,
            from_status: history.from_status,
            to_status: history.to_status,
            changed_at: history.changed_at,
            changed_by: history.changed_by,
            comment: history.comment,
            user_role: history.user_role.as_str().to_string(),
        }
    }
}

impl From<TaskAnalytics> for TaskAnalyticsDto {
    fn from(analytics: TaskAnalytics) -> Self {
        Self {
            task_id: analytics.task_id,
            total_time_in_progress: analytics.total_time_in_progress.map(|d| format_duration(d)),
            time_to_completion: analytics.time_to_completion.map(|d| format_duration(d)),
            number_of_transitions: analytics.number_of_transitions,
            was_approved: analytics.was_approved,
            approval_time: analytics.approval_time.map(|d| format_duration(d)),
            created_at: analytics.created_at,
            completed_at: analytics.completed_at,
        }
    }
}

pub fn format_duration(duration: chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

