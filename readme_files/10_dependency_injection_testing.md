# Dependency Injection & Testing

This document covers advanced dependency injection patterns and comprehensive testing strategies for hexagonal architecture.

## 🔧 Dependency Injection Patterns

### 1. Constructor Injection

The primary pattern used throughout the application:

```rust
pub struct TaskUseCases {
    task_repository: Arc<dyn TaskRepository>,
    domain_service: TaskDomainService,
}

impl TaskUseCases {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self {
            task_repository,
            domain_service: TaskDomainService::new(),
        }
    }
}
```

**Benefits:**
- ✅ Dependencies are explicit and required
- ✅ Immutable after construction
- ✅ Compile-time dependency verification
- ✅ Easy to test with mock objects

### 2. Service Locator Alternative

For more complex scenarios, you might use a service container:

```rust
pub struct ServiceContainer {
    task_repository: Arc<dyn TaskRepository>,
    // other services...
}

impl ServiceContainer {
    pub fn new() -> Self {
        let config = Config::from_env().expect("Failed to load config");
        let db_pool = Database::connect(&config).await.expect("Failed to connect to DB");
        
        Self {
            task_repository: Arc::new(PostgresTaskRepository::new(db_pool)),
        }
    }
    
    pub fn task_use_cases(&self) -> TaskUseCases {
        TaskUseCases::new(self.task_repository.clone())
    }
}
```

### 3. Configuration-Based Injection

Different implementations based on configuration:

```rust
pub fn create_task_repository(config: &Config) -> Arc<dyn TaskRepository> {
    match config.database_type.as_str() {
        "postgres" => {
            let pool = create_postgres_pool(&config.database_url);
            Arc::new(PostgresTaskRepository::new(pool))
        },
        "sqlite" => {
            let pool = create_sqlite_pool(&config.database_url);
            Arc::new(SqliteTaskRepository::new(pool))
        },
        "memory" => {
            Arc::new(InMemoryTaskRepository::new())
        },
        _ => panic!("Unsupported database type: {}", config.database_type),
    }
}
```

## 🧪 Testing Strategies

### 1. Unit Testing Domain Layer

Domain layer tests are pure and isolated:

```rust
#[cfg(test)]
mod domain_tests {
    use super::*;

    #[test]
    fn task_validates_name_on_creation() {
        let result = Task::new(TaskId::new(1), "".to_string(), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn task_validates_priority_range() {
        let result = Task::new(TaskId::new(1), "Valid Name".to_string(), Some(15));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn task_domain_service_validates_name_length() {
        let service = TaskDomainService::new();
        let long_name = "a".repeat(300);
        
        let result = service.validate_task_name(&long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot exceed 255 characters"));
    }
}
```

### 2. Testing Application Layer with Mocks

Using `mockall` for mocking repositories:

```rust
// Add to Cargo.toml
[dev-dependencies]
mockall = "0.12"

// In test file
use mockall::{predicate::*, mock};

mock! {
    TaskRepo {}
    
    #[async_trait]
    impl TaskRepository for TaskRepo {
        async fn find_all(&self) -> Result<Vec<Task>, RepositoryError>;
        async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError>;
        async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError>;
        async fn update(&self, task: &Task) -> Result<(), RepositoryError>;
        async fn delete(&self, id: TaskId) -> Result<(), RepositoryError>;
        async fn find_by_priority(&self, priority: i32) -> Result<Vec<Task>, RepositoryError>;
    }
}

#[cfg(test)]
mod use_case_tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn create_task_success() {
        let mut mock_repo = MockTaskRepo::new();
        
        // Set up expectations
        mock_repo
            .expect_save()
            .times(1)
            .with(predicate::function(|task: &Task| task.name == "Test Task"))
            .returning(|_| Ok(TaskId::new(42)));

        let use_cases = TaskUseCases::new(Arc::new(mock_repo));
        
        let request = CreateTaskRequest {
            name: "Test Task".to_string(),
            priority: Some(5),
        };

        let result = use_cases.create_task(request).await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn create_task_validation_error() {
        let mock_repo = MockTaskRepo::new();
        let use_cases = TaskUseCases::new(Arc::new(mock_repo));
        
        let request = CreateTaskRequest {
            name: "".to_string(),  // Invalid empty name
            priority: Some(5),
        };

        let result = use_cases.create_task(request).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            UseCaseError::ValidationError(msg) => {
                assert_eq!(msg, "Task name cannot be empty");
            },
            _ => panic!("Expected ValidationError"),
        }
    }

    #[tokio::test]
    async fn update_non_existent_task() {
        let mut mock_repo = MockTaskRepo::new();
        
        mock_repo
            .expect_find_by_id()
            .with(eq(TaskId::new(999)))
            .times(1)
            .returning(|_| Ok(None));

        let use_cases = TaskUseCases::new(Arc::new(mock_repo));
        
        let request = UpdateTaskRequest {
            name: Some("Updated Name".to_string()),
            priority: None,
        };

        let result = use_cases.update_task(999, request).await;
        assert!(result.is_err());
        
        match result.unwrap_err() {
            UseCaseError::NotFound(msg) => {
                assert!(msg.contains("999"));
            },
            _ => panic!("Expected NotFound error"),
        }
    }
}
```

### 3. Integration Testing with Test Database

```rust
// Test utilities
pub async fn setup_test_database() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://test:test@localhost/test_db".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE tasks RESTART IDENTITY")
        .execute(pool)
        .await
        .expect("Failed to cleanup database");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_task_lifecycle() {
        let pool = setup_test_database().await;
        let repo = PostgresTaskRepository::new(pool.clone());
        let use_cases = TaskUseCases::new(Arc::new(repo));

        // Create task
        let create_request = CreateTaskRequest {
            name: "Integration Test Task".to_string(),
            priority: Some(3),
        };
        
        let task_id = use_cases.create_task(create_request).await.unwrap();
        assert!(task_id > 0);

        // Retrieve task
        let task = use_cases.get_task_by_id(task_id).await.unwrap();
        assert_eq!(task.name, "Integration Test Task");
        assert_eq!(task.priority, Some(3));

        // Update task
        let update_request = UpdateTaskRequest {
            name: Some("Updated Task".to_string()),
            priority: Some(8),
        };
        
        use_cases.update_task(task_id, update_request).await.unwrap();

        // Verify update
        let updated_task = use_cases.get_task_by_id(task_id).await.unwrap();
        assert_eq!(updated_task.name, "Updated Task");
        assert_eq!(updated_task.priority, Some(8));

        // Delete task
        use_cases.delete_task(task_id).await.unwrap();

        // Verify deletion
        let result = use_cases.get_task_by_id(task_id).await;
        assert!(result.is_err());

        cleanup_database(&pool).await;
    }
}
```

### 4. HTTP Integration Testing

```rust
use axum_test::TestServer;

#[tokio::test]
async fn test_http_endpoints() {
    // Set up test server with in-memory repository
    let repo = Arc::new(InMemoryTaskRepository::new());
    let use_cases = Arc::new(TaskUseCases::new(repo));
    let controller = Arc::new(TaskController::new(use_cases));
    
    let app = Router::new()
        .route("/tasks", get(TaskController::get_tasks).post(TaskController::create_task))
        .route("/tasks/{id}", get(TaskController::get_task))
        .with_state(controller);

    let server = TestServer::new(app).unwrap();

    // Test creating a task
    let create_response = server
        .post("/tasks")
        .json(&serde_json::json!({
            "name": "HTTP Test Task",
            "priority": 7
        }))
        .await;
    
    create_response.assert_status_created();
    let created_task: ApiResponse<TaskCreatedResponse> = create_response.json();
    assert!(created_task.success);
    let task_id = created_task.data.unwrap().task_id;

    // Test retrieving the task
    let get_response = server
        .get(&format!("/tasks/{}", task_id))
        .await;
    
    get_response.assert_status_ok();
    let task_response: ApiResponse<TaskDto> = get_response.json();
    assert_eq!(task_response.data.unwrap().name, "HTTP Test Task");
}
```

## 🎯 Testing Best Practices

### 1. Test Pyramid

```
        🔺 E2E Tests (Few)
       /               \
      /  Integration     \
     /     Tests         \
    /    (Some)          \
   /                     \
  /_______________________\
        Unit Tests (Many)
```

- **Many unit tests**: Fast, isolated, test business logic
- **Some integration tests**: Test component interactions
- **Few E2E tests**: Test complete user workflows

### 2. Test Organization

```rust
// Domain tests - pure unit tests
#[cfg(test)]
mod domain_tests { /* ... */ }

// Application tests - with mocks
#[cfg(test)]
mod application_tests { /* ... */ }

// Integration tests - separate files
// tests/integration_tests.rs
// tests/http_tests.rs
```

### 3. Test Data Builders

```rust
pub struct TaskBuilder {
    id: TaskId,
    name: String,
    priority: Option<i32>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            id: TaskId::new(1),
            name: "Default Task".to_string(),
            priority: Some(1),
        }
    }
    
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = TaskId::new(id);
        self
    }
    
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }
    
    pub fn build(self) -> Result<Task, String> {
        Task::new(self.id, self.name, self.priority)
    }
}

// Usage in tests
let task = TaskBuilder::new()
    .with_name("Test Task")
    .with_priority(5)
    .build()
    .unwrap();
```

### 4. Async Testing Utilities

```rust
// Helper for testing async use cases
pub async fn assert_use_case_error<T>(
    result: Result<T, UseCaseError>,
    expected_error: &str,
) {
    match result {
        Err(UseCaseError::ValidationError(msg)) => assert_eq!(msg, expected_error),
        Err(UseCaseError::NotFound(msg)) => assert_eq!(msg, expected_error),
        Err(other) => panic!("Unexpected error type: {:?}", other),
        Ok(_) => panic!("Expected error but got success"),
    }
}

// Usage
assert_use_case_error(
    use_cases.create_task(invalid_request).await,
    "Task name cannot be empty"
).await;
```

## 📊 Test Coverage

Run tests with coverage:

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run tests with coverage
cargo tarpaulin --out Html

# View coverage report
open tarpaulin-report.html
```

## 🚀 Continuous Integration

Example GitHub Actions workflow:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test
      env:
        TEST_DATABASE_URL: postgres://postgres:postgres@localhost/test_db
        
    - name: Run integration tests
      run: cargo test --test integration_tests
```

This comprehensive testing strategy ensures that your hexagonal architecture maintains its integrity and business logic correctness across all layers.