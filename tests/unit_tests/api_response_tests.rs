use axum_postgres_rust::responses::{ApiResponse, TaskListResponse, TaskCreatedResponse};
use axum_postgres_rust::application::dto::TaskDto;
use axum_postgres_rust::domain::TaskStatus;
use chrono::Utc;
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
    fn test_api_response_success() {
        let data = "test data";
        let response = ApiResponse::success(data);

        assert_eq!(response.success, true);
        assert_eq!(response.data, Some("test data"));
        assert_eq!(response.message, None);
    }

    #[test]
    fn test_api_response_success_with_complex_data() {
        let task_dto = create_test_dto(1, "Test Task", Some(5));
        let response = ApiResponse::success(task_dto);

        assert_eq!(response.success, true);
        assert!(response.data.is_some());
        assert_eq!(response.message, None);
        
        let data = response.data.unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "Test Task");
        assert_eq!(data.priority, Some(5));
    }

    #[test]
    fn test_api_response_error() {
        let error_message = "Something went wrong".to_string();
        let response = ApiResponse::<()>::error(error_message);

        assert_eq!(response.success, false);
        assert_eq!(response.data, None);
        assert_eq!(response.message, Some("Something went wrong".to_string()));
    }

    #[test]
    fn test_api_response_success_serialization() {
        let response = ApiResponse::success("test");
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("\"success\":true"));
        assert!(serialized.contains("\"data\":\"test\""));
        assert!(serialized.contains("\"message\":null"));
    }

    #[test]
    fn test_api_response_error_serialization() {
        let response = ApiResponse::<()>::error("error message".to_string());
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("\"success\":false"));
        assert!(serialized.contains("\"data\":null"));
        assert!(serialized.contains("\"message\":\"error message\""));
    }

    #[test]
    fn test_api_response_debug() {
        let response = ApiResponse::success(42);
        let debug_output = format!("{:?}", response);

        assert!(debug_output.contains("ApiResponse"));
        assert!(debug_output.contains("success: true"));
        assert!(debug_output.contains("data: Some(42)"));
    }

    #[test]
    fn test_task_list_response() {
        let tasks = vec![
            create_test_dto(1, "Task 1", Some(3)),
            create_test_dto(2, "Task 2", None),
        ];
        let response = TaskListResponse { tasks };

        assert_eq!(response.tasks.len(), 2);
        assert_eq!(response.tasks[0].name, "Task 1");
        assert_eq!(response.tasks[1].name, "Task 2");
    }

    #[test]
    fn test_task_list_response_empty() {
        let response = TaskListResponse { tasks: vec![] };
        assert_eq!(response.tasks.len(), 0);
    }

    #[test]
    fn test_task_list_response_serialization() {
        let tasks = vec![create_test_dto(1, "Test", Some(5))];
        let response = TaskListResponse { tasks };
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("\"tasks\""));
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"name\":\"Test\""));
        assert!(serialized.contains("\"priority\":5"));
    }

    #[test]
    fn test_task_list_response_debug() {
        let tasks = vec![create_test_dto(1, "Debug Task", Some(7))];
        let response = TaskListResponse { tasks };
        let debug_output = format!("{:?}", response);

        assert!(debug_output.contains("TaskListResponse"));
        assert!(debug_output.contains("Debug Task"));
    }

    #[test]
    fn test_task_created_response() {
        let response = TaskCreatedResponse {
            task_id: 42,
            message: "Task created successfully".to_string(),
        };

        assert_eq!(response.task_id, 42);
        assert_eq!(response.message, "Task created successfully");
    }

    #[test]
    fn test_task_created_response_serialization() {
        let response = TaskCreatedResponse {
            task_id: 123,
            message: "Success".to_string(),
        };
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("\"task_id\":123"));
        assert!(serialized.contains("\"message\":\"Success\""));
    }

    #[test]
    fn test_task_created_response_debug() {
        let response = TaskCreatedResponse {
            task_id: 99,
            message: "Created".to_string(),
        };
        let debug_output = format!("{:?}", response);

        assert!(debug_output.contains("TaskCreatedResponse"));
        assert!(debug_output.contains("99"));
        assert!(debug_output.contains("Created"));
    }

    #[test]
    fn test_api_response_with_task_list() {
        let tasks = vec![create_test_dto(1, "API Test", Some(2))];
        let task_list = TaskListResponse { tasks };
        let api_response = ApiResponse::success(task_list);

        assert_eq!(api_response.success, true);
        assert!(api_response.data.is_some());
        assert_eq!(api_response.message, None);
        
        let data = api_response.data.unwrap();
        assert_eq!(data.tasks.len(), 1);
        assert_eq!(data.tasks[0].name, "API Test");
    }

    #[test]
    fn test_api_response_with_task_created() {
        let created_response = TaskCreatedResponse {
            task_id: 456,
            message: "Task created".to_string(),
        };
        let api_response = ApiResponse::success(created_response);

        assert_eq!(api_response.success, true);
        assert!(api_response.data.is_some());
        
        let data = api_response.data.unwrap();
        assert_eq!(data.task_id, 456);
        assert_eq!(data.message, "Task created");
    }

    #[test]
    fn test_api_response_serialization_with_nested_data() {
        let tasks = vec![create_test_dto(1, "Nested", Some(1))];
        let task_list = TaskListResponse { tasks };
        let api_response = ApiResponse::success(task_list);
        
        let serialized = serde_json::to_string(&api_response).unwrap();

        assert!(serialized.contains("\"success\":true"));
        assert!(serialized.contains("\"tasks\""));
        assert!(serialized.contains("\"Nested\""));
        assert!(serialized.contains("\"message\":null"));
    }

    #[test]
    fn test_api_response_different_data_types() {
        // Test with string
        let string_response = ApiResponse::success("string data".to_string());
        assert_eq!(string_response.data, Some("string data".to_string()));

        // Test with number
        let number_response = ApiResponse::success(42);
        assert_eq!(number_response.data, Some(42));

        // Test with boolean
        let bool_response = ApiResponse::success(true);
        assert_eq!(bool_response.data, Some(true));
    }

    #[test]
    fn test_api_response_error_with_various_messages() {
        let test_messages = vec![
            "Validation failed",
            "Resource not found", 
            "Internal server error",
            "Permission denied",
        ];

        for message in test_messages {
            let response = ApiResponse::<()>::error(message.to_string());
            assert_eq!(response.success, false);
            assert_eq!(response.data, None);
            assert_eq!(response.message, Some(message.to_string()));
        }
    }

    #[test]
    fn test_task_list_response_with_many_tasks() {
        let tasks: Vec<TaskDto> = (1..=100)
            .map(|i| create_test_dto(i, &format!("Task {}", i), Some(i % 10 + 1)))
            .collect();

        let response = TaskListResponse { tasks };
        assert_eq!(response.tasks.len(), 100);
        assert_eq!(response.tasks[0].name, "Task 1");
        assert_eq!(response.tasks[99].name, "Task 100");
    }

    #[test]
    fn test_task_created_response_with_special_characters() {
        let response = TaskCreatedResponse {
            task_id: 1,
            message: "Task créé avec succès! 🎉".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: TaskCreatedResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.task_id, 1);
        assert_eq!(deserialized.message, "Task créé avec succès! 🎉");
    }

    #[test]
    fn test_api_response_success_with_none_data() {
        let response: ApiResponse<Option<String>> = ApiResponse::success(None);
        
        assert_eq!(response.success, true);
        assert_eq!(response.data, Some(None));
        assert_eq!(response.message, None);
    }
}