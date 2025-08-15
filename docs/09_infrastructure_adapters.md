# Infrastructure Adapters

The **Infrastructure Layer** implements the ports defined in the domain layer and handles all external concerns like databases, web frameworks, and external services.

## Structure Overview

```
src/infrastructure/
├── adapters/
│   ├── repositories/          # Database implementations
│   │   └── postgres_task_repository.rs
│   └── web/                   # HTTP adapters
│       └── task_controller.rs
└── persistence/               # Database schema and migrations
```

## Repository Adapters

Repository adapters implement the domain's repository ports using specific database technologies.

### PostgresTaskRepository

```rust
pub struct PostgresTaskRepository {
    pool: PgPool,
}

impl PostgresTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PostgresTaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let rows = sqlx::query("SELECT task_id, name, priority FROM tasks ORDER BY task_id")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut tasks = Vec::new();
        for row in rows {
            let task_id: i32 = row.get("task_id");
            let name: String = row.get("name");
            let priority: Option<i32> = row.get("priority");
            
            let task = Task::new(TaskId::new(task_id), name, priority)
                .map_err(RepositoryError::ValidationError)?;
            tasks.push(task);
        }

        Ok(tasks)
    }
    
    // ... other implementations
}
```

**Key Characteristics:**
- Implements the domain's `TaskRepository` trait
- Handles database-specific concerns (SQL queries, connection pooling)
- Converts between database rows and domain entities
- Maps database errors to domain errors

### Benefits of the Adapter Pattern

1. **Technology Independence**
   ```rust
   // Easy to swap implementations
   let postgres_repo = PostgresTaskRepository::new(pg_pool);
   let sqlite_repo = SqliteTaskRepository::new(sqlite_pool);
   let mongo_repo = MongoTaskRepository::new(mongo_client);
   
   // All implement the same TaskRepository trait
   ```

2. **Isolated Database Logic**
   ```rust
   // Database queries are isolated in adapters
   // Business logic never sees SQL or database details
   ```

3. **Error Translation**
   ```rust
   // Database errors are converted to domain errors
   .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
   ```

## Web Adapters

Web adapters handle HTTP concerns and translate between HTTP and application layer.

### TaskController

```rust
pub struct TaskController {
    task_use_cases: Arc<TaskUseCases>,
}

impl TaskController {
    pub fn new(task_use_cases: Arc<TaskUseCases>) -> Self {
        Self { task_use_cases }
    }

    pub async fn get_tasks(
        State(controller): State<Arc<TaskController>>,
        Query(params): Query<TaskQuery>,
    ) -> Result<Json<ApiResponse<TaskListResponse>>, WebError> {
        let tasks = match params.priority {
            Some(priority) => controller.task_use_cases.get_tasks_by_priority(priority).await?,
            None => controller.task_use_cases.get_all_tasks().await?,
        };

        let response = ApiResponse::success(TaskListResponse { tasks });
        Ok(Json(response))
    }
}
```

**Responsibilities:**
- HTTP request/response handling
- Parameter extraction and validation
- Error translation to HTTP status codes
- Response formatting

### Web Error Handling

```rust
#[derive(Debug)]
pub enum WebError {
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl From<UseCaseError> for WebError {
    fn from(error: UseCaseError) -> Self {
        match error {
            UseCaseError::ValidationError(msg) => WebError::ValidationError(msg),
            UseCaseError::NotFound(msg) => WebError::NotFound(msg),
            UseCaseError::RepositoryError(msg) => WebError::InternalError(msg),
        }
    }
}

impl axum::response::IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            WebError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            WebError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            WebError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_response = ApiResponse::<()>::error(message);
        (status, Json(error_response)).into_response()
    }
}
```

**Error Translation Strategy:**
- Application errors → HTTP status codes
- Consistent error response format
- No internal error details leaked to clients

## Dependency Injection

The infrastructure layer wires everything together in `main.rs`.

### Dependency Wiring

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_env()?;

    // Create database connection pool
    let db_pool = Database::connect(&config).await?;

    // Create repository (infrastructure → domain port)
    let task_repository: Arc<dyn TaskRepository> = 
        Arc::new(PostgresTaskRepository::new(db_pool));
    
    // Create use cases (application layer)
    let task_use_cases = Arc::new(TaskUseCases::new(task_repository));
    
    // Create controllers (infrastructure web adapter)
    let task_controller = Arc::new(TaskController::new(task_use_cases));

    // Build router with dependency injection
    let app = Router::new()
        .route("/tasks", get(TaskController::get_tasks))
        .with_state(task_controller);

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}
```

**Dependency Flow:**
```
PostgresTaskRepository → TaskUseCases → TaskController → Axum Router
```

## Adapter Benefits

### 1. **Testability**
```rust
// Easy to create test doubles
struct InMemoryTaskRepository {
    tasks: Arc<Mutex<HashMap<TaskId, Task>>>,
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    // In-memory implementation for testing
}
```

### 2. **Technology Swapping**
```rust
// Production
let repo = PostgresTaskRepository::new(pg_pool);

// Testing
let repo = InMemoryTaskRepository::new();

// Different database
let repo = MongoTaskRepository::new(mongo_client);
```

### 3. **Configuration-Based Selection**
```rust
let repo: Arc<dyn TaskRepository> = match config.database_type {
    DatabaseType::Postgres => Arc::new(PostgresTaskRepository::new(pg_pool)),
    DatabaseType::Sqlite => Arc::new(SqliteTaskRepository::new(sqlite_pool)),
    DatabaseType::InMemory => Arc::new(InMemoryTaskRepository::new()),
};
```

## Request Flow Through Adapters

```
1. HTTP Request → Axum → TaskController (Web Adapter)
2. TaskController → TaskUseCases (Application Layer)
3. TaskUseCases → TaskRepository Port (Domain Layer)
4. PostgresTaskRepository (Database Adapter) → PostgreSQL
5. Response flows back through the same path
```

## Testing Adapters

### Integration Tests
```rust
#[tokio::test]
async fn test_postgres_repository_integration() {
    let pool = setup_test_database().await;
    let repo = PostgresTaskRepository::new(pool);
    
    let task = Task::new(TaskId::new(0), "Test Task".to_string(), Some(5)).unwrap();
    let task_id = repo.save(&task).await.unwrap();
    
    let retrieved = repo.find_by_id(task_id).await.unwrap().unwrap();
    assert_eq!(retrieved.name, "Test Task");
}
```

### Unit Tests for Controllers
```rust
#[tokio::test]
async fn test_get_tasks_endpoint() {
    let mock_use_cases = create_mock_use_cases();
    let controller = TaskController::new(Arc::new(mock_use_cases));
    
    // Test HTTP handling logic
}
```

## Design Patterns in Infrastructure

### 1. **Adapter Pattern**
- Adapts external interfaces to domain ports
- Allows technology independence

### 2. **Dependency Injection**
- Dependencies injected through constructors
- Enables loose coupling and testability

### 3. **Repository Pattern**
- Abstract data access behind repository interface
- Separates business logic from data access concerns

### 4. **Error Translation**
- Each layer translates errors appropriately
- External errors never leak to inner layers

## Next: Dependency Injection & Testing

Continue to [Dependency Injection & Testing](10_dependency_injection_testing.md) for advanced testing strategies.