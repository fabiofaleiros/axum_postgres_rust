use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    PendingReview,
    Completed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "InProgress",
            TaskStatus::PendingReview => "PendingReview",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Pending" => Ok(TaskStatus::Pending),
            "InProgress" => Ok(TaskStatus::InProgress),
            "PendingReview" => Ok(TaskStatus::PendingReview),
            "Completed" => Ok(TaskStatus::Completed),
            "Cancelled" => Ok(TaskStatus::Cancelled),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }

    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
        match (self, target) {
            // From Pending
            (TaskStatus::Pending, TaskStatus::InProgress) => true,
            (TaskStatus::Pending, TaskStatus::Cancelled) => true,
            
            // From InProgress
            (TaskStatus::InProgress, TaskStatus::Completed) => true,
            (TaskStatus::InProgress, TaskStatus::PendingReview) => true,
            (TaskStatus::InProgress, TaskStatus::Cancelled) => true,
            
            // From PendingReview
            (TaskStatus::PendingReview, TaskStatus::Completed) => true,
            (TaskStatus::PendingReview, TaskStatus::Cancelled) => true,
            
            // Cannot transition from Completed or Cancelled
            (TaskStatus::Completed, _) => false,
            (TaskStatus::Cancelled, _) => false,
            
            // No other transitions allowed
            _ => false,
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_serialization() {
        let status = TaskStatus::Pending;
        assert_eq!(status.as_str(), "Pending");
        
        let parsed = TaskStatus::from_str("InProgress").unwrap();
        assert_eq!(parsed, TaskStatus::InProgress);
    }

    #[test]
    fn test_invalid_status_parsing() {
        let result = TaskStatus::from_str("InvalidStatus");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_transitions() {
        let pending = TaskStatus::Pending;
        assert!(pending.can_transition_to(&TaskStatus::InProgress));
        assert!(pending.can_transition_to(&TaskStatus::Cancelled));
        assert!(!pending.can_transition_to(&TaskStatus::Completed));
        
        let in_progress = TaskStatus::InProgress;
        assert!(in_progress.can_transition_to(&TaskStatus::Completed));
        assert!(in_progress.can_transition_to(&TaskStatus::PendingReview));
        assert!(in_progress.can_transition_to(&TaskStatus::Cancelled));
        
        let completed = TaskStatus::Completed;
        assert!(!completed.can_transition_to(&TaskStatus::Cancelled));
        assert!(!completed.can_transition_to(&TaskStatus::InProgress));
    }

    #[test]
    fn test_default_status() {
        let default_status = TaskStatus::default();
        assert_eq!(default_status, TaskStatus::Pending);
    }
}