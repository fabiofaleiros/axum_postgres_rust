use axum_postgres_rust::domain::{Task, TaskId, TaskStatus};
use chrono::Utc;

#[allow(dead_code)]
fn create_test_task(id: i32, name: &str, priority: Option<i32>) -> Task {
    Task::new(TaskId::new(id), name.to_string(), priority).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_new_valid() {
        let task_id = TaskId::new(1);
        let task = Task::new(task_id, "Valid task".to_string(), Some(5));
        
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(task.id.value(), 1);
        assert_eq!(task.name, "Valid task");
        assert_eq!(task.priority, Some(5));
    }

    #[test]
    fn test_task_new_without_priority() {
        let task_id = TaskId::new(1);
        let task = Task::new(task_id, "Valid task".to_string(), None);
        
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(task.priority, None);
    }

    #[test]
    fn test_task_new_trims_whitespace() {
        let task_id = TaskId::new(1);
        let task = Task::new(task_id, "  Whitespace task  ".to_string(), Some(3));
        
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(task.name, "Whitespace task");
    }

    #[test]
    fn test_task_new_empty_name_fails() {
        let task_id = TaskId::new(1);
        let result = Task::new(task_id, "".to_string(), Some(5));
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_task_new_whitespace_only_name_fails() {
        let task_id = TaskId::new(1);
        let result = Task::new(task_id, "   ".to_string(), Some(5));
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_task_new_priority_too_low_fails() {
        let task_id = TaskId::new(1);
        let result = Task::new(task_id, "Valid task".to_string(), Some(0));
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_task_new_priority_too_high_fails() {
        let task_id = TaskId::new(1);
        let result = Task::new(task_id, "Valid task".to_string(), Some(11));
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_task_new_boundary_priorities_succeed() {
        let task_id = TaskId::new(1);
        
        let task_min = Task::new(task_id, "Min priority task".to_string(), Some(1));
        assert!(task_min.is_ok());
        assert_eq!(task_min.unwrap().priority, Some(1));
        
        let task_max = Task::new(task_id, "Max priority task".to_string(), Some(10));
        assert!(task_max.is_ok());
        assert_eq!(task_max.unwrap().priority, Some(10));
    }

    #[test]
    fn test_update_name_valid() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Original name".to_string(), Some(5)).unwrap();
        
        let result = task.update_name("Updated name".to_string());
        assert!(result.is_ok());
        assert_eq!(task.name, "Updated name");
    }

    #[test]
    fn test_update_name_trims_whitespace() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Original name".to_string(), Some(5)).unwrap();
        
        let result = task.update_name("  Updated name  ".to_string());
        assert!(result.is_ok());
        assert_eq!(task.name, "Updated name");
    }

    #[test]
    fn test_update_name_empty_fails() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Original name".to_string(), Some(5)).unwrap();
        
        let result = task.update_name("".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
        assert_eq!(task.name, "Original name"); // Name should remain unchanged
    }

    #[test]
    fn test_update_name_whitespace_only_fails() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Original name".to_string(), Some(5)).unwrap();
        
        let result = task.update_name("   ".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
        assert_eq!(task.name, "Original name"); // Name should remain unchanged
    }

    #[test]
    fn test_update_priority_valid() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        
        let result = task.update_priority(Some(8));
        assert!(result.is_ok());
        assert_eq!(task.priority, Some(8));
    }

    #[test]
    fn test_update_priority_to_none() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        
        let result = task.update_priority(None);
        assert!(result.is_ok());
        assert_eq!(task.priority, None);
    }

    #[test]
    fn test_update_priority_too_low_fails() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        
        let result = task.update_priority(Some(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
        assert_eq!(task.priority, Some(5)); // Priority should remain unchanged
    }

    #[test]
    fn test_update_priority_too_high_fails() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        
        let result = task.update_priority(Some(11));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
        assert_eq!(task.priority, Some(5)); // Priority should remain unchanged
    }

    #[test]
    fn test_update_priority_boundary_values() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        
        let result_min = task.update_priority(Some(1));
        assert!(result_min.is_ok());
        assert_eq!(task.priority, Some(1));
        
        let result_max = task.update_priority(Some(10));
        assert!(result_max.is_ok());
        assert_eq!(task.priority, Some(10));
    }

    #[test]
    fn test_task_clone_and_equality() {
        let task_id = TaskId::new(1);
        let task1 = Task::new(task_id, "Test task".to_string(), Some(5)).unwrap();
        let task2 = task1.clone();
        
        assert_eq!(task1, task2);
        assert_eq!(task1.id, task2.id);
        assert_eq!(task1.name, task2.name);
        assert_eq!(task1.priority, task2.priority);
    }

    #[test]
    fn test_task_debug_trait() {
        let task_id = TaskId::new(1);
        let task = Task::new(task_id, "Debug task".to_string(), Some(7)).unwrap();
        
        let debug_output = format!("{:?}", task);
        assert!(debug_output.contains("Task"));
        assert!(debug_output.contains("Debug task"));
        assert!(debug_output.contains("7"));
    }

    // ===== STATUS TRANSITION TESTS =====

    #[test]
    fn test_new_task_has_pending_status() {
        let task_id = TaskId::new(1);
        let task = Task::new(task_id, "Test Task".to_string(), Some(5)).unwrap();
        
        assert_eq!(*task.status(), TaskStatus::Pending);
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.priority, Some(5));
    }

    #[test]
    fn test_start_progress_from_pending() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Test Task".to_string(), Some(5)).unwrap();
        
        let result = task.start_progress();
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::InProgress);
    }

    #[test]
    fn test_cannot_start_progress_from_completed() {
        let task_id = TaskId::new(1);
        let mut task = Task::new_with_status(
            task_id, 
            "Test Task".to_string(), 
            Some(5), 
            TaskStatus::Completed, 
            Utc::now(), 
            Utc::now()
        ).unwrap();
        
        let result = task.start_progress();
        assert!(result.is_err());
        assert_eq!(*task.status(), TaskStatus::Completed);
    }

    #[test]
    fn test_low_priority_task_completion() {
        let task_id = TaskId::new(1);
        let mut task = Task::new_with_status(
            task_id, 
            "Test Task".to_string(), 
            Some(5), 
            TaskStatus::InProgress, 
            Utc::now(), 
            Utc::now()
        ).unwrap();
        
        let result = task.complete();
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::Completed);
    }

    #[test]
    fn test_high_priority_task_requires_review() {
        let task_id = TaskId::new(1);
        let mut task = Task::new_with_status(
            task_id, 
            "Test Task".to_string(), 
            Some(2), // High priority
            TaskStatus::InProgress, 
            Utc::now(), 
            Utc::now()
        ).unwrap();
        
        let result = task.complete();
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::PendingReview);
    }

    #[test]
    fn test_approve_completion() {
        let task_id = TaskId::new(1);
        let mut task = Task::new_with_status(
            task_id, 
            "Test Task".to_string(), 
            Some(2), 
            TaskStatus::PendingReview, 
            Utc::now(), 
            Utc::now()
        ).unwrap();
        
        let result = task.approve_completion();
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::Completed);
    }

    #[test]
    fn test_cancel_task() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Test Task".to_string(), Some(5)).unwrap();
        
        let result = task.cancel();
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::Cancelled);
    }

    #[test]
    fn test_cannot_cancel_completed_task() {
        let task_id = TaskId::new(1);
        let mut task = Task::new_with_status(
            task_id, 
            "Test Task".to_string(), 
            Some(5), 
            TaskStatus::Completed, 
            Utc::now(), 
            Utc::now()
        ).unwrap();
        
        let result = task.cancel();
        assert!(result.is_err());
        assert_eq!(*task.status(), TaskStatus::Completed);
    }

    #[test]
    fn test_is_high_priority() {
        let task_id = TaskId::new(1);
        let high_priority_task = Task::new(task_id, "High Priority".to_string(), Some(1)).unwrap();
        let low_priority_task = Task::new(TaskId::new(2), "Low Priority".to_string(), Some(8)).unwrap();
        
        assert!(high_priority_task.is_high_priority());
        assert!(!low_priority_task.is_high_priority());
    }

    #[test]
    fn test_transition_to_method() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Test Task".to_string(), Some(5)).unwrap();
        
        // Valid transition
        let result = task.transition_to(TaskStatus::InProgress);
        assert!(result.is_ok());
        assert_eq!(*task.status(), TaskStatus::InProgress);
        
        // Invalid transition
        let result = task.transition_to(TaskStatus::PendingReview);
        assert!(result.is_err());
        assert_eq!(*task.status(), TaskStatus::InProgress);
    }

    #[test]
    fn test_update_name_updates_timestamp() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Original name".to_string(), Some(5)).unwrap();
        let original_updated_at = task.updated_at;
        
        // Wait a small amount to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let result = task.update_name("New name".to_string());
        assert!(result.is_ok());
        assert_ne!(task.updated_at, original_updated_at);
        assert!(task.updated_at > original_updated_at);
    }

    #[test]
    fn test_update_priority_updates_timestamp() {
        let task_id = TaskId::new(1);
        let mut task = Task::new(task_id, "Task name".to_string(), Some(5)).unwrap();
        let original_updated_at = task.updated_at;
        
        // Wait a small amount to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        let result = task.update_priority(Some(8));
        assert!(result.is_ok());
        assert_ne!(task.updated_at, original_updated_at);
        assert!(task.updated_at > original_updated_at);
    }
}