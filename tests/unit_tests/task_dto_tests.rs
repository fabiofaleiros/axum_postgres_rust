use axum_postgres_rust::application::dto::{TaskDto, CreateTaskRequest, UpdateTaskRequest};
use axum_postgres_rust::domain::{Task, TaskId, TaskStatus};
use chrono::Utc;
use serde_json;

fn create_test_task(id: i32, name: &str, priority: Option<i32>) -> Task {
    Task::new(TaskId::new(id), name.to_string(), priority).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_dto_from_task() {
        let task = create_test_task(1, "Test Task", Some(5));
        let dto = TaskDto::from(task);

        assert_eq!(dto.id, 1);
        assert_eq!(dto.name, "Test Task");
        assert_eq!(dto.priority, Some(5));
    }

    #[test]
    fn test_task_dto_from_task_no_priority() {
        let task = create_test_task(2, "No Priority Task", None);
        let dto = TaskDto::from(task);

        assert_eq!(dto.id, 2);
        assert_eq!(dto.name, "No Priority Task");
        assert_eq!(dto.priority, None);
    }

    #[test]
    fn test_task_from_task_dto_success() {
        let dto = TaskDto {
            id: 3,
            name: "Valid Task".to_string(),
            priority: Some(7),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let task = Task::try_from(dto).unwrap();
        assert_eq!(task.id.value(), 3);
        assert_eq!(task.name, "Valid Task");
        assert_eq!(task.priority, Some(7));
    }

    #[test]
    fn test_task_from_task_dto_invalid_name() {
        let dto = TaskDto {
            id: 4,
            name: "".to_string(), // Invalid empty name
            priority: Some(3),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = Task::try_from(dto);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn test_task_from_task_dto_invalid_priority() {
        let dto = TaskDto {
            id: 5,
            name: "Valid Name".to_string(),
            priority: Some(11), // Invalid priority
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = Task::try_from(dto);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn test_task_dto_serialization() {
        let dto = TaskDto {
            id: 1,
            name: "Serialization Test".to_string(),
            priority: Some(8),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"name\":\"Serialization Test\""));
        assert!(serialized.contains("\"priority\":8"));
    }

    #[test]
    fn test_task_dto_deserialization() {
        let json = r#"{"id":2,"name":"Deserialization Test","priority":6,"status":"Pending","created_at":"2023-01-01T00:00:00Z","updated_at":"2023-01-01T00:00:00Z"}"#;
        let dto: TaskDto = serde_json::from_str(json).unwrap();

        assert_eq!(dto.id, 2);
        assert_eq!(dto.name, "Deserialization Test");
        assert_eq!(dto.priority, Some(6));
        assert_eq!(dto.status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_dto_serialization_no_priority() {
        let dto = TaskDto {
            id: 3,
            name: "No Priority".to_string(),
            priority: None,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        assert!(serialized.contains("\"priority\":null"));
    }

    #[test]
    fn test_create_task_request_deserialization() {
        let json = r#"{"name":"New Task","priority":4}"#;
        let request: CreateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, "New Task");
        assert_eq!(request.priority, Some(4));
    }

    #[test]
    fn test_create_task_request_deserialization_no_priority() {
        let json = r#"{"name":"Task without priority","priority":null}"#;
        let request: CreateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, "Task without priority");
        assert_eq!(request.priority, None);
    }

    #[test]
    fn test_create_task_request_debug() {
        let request = CreateTaskRequest {
            name: "Debug Test".to_string(),
            priority: Some(9),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("CreateTaskRequest"));
        assert!(debug_output.contains("Debug Test"));
        assert!(debug_output.contains("9"));
    }

    #[test]
    fn test_update_task_request_deserialization_full() {
        let json = r#"{"name":"Updated Task","priority":7}"#;
        let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, Some("Updated Task".to_string()));
        assert_eq!(request.priority, Some(7));
    }

    #[test]
    fn test_update_task_request_deserialization_partial_name() {
        let json = r#"{"name":"Updated Name","priority":null}"#;
        let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, Some("Updated Name".to_string()));
        assert_eq!(request.priority, None);
    }

    #[test]
    fn test_update_task_request_deserialization_partial_priority() {
        let json = r#"{"name":null,"priority":10}"#;
        let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, None);
        assert_eq!(request.priority, Some(10));
    }

    #[test]
    fn test_update_task_request_deserialization_empty() {
        let json = r#"{"name":null,"priority":null}"#;
        let request: UpdateTaskRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, None);
        assert_eq!(request.priority, None);
    }

    #[test]
    fn test_update_task_request_debug() {
        let request = UpdateTaskRequest {
            name: Some("Debug Update".to_string()),
            priority: None,
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("UpdateTaskRequest"));
        assert!(debug_output.contains("Debug Update"));
    }

    #[test]
    fn test_task_dto_roundtrip_conversion() {
        let original_task = create_test_task(10, "Roundtrip Test", Some(3));
        let dto = TaskDto::from(original_task.clone());
        let converted_task = Task::try_from(dto).unwrap();

        assert_eq!(original_task.id, converted_task.id);
        assert_eq!(original_task.name, converted_task.name);
        assert_eq!(original_task.priority, converted_task.priority);
    }

    #[test]
    fn test_task_dto_with_special_characters() {
        let task = create_test_task(11, "Task with special chars: éñ中文🚀", Some(2));
        let dto = TaskDto::from(task);

        assert_eq!(dto.name, "Task with special chars: éñ中文🚀");
        
        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: TaskDto = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(deserialized.name, "Task with special chars: éñ中文🚀");
    }

    #[test]
    fn test_create_task_request_missing_fields_error() {
        let json = r#"{"priority":5}"#; // Missing name field
        let result = serde_json::from_str::<CreateTaskRequest>(json);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_task_dto_with_negative_id() {
        let task = create_test_task(-1, "Negative ID Task", Some(1));
        let dto = TaskDto::from(task);

        assert_eq!(dto.id, -1);
        assert_eq!(dto.name, "Negative ID Task");
    }

    #[test]
    fn test_task_dto_equality_after_serialization_roundtrip() {
        let dto = TaskDto {
            id: 100,
            name: "Roundtrip Equality Test".to_string(),
            priority: Some(5),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: TaskDto = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dto.id, deserialized.id);
        assert_eq!(dto.name, deserialized.name);
        assert_eq!(dto.priority, deserialized.priority);
    }
}
