use axum_postgres_rust::domain::{Task, TaskId, TaskDomainService};

fn create_test_task() -> Task {
    Task::new(TaskId::new(1), "Test Task".to_string(), Some(5)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_domain_service() {
        let service = TaskDomainService::new();
        // Just verify it can be created
        assert_eq!(std::mem::size_of_val(&service), 0); // Zero-sized type
    }

    #[test]
    fn test_validate_task_name_valid() {
        let service = TaskDomainService::new();
        
        let result = service.validate_task_name("Valid task name");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_task_name_empty() {
        let service = TaskDomainService::new();
        
        let result = service.validate_task_name("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_validate_task_name_whitespace_only() {
        let service = TaskDomainService::new();
        
        let result = service.validate_task_name("   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_validate_task_name_too_long() {
        let service = TaskDomainService::new();
        let long_name = "a".repeat(256); // 256 characters
        
        let result = service.validate_task_name(&long_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot exceed 255 characters");
    }

    #[test]
    fn test_validate_task_name_exactly_255_chars() {
        let service = TaskDomainService::new();
        let name_255_chars = "a".repeat(255);
        
        let result = service.validate_task_name(&name_255_chars);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_task_name_with_special_characters() {
        let service = TaskDomainService::new();
        
        let result = service.validate_task_name("Task with special chars: !@#$%^&*()");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority_none() {
        let service = TaskDomainService::new();
        
        let result = service.validate_priority(None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority_valid_range() {
        let service = TaskDomainService::new();
        
        for priority in 1..=10 {
            let result = service.validate_priority(Some(priority));
            assert!(result.is_ok(), "Priority {} should be valid", priority);
        }
    }

    #[test]
    fn test_validate_priority_too_low() {
        let service = TaskDomainService::new();
        
        let result = service.validate_priority(Some(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
        
        let result = service.validate_priority(Some(-1));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_validate_priority_too_high() {
        let service = TaskDomainService::new();
        
        let result = service.validate_priority(Some(11));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
        
        let result = service.validate_priority(Some(100));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_can_update_task_valid_name_only() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, Some("New name"), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_update_task_valid_priority_only() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, None, Some(8));
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_update_task_valid_both() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, Some("New name"), Some(8));
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_update_task_invalid_name() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, Some(""), Some(8));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_can_update_task_invalid_priority() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, Some("Valid name"), Some(11));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_can_update_task_no_changes() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_can_update_task_long_name() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        let long_name = "a".repeat(256);
        
        let result = service.can_update_task(&task, Some(&long_name), Some(5));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot exceed 255 characters");
    }

    #[test]
    fn test_can_update_task_whitespace_name() {
        let service = TaskDomainService::new();
        let task = create_test_task();
        
        let result = service.can_update_task(&task, Some("   "), Some(5));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }
}