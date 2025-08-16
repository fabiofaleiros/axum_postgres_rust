use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::{TaskStatus, UserRole};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusHistory {
    pub id: String,
    pub task_id: i32,
    pub from_status: Option<TaskStatus>,
    pub to_status: TaskStatus,
    pub changed_at: DateTime<Utc>,
    pub changed_by: String,
    pub comment: Option<String>,
    pub user_role: UserRole,
}

impl StatusHistory {
    pub fn new(
        id: String,
        task_id: i32,
        from_status: Option<TaskStatus>,
        to_status: TaskStatus,
        changed_at: DateTime<Utc>,
        changed_by: String,
        comment: Option<String>,
        user_role: UserRole,
    ) -> Self {
        Self {
            id,
            task_id,
            from_status,
            to_status,
            changed_at,
            changed_by,
            comment,
            user_role,
        }
    }

    pub fn is_initial_creation(&self) -> bool {
        self.from_status.is_none()
    }

    pub fn is_completion(&self) -> bool {
        self.to_status == TaskStatus::Completed
    }

    pub fn is_cancellation(&self) -> bool {
        self.to_status == TaskStatus::Cancelled
    }

    pub fn is_approval(&self) -> bool {
        matches!(
            (&self.from_status, &self.to_status),
            (Some(TaskStatus::PendingReview), TaskStatus::Completed)
        )
    }

    pub fn duration_from_previous(&self, previous: &StatusHistory) -> Option<chrono::Duration> {
        if previous.task_id == self.task_id && previous.changed_at <= self.changed_at {
            Some(self.changed_at - previous.changed_at)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalytics {
    pub task_id: i32,
    pub total_time_in_progress: Option<chrono::Duration>,
    pub time_to_completion: Option<chrono::Duration>,
    pub number_of_transitions: usize,
    pub was_approved: bool,
    pub approval_time: Option<chrono::Duration>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl TaskAnalytics {
    pub fn from_history(history: Vec<StatusHistory>) -> Option<Self> {
        if history.is_empty() {
            return None;
        }

        let task_id = history[0].task_id;
        let creation_entry = history.iter().find(|h| h.is_initial_creation())?;
        let created_at = creation_entry.changed_at;

        let mut total_time_in_progress = chrono::Duration::zero();
        let mut time_to_completion = None;
        let mut was_approved = false;
        let mut approval_time = None;
        let mut completed_at = None;

        // Calculate time spent in InProgress status
        let mut in_progress_start: Option<DateTime<Utc>> = None;
        let mut pending_review_start: Option<DateTime<Utc>> = None;

        for entry in &history {
            match entry.to_status {
                TaskStatus::InProgress => {
                    in_progress_start = Some(entry.changed_at);
                }
                TaskStatus::PendingReview => {
                    if let Some(start) = in_progress_start {
                        total_time_in_progress = total_time_in_progress + (entry.changed_at - start);
                    }
                    pending_review_start = Some(entry.changed_at);
                }
                TaskStatus::Completed => {
                    if let Some(start) = in_progress_start {
                        total_time_in_progress = total_time_in_progress + (entry.changed_at - start);
                    }
                    
                    if entry.is_approval() {
                        was_approved = true;
                        if let Some(review_start) = pending_review_start {
                            approval_time = Some(entry.changed_at - review_start);
                        }
                    }
                    
                    completed_at = Some(entry.changed_at);
                    time_to_completion = Some(entry.changed_at - created_at);
                    break;
                }
                TaskStatus::Cancelled => {
                    completed_at = Some(entry.changed_at);
                    break;
                }
                _ => {}
            }
        }

        Some(TaskAnalytics {
            task_id,
            total_time_in_progress: if total_time_in_progress.is_zero() { None } else { Some(total_time_in_progress) },
            time_to_completion,
            number_of_transitions: history.len(),
            was_approved,
            approval_time,
            created_at,
            completed_at,
        })
    }
}