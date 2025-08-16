use axum_postgres_rust::{
    domain::{Task, TaskId, TaskRepository, StatusHistoryRepository, RepositoryError, StatusHistory, TaskStatus},
    application::{TaskUseCases, TaskDto, CreateTaskRequest, UpdateTaskRequest, UseCaseError},
    responses::{ApiResponse, TaskListResponse, TaskCreatedResponse},
};
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;

// Mock repository for integration testing
#[derive(Clone)]
struct MockRepository {
    tasks: Vec<Task>,
    next_id: i32,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            tasks: vec![],
            next_id: 1,
        }
    }

    fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
        self.next_id = tasks.iter().map(|t| t.id.value()).max().unwrap_or(0) + 1;
        self.tasks = tasks;
        self
    }
}

#[async_trait]
impl TaskRepository for MockRepository {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError> {
        Ok(self.tasks.clone())
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError> {
        Ok(self.tasks.iter().find(|t| t.id == id).cloned())
    }

    async fn find_by_priority(&self, priority: i32) -> Result<Vec<Task>, RepositoryError> {
        Ok(self.tasks
            .iter()
            .filter(|t| t.priority == Some(priority))
            .cloned()
            .collect())
    }

    async fn save(&self, _task: &Task) -> Result<TaskId, RepositoryError> {
        Ok(TaskId::new(self.next_id))
    }

    async fn update(&self, _task: &Task) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete(&self, _id: TaskId) -> Result<(), RepositoryError> {
        Ok(())
    }
}

// Mock status history repository for integration testing
#[derive(Clone)]
struct MockStatusHistoryRepository;

#[async_trait]
impl StatusHistoryRepository for MockStatusHistoryRepository {
    async fn find_by_task_id(&self, _task_id: i32) -> Result<Vec<StatusHistory>, RepositoryError> {
        Ok(vec![])
    }
    
    async fn find_by_date_range(
        &self, 
        _start_date: chrono::DateTime<chrono::Utc>, 
        _end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<StatusHistory>, RepositoryError> {
        Ok(vec![])
    }
    
    async fn find_latest_by_task_id(&self, _task_id: i32) -> Result<Option<StatusHistory>, RepositoryError> {
        Ok(None)
    }
    
    async fn get_task_analytics(&self, _task_id: i32) -> Result<Option<axum_postgres_rust::domain::TaskAnalytics>, RepositoryError> {
        Ok(None)
    }
    
    async fn get_completion_analytics(
        &self, 
        _start_date: chrono::DateTime<chrono::Utc>, 
        _end_date: chrono::DateTime<chrono::Utc>
    ) -> Result<Vec<axum_postgres_rust::domain::TaskAnalytics>, RepositoryError> {
        Ok(vec![])
    }
    
    async fn get_average_completion_times(&self) -> Result<Vec<(i32, chrono::Duration)>, RepositoryError> {
        Ok(vec![])
    }
    
    async fn save(&self, _history: &StatusHistory) -> Result<String, RepositoryError> {
        Ok("mock-id".to_string())
    }
    
    async fn delete(&self, _id: String) -> Result<(), RepositoryError> {
        Ok(())
    }
}

fn create_test_task(id: i32, name: &str, priority: Option<i32>) -> Task {
    Task::new(TaskId::new(id), name.to_string(), priority).unwrap()
}

fn create_use_cases_with_mock(mock_repo: MockRepository) -> TaskUseCases {
    TaskUseCases::new(Arc::new(mock_repo), Arc::new(MockStatusHistoryRepository))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests for the complete flow
    #[tokio::test]
    async fn test_complete_task_lifecycle() {
        let mock_repo = MockRepository::new();
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test create task
        let create_request = CreateTaskRequest {
            name: "Integration Test Task".to_string(),
            priority: Some(5),
        };

        let created_id = use_cases.create_task(create_request).await.unwrap();
        assert_eq!(created_id, 1);

        // Test get all tasks (empty in this mock)
        let all_tasks = use_cases.get_all_tasks().await.unwrap();
        assert_eq!(all_tasks.len(), 0); // Mock doesn't actually store

        // Test get task by id (not found in this mock)
        let result = use_cases.get_task_by_id(1).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::NotFound(_) => {}, // Expected
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_use_cases_with_existing_tasks() {
        let existing_tasks = vec![
            create_test_task(1, "Task 1", Some(5)),
            create_test_task(2, "Task 2", Some(3)),
            create_test_task(3, "Task 3", None),
        ];

        let mock_repo = MockRepository::new().with_tasks(existing_tasks);
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test get all tasks
        let all_tasks = use_cases.get_all_tasks().await.unwrap();
        assert_eq!(all_tasks.len(), 3);
        assert_eq!(all_tasks[0].name, "Task 1");
        assert_eq!(all_tasks[1].name, "Task 2");
        assert_eq!(all_tasks[2].name, "Task 3");

        // Test get task by id
        let task = use_cases.get_task_by_id(1).await.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.name, "Task 1");
        assert_eq!(task.priority, Some(5));

        // Test get tasks by priority
        let high_priority_tasks = use_cases.get_tasks_by_priority(5).await.unwrap();
        assert_eq!(high_priority_tasks.len(), 1);
        assert_eq!(high_priority_tasks[0].name, "Task 1");

        let medium_priority_tasks = use_cases.get_tasks_by_priority(3).await.unwrap();
        assert_eq!(medium_priority_tasks.len(), 1);
        assert_eq!(medium_priority_tasks[0].name, "Task 2");

        // Test get tasks by non-existent priority
        let no_tasks = use_cases.get_tasks_by_priority(10).await.unwrap();
        assert_eq!(no_tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_validation_flow() {
        let mock_repo = MockRepository::new();
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test create task with empty name
        let invalid_request = CreateTaskRequest {
            name: "".to_string(),
            priority: Some(5),
        };

        let result = use_cases.create_task(invalid_request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::ValidationError(msg) => {
                assert_eq!(msg, "Task name cannot be empty");
            }
            _ => panic!("Expected ValidationError"),
        }

        // Test create task with invalid priority
        let invalid_priority_request = CreateTaskRequest {
            name: "Valid Name".to_string(),
            priority: Some(15), // Invalid priority
        };

        let result = use_cases.create_task(invalid_priority_request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::ValidationError(msg) => {
                assert_eq!(msg, "Priority must be between 1 and 10");
            }
            _ => panic!("Expected ValidationError"),
        }

        // Test get tasks by invalid priority
        let result = use_cases.get_tasks_by_priority(0).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::ValidationError(msg) => {
                assert_eq!(msg, "Priority must be between 1 and 10");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn test_update_and_delete_operations() {
        let existing_tasks = vec![
            create_test_task(1, "Original Task", Some(5)),
        ];

        let mock_repo = MockRepository::new().with_tasks(existing_tasks);
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test update existing task
        let update_request = UpdateTaskRequest {
            name: Some("Updated Task".to_string()),
            priority: Some(8),
        };

        let result = use_cases.update_task(1, update_request).await;
        assert!(result.is_ok());

        // Test update non-existent task
        let update_request = UpdateTaskRequest {
            name: Some("Won't work".to_string()),
            priority: None,
        };

        let result = use_cases.update_task(999, update_request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::NotFound(_) => {}, // Expected
            _ => panic!("Expected NotFound error"),
        }

        // Test delete existing task
        let result = use_cases.delete_task(1).await;
        assert!(result.is_ok());

        // Test delete non-existent task
        let result = use_cases.delete_task(999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::NotFound(_) => {}, // Expected
            _ => panic!("Expected NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_dto_conversions() {
        let task = create_test_task(1, "DTO Test", Some(7));
        let dto = TaskDto::from(task.clone());

        // Test conversion from Task to TaskDto
        assert_eq!(dto.id, 1);
        assert_eq!(dto.name, "DTO Test");
        assert_eq!(dto.priority, Some(7));

        // Test conversion back from TaskDto to Task
        let converted_task = Task::try_from(dto).unwrap();
        assert_eq!(converted_task.id, task.id);
        assert_eq!(converted_task.name, task.name);
        assert_eq!(converted_task.priority, task.priority);
    }

    #[tokio::test]
    async fn test_api_response_structures() {
        // Test successful API responses
        let task_dto = TaskDto {
            id: 1,
            name: "API Test".to_string(),
            priority: Some(5),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let success_response = ApiResponse::success(task_dto);
        assert_eq!(success_response.success, true);
        assert!(success_response.data.is_some());
        assert_eq!(success_response.message, None);

        // Test error API responses
        let error_response = ApiResponse::<()>::error("Test error".to_string());
        assert_eq!(error_response.success, false);
        assert_eq!(error_response.data, None);
        assert_eq!(error_response.message, Some("Test error".to_string()));

        // Test task list response
        let tasks = vec![
            TaskDto { id: 1, name: "Task 1".to_string(), priority: Some(1), status: TaskStatus::Pending, created_at: Utc::now(), updated_at: Utc::now() },
            TaskDto { id: 2, name: "Task 2".to_string(), priority: Some(2), status: TaskStatus::Pending, created_at: Utc::now(), updated_at: Utc::now() },
        ];

        let list_response = TaskListResponse { tasks };
        assert_eq!(list_response.tasks.len(), 2);

        // Test task created response
        let created_response = TaskCreatedResponse {
            task_id: 42,
            message: "Task created successfully".to_string(),
        };

        assert_eq!(created_response.task_id, 42);
        assert_eq!(created_response.message, "Task created successfully");
    }

    #[tokio::test]
    async fn test_boundary_conditions() {
        let mock_repo = MockRepository::new();
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test with boundary priority values
        let min_priority_request = CreateTaskRequest {
            name: "Min Priority".to_string(),
            priority: Some(1),
        };

        let result = use_cases.create_task(min_priority_request).await;
        assert!(result.is_ok());

        let max_priority_request = CreateTaskRequest {
            name: "Max Priority".to_string(),
            priority: Some(10),
        };

        let result = use_cases.create_task(max_priority_request).await;
        assert!(result.is_ok());

        // Test with maximum allowed name length (255 chars)
        let long_name = "a".repeat(255);
        let long_name_request = CreateTaskRequest {
            name: long_name.clone(),
            priority: Some(5),
        };

        let result = use_cases.create_task(long_name_request).await;
        assert!(result.is_ok());

        // Test with name that's too long (256 chars)
        let too_long_name = "a".repeat(256);
        let too_long_request = CreateTaskRequest {
            name: too_long_name,
            priority: Some(5),
        };

        let result = use_cases.create_task(too_long_request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            UseCaseError::ValidationError(msg) => {
                assert_eq!(msg, "Task name cannot exceed 255 characters");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn test_edge_cases() {
        let existing_tasks = vec![
            create_test_task(1, "Whitespace Test", Some(5)), // Task with trimmed whitespace
            create_test_task(2, "Special Chars: éñ中文🚀", Some(3)), // Special characters
        ];

        let mock_repo = MockRepository::new().with_tasks(existing_tasks);
        let use_cases = create_use_cases_with_mock(mock_repo);

        // Test that we can retrieve tasks with special characters
        let all_tasks = use_cases.get_all_tasks().await.unwrap();
        assert_eq!(all_tasks.len(), 2);
        
        // The first task name should be trimmed during creation (the Task constructor trims whitespace)
        assert_eq!(all_tasks[0].name, "Whitespace Test"); // Task constructor trims whitespace
        assert_eq!(all_tasks[1].name, "Special Chars: éñ中文🚀");

        // Test partial updates
        let partial_update = UpdateTaskRequest {
            name: Some("Partially Updated".to_string()),
            priority: None, // Don't update priority
        };

        let result = use_cases.update_task(1, partial_update).await;
        assert!(result.is_ok());

        let priority_only_update = UpdateTaskRequest {
            name: None, // Don't update name
            priority: Some(9),
        };

        let result = use_cases.update_task(2, priority_only_update).await;
        assert!(result.is_ok());

        // Test empty update (no fields to update)
        let empty_update = UpdateTaskRequest {
            name: None,
            priority: None,
        };

        let result = use_cases.update_task(1, empty_update).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_operations_simulation() {
        // This test simulates concurrent operations by running multiple async operations
        let mock_repo = MockRepository::new();
        let use_cases = Arc::new(create_use_cases_with_mock(mock_repo));

        // Create multiple tasks concurrently
        let mut handles = vec![];

        for i in 1..=5 {
            let use_cases_clone = Arc::clone(&use_cases);
            let handle = tokio::spawn(async move {
                let request = CreateTaskRequest {
                    name: format!("Concurrent Task {}", i),
                    priority: Some(i % 10 + 1),
                };
                use_cases_clone.create_task(request).await
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        let results: Vec<Result<i32, UseCaseError>> = futures::future::try_join_all(handles)
            .await
            .unwrap();

        // All should succeed (in our mock implementation)
        for (i, result) in results.into_iter().enumerate() {
            assert!(result.is_ok(), "Task {} creation failed", i + 1);
        }
    }

    // Helper function to demonstrate the full architectural flow
    async fn demonstrate_hexagonal_architecture_flow() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Infrastructure Layer: Create repository
        let repository = MockRepository::new().with_tasks(vec![
            create_test_task(1, "Architecture Demo", Some(5)),
        ]);

        // 2. Application Layer: Create use cases
        let use_cases = TaskUseCases::new(Arc::new(repository), Arc::new(MockStatusHistoryRepository));

        // 3. Application Layer: Execute business logic
        let all_tasks = use_cases.get_all_tasks().await?;

        // 4. Infrastructure Layer (Web): Format response
        let response = ApiResponse::success(TaskListResponse { tasks: all_tasks });

        // 5. Verify the complete flow worked
        assert_eq!(response.success, true);
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert_eq!(data.tasks.len(), 1);
        assert_eq!(data.tasks[0].name, "Architecture Demo");

        Ok(())
    }

    #[tokio::test]
    async fn test_hexagonal_architecture_demo() {
        demonstrate_hexagonal_architecture_flow().await.unwrap();
    }
}