use axum_postgres_rust::domain::TaskStatus;

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
    fn test_all_status_variants_serialization() {
        assert_eq!(TaskStatus::Pending.as_str(), "Pending");
        assert_eq!(TaskStatus::InProgress.as_str(), "InProgress");
        assert_eq!(TaskStatus::PendingReview.as_str(), "PendingReview");
        assert_eq!(TaskStatus::Completed.as_str(), "Completed");
        assert_eq!(TaskStatus::Cancelled.as_str(), "Cancelled");
    }

    #[test]
    fn test_all_status_variants_parsing() {
        assert_eq!(TaskStatus::from_str("Pending").unwrap(), TaskStatus::Pending);
        assert_eq!(TaskStatus::from_str("InProgress").unwrap(), TaskStatus::InProgress);
        assert_eq!(TaskStatus::from_str("PendingReview").unwrap(), TaskStatus::PendingReview);
        assert_eq!(TaskStatus::from_str("Completed").unwrap(), TaskStatus::Completed);
        assert_eq!(TaskStatus::from_str("Cancelled").unwrap(), TaskStatus::Cancelled);
    }

    #[test]
    fn test_invalid_status_parsing() {
        let result = TaskStatus::from_str("InvalidStatus");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid task status"));
    }

    #[test]
    fn test_case_sensitive_parsing() {
        let result = TaskStatus::from_str("pending");
        assert!(result.is_err());
        
        let result = TaskStatus::from_str("PENDING");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_transitions_from_pending() {
        let pending = TaskStatus::Pending;
        assert!(pending.can_transition_to(&TaskStatus::InProgress));
        assert!(pending.can_transition_to(&TaskStatus::Cancelled));
        assert!(!pending.can_transition_to(&TaskStatus::Completed));
        assert!(!pending.can_transition_to(&TaskStatus::PendingReview));
    }

    #[test]
    fn test_valid_transitions_from_in_progress() {
        let in_progress = TaskStatus::InProgress;
        assert!(in_progress.can_transition_to(&TaskStatus::Completed));
        assert!(in_progress.can_transition_to(&TaskStatus::PendingReview));
        assert!(in_progress.can_transition_to(&TaskStatus::Cancelled));
        assert!(!in_progress.can_transition_to(&TaskStatus::Pending));
    }

    #[test]
    fn test_valid_transitions_from_pending_review() {
        let pending_review = TaskStatus::PendingReview;
        assert!(pending_review.can_transition_to(&TaskStatus::Completed));
        assert!(pending_review.can_transition_to(&TaskStatus::Cancelled));
        assert!(!pending_review.can_transition_to(&TaskStatus::Pending));
        assert!(!pending_review.can_transition_to(&TaskStatus::InProgress));
    }

    #[test]
    fn test_no_transitions_from_completed() {
        let completed = TaskStatus::Completed;
        assert!(!completed.can_transition_to(&TaskStatus::Pending));
        assert!(!completed.can_transition_to(&TaskStatus::InProgress));
        assert!(!completed.can_transition_to(&TaskStatus::PendingReview));
        assert!(!completed.can_transition_to(&TaskStatus::Cancelled));
        assert!(!completed.can_transition_to(&TaskStatus::Completed));
    }

    #[test]
    fn test_no_transitions_from_cancelled() {
        let cancelled = TaskStatus::Cancelled;
        assert!(!cancelled.can_transition_to(&TaskStatus::Pending));
        assert!(!cancelled.can_transition_to(&TaskStatus::InProgress));
        assert!(!cancelled.can_transition_to(&TaskStatus::PendingReview));
        assert!(!cancelled.can_transition_to(&TaskStatus::Completed));
        assert!(!cancelled.can_transition_to(&TaskStatus::Cancelled));
    }

    #[test]
    fn test_default_status() {
        let default_status = TaskStatus::default();
        assert_eq!(default_status, TaskStatus::Pending);
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(TaskStatus::Pending, TaskStatus::Pending);
        assert_ne!(TaskStatus::Pending, TaskStatus::InProgress);
    }

    #[test]
    fn test_status_clone() {
        let status = TaskStatus::InProgress;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }
}