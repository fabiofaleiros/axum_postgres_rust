use axum_postgres_rust::infrastructure::adapters::web::task_controller::WebError;
use axum_postgres_rust::application::use_cases::task_use_cases::UseCaseError;
use axum_postgres_rust::application::dto::{TaskDto, CreateTaskRequest, UpdateTaskRequest};
use axum_postgres_rust::domain::TaskStatus;
use chrono::Utc;
use axum_postgres_rust::responses::{TaskListResponse, TaskCreatedResponse};
use serde_json;

fn create_test_dto(id: i32, name: &str, priority: Option<i32>) -> TaskDto {
    TaskDto {
        id,
        name: name.to_string(),
        priority,
        status: TaskStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_error_from_use_case_error() {
        let validation_error = UseCaseError::ValidationError("Invalid input".to_string());
        let web_error = WebError::from(validation_error);
        
        match web_error {
            WebError::ValidationError(msg) => assert_eq!(msg, "Invalid input"),
            _ => panic!("Expected ValidationError"),
        }

        let not_found_error = UseCaseError::NotFound("Resource not found".to_string());
        let web_error = WebError::from(not_found_error);
        
        match web_error {
            WebError::NotFound(msg) => assert_eq!(msg, "Resource not found"),
            _ => panic!("Expected NotFound error"),
        }

        let repository_error = UseCaseError::RepositoryError("Database error".to_string());
        let web_error = WebError::from(repository_error);
        
        match web_error {
            WebError::InternalError(msg) => assert_eq!(msg, "Database error"),
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_web_error_debug() {
        let validation_error = WebError::ValidationError("Test validation".to_string());
        let debug_output = format!("{:?}", validation_error);
        
        assert!(debug_output.contains("ValidationError"));
        assert!(debug_output.contains("Test validation"));

        let not_found_error = WebError::NotFound("Test not found".to_string());
        let debug_output = format!("{:?}", not_found_error);
        
        assert!(debug_output.contains("NotFound"));
        assert!(debug_output.contains("Test not found"));

        let internal_error = WebError::InternalError("Test internal".to_string());
        let debug_output = format!("{:?}", internal_error);
        
        assert!(debug_output.contains("InternalError"));
        assert!(debug_output.contains("Test internal"));
    }

    #[tokio::test]
    async fn test_web_error_into_response() {
        use axum::response::IntoResponse;
        
        let validation_error = WebError::ValidationError("Validation failed".to_string());
        let response = validation_error.into_response();
        
        // We can't easily test the full response without axum test utils,
        // but we can verify the error converts to response
        assert!(response.status().is_client_error());
    }

    #[test]
    fn test_create_task_request_structure() {
        let request = CreateTaskRequest {
            name: "Test Task".to_string(),
            priority: Some(5),
        };
        
        assert_eq!(request.name, "Test Task");
        assert_eq!(request.priority, Some(5));
    }

    #[test]
    fn test_update_task_request_structure() {
        let request = UpdateTaskRequest {
            name: Some("Updated Task".to_string()),
            priority: Some(8),
        };
        
        assert_eq!(request.name, Some("Updated Task".to_string()));
        assert_eq!(request.priority, Some(8));

        let partial_request = UpdateTaskRequest {
            name: None,
            priority: Some(3),
        };
        
        assert_eq!(partial_request.name, None);
        assert_eq!(partial_request.priority, Some(3));
    }

    #[test]
    fn test_task_dto_structure() {
        let dto = create_test_dto(1, "Test Task", Some(5));
        
        assert_eq!(dto.id, 1);
        assert_eq!(dto.name, "Test Task");
        assert_eq!(dto.priority, Some(5));
    }

    #[test]
    fn test_task_query_deserialization() {
        // Test TaskQuery would be here if we could access it directly
        // For now, just test the JSON structure it would expect
        
        let json_with_priority = r#"{"priority":5}"#;
        let parsed: serde_json::Value = serde_json::from_str(json_with_priority).unwrap();
        assert_eq!(parsed["priority"], 5);

        let json_without_priority = r#"{}"#;
        let parsed: serde_json::Value = serde_json::from_str(json_without_priority).unwrap();
        assert!(parsed.get("priority").is_none());
    }

    #[test]
    fn test_api_response_structures() {
        let task_dto = create_test_dto(1, "Test", Some(5));
        let list_response = TaskListResponse { tasks: vec![task_dto] };
        
        assert_eq!(list_response.tasks.len(), 1);
        assert_eq!(list_response.tasks[0].name, "Test");

        let created_response = TaskCreatedResponse {
            task_id: 42,
            message: "Created".to_string(),
        };
        
        assert_eq!(created_response.task_id, 42);
        assert_eq!(created_response.message, "Created");
    }

    #[test]
    fn test_error_conversion_chain() {
        // Start with a use case error
        let original_error = UseCaseError::ValidationError("Original validation error".to_string());
        
        // Convert to web error
        let web_error = WebError::from(original_error);
        
        // Verify the conversion preserved the message
        match web_error {
            WebError::ValidationError(msg) => {
                assert_eq!(msg, "Original validation error");
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_all_use_case_error_conversions() {
        let test_cases = vec![
            (UseCaseError::ValidationError("val".to_string()), "ValidationError"),
            (UseCaseError::NotFound("not_found".to_string()), "NotFound"), 
            (UseCaseError::RepositoryError("repo".to_string()), "InternalError"),
        ];

        for (use_case_error, expected_variant) in test_cases {
            let web_error = WebError::from(use_case_error);
            let debug_str = format!("{:?}", web_error);
            assert!(debug_str.contains(expected_variant), 
                   "Expected {} in debug output: {}", expected_variant, debug_str);
        }
    }

    #[test]
    fn test_web_error_variants() {
        let validation = WebError::ValidationError("validation".to_string());
        let not_found = WebError::NotFound("not found".to_string());
        let internal = WebError::InternalError("internal".to_string());

        // Test that all variants can be created and matched
        match validation {
            WebError::ValidationError(msg) => assert_eq!(msg, "validation"),
            _ => panic!("Expected ValidationError"),
        }

        match not_found {
            WebError::NotFound(msg) => assert_eq!(msg, "not found"),
            _ => panic!("Expected NotFound"),
        }

        match internal {
            WebError::InternalError(msg) => assert_eq!(msg, "internal"),
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_request_and_response_serialization() {
        // Test that our DTOs can be serialized/deserialized
        let create_request = CreateTaskRequest {
            name: "New Task".to_string(),
            priority: Some(7),
        };

        let json = serde_json::to_string(&create_request).unwrap();
        let deserialized: CreateTaskRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, "New Task");
        assert_eq!(deserialized.priority, Some(7));

        let update_request = UpdateTaskRequest {
            name: Some("Updated".to_string()),
            priority: None,
        };

        let json = serde_json::to_string(&update_request).unwrap();
        let deserialized: UpdateTaskRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, Some("Updated".to_string()));
        assert_eq!(deserialized.priority, None);
    }

    #[test]
    fn test_task_list_response_with_multiple_tasks() {
        let tasks = vec![
            create_test_dto(1, "First", Some(1)),
            create_test_dto(2, "Second", Some(2)),
            create_test_dto(3, "Third", None),
        ];

        let response = TaskListResponse { tasks };
        
        assert_eq!(response.tasks.len(), 3);
        assert_eq!(response.tasks[0].name, "First");
        assert_eq!(response.tasks[1].name, "Second");
        assert_eq!(response.tasks[2].name, "Third");
        assert_eq!(response.tasks[2].priority, None);
    }

    #[test]
    fn test_task_created_response_serialization() {
        let response = TaskCreatedResponse {
            task_id: 999,
            message: "Successfully created task".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: TaskCreatedResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.task_id, 999);
        assert_eq!(deserialized.message, "Successfully created task");
    }
}