# Application Layer & Use Cases

The **Application Layer** orchestrates domain objects to fulfill specific use cases. It's the coordination layer between the domain and infrastructure.

## 🔄 Structure Overview

```
src/application/
├── dto/              # Data Transfer Objects
│   └── task_dto.rs  # External representation of tasks
└── use_cases/        # Application use cases
    └── task_use_cases.rs  # Task business operations
```

## 📋 Use Cases

Use cases represent the application-specific business rules and coordinate domain objects to fulfill user intentions.

### TaskUseCases

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

**Key Characteristics:**
- ✅ Coordinates multiple domain objects
- ✅ Implements application-specific business rules
- ✅ Uses dependency injection for external dependencies
- ✅ Returns application-specific errors

## 🎯 Use Case Implementation Examples

### 1. Get All Tasks

```rust
pub async fn get_all_tasks(&self) -> Result<Vec<TaskDto>, UseCaseError> {
    let tasks = self.task_repository.find_all().await?;
    Ok(tasks.into_iter().map(TaskDto::from).collect())
}
```

**Flow:**
1. Query repository for all tasks
2. Convert domain entities to DTOs
3. Return result to caller

### 2. Create Task

```rust
pub async fn create_task(&self, request: CreateTaskRequest) -> Result<i32, UseCaseError> {
    // Application-level validation
    self.domain_service.validate_task_name(&request.name)
        .map_err(UseCaseError::ValidationError)?;
    self.domain_service.validate_priority(request.priority)
        .map_err(UseCaseError::ValidationError)?;

    // Create domain entity
    let task = Task::new(TaskId::new(0), request.name, request.priority)
        .map_err(UseCaseError::ValidationError)?;

    // Persist through repository
    let task_id = self.task_repository.save(&task).await?;
    Ok(task_id.value())
}
```

**Flow:**
1. Validate input using domain service
2. Create domain entity (Task)
3. Save through repository port
4. Return generated ID

### 3. Update Task

```rust
pub async fn update_task(&self, id: i32, request: UpdateTaskRequest) -> Result<(), UseCaseError> {
    let task_id = TaskId::new(id);
    
    // Check if task exists
    let mut task = self.task_repository.find_by_id(task_id).await?
        .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;

    // Validate changes
    self.domain_service.can_update_task(&task, request.name.as_deref(), request.priority)
        .map_err(UseCaseError::ValidationError)?;

    // Apply changes using domain methods
    if let Some(name) = request.name {
        task.update_name(name).map_err(UseCaseError::ValidationError)?;
    }
    if let Some(priority) = request.priority {
        task.update_priority(Some(priority)).map_err(UseCaseError::ValidationError)?;
    }

    // Persist changes
    self.task_repository.update(&task).await?;
    Ok(())
}
```

**Flow:**
1. Retrieve existing task
2. Validate proposed changes
3. Apply changes using domain methods
4. Persist updated entity

## 📦 Data Transfer Objects (DTOs)

DTOs are used to transfer data across layer boundaries without exposing internal domain structure.

### TaskDto

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: i32,
    pub name: String,
    pub priority: Option<i32>,
}

// Conversion from domain entity to DTO
impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.value(),
            name: task.name,
            priority: task.priority,
        }
    }
}

// Conversion from DTO to domain entity
impl TryFrom<TaskDto> for Task {
    type Error = String;

    fn try_from(dto: TaskDto) -> Result<Self, Self::Error> {
        Task::new(TaskId::new(dto.id), dto.name, dto.priority)
    }
}
```

### Request DTOs

```rust
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub priority: Option<i32>,
}
```

**Benefits of DTOs:**
- ✅ Stable external API contracts
- ✅ Protect internal domain model from external changes
- ✅ Allow different representations for different contexts
- ✅ Enable serialization/deserialization

## 🚨 Error Handling

Application layer defines its own error types that aggregate domain and infrastructure errors.

### UseCaseError

```rust
#[derive(Debug)]
pub enum UseCaseError {
    ValidationError(String),
    NotFound(String),
    RepositoryError(String),
}

// Convert domain errors to application errors
impl From<RepositoryError> for UseCaseError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound(msg) => UseCaseError::NotFound(msg),
            RepositoryError::ValidationError(msg) => UseCaseError::ValidationError(msg),
            RepositoryError::DatabaseError(msg) => UseCaseError::RepositoryError(msg),
        }
    }
}
```

**Error Handling Strategy:**
- ✅ Each layer has its own error types
- ✅ Errors are converted at layer boundaries
- ✅ Higher layers never see lower-level error details
- ✅ Clear error semantics for each layer

## 🎯 Design Patterns Applied

### 1. **Use Case Pattern**
Each use case represents a single user intention:
- `get_all_tasks()` → User wants to see all tasks
- `create_task()` → User wants to create a new task
- `update_task()` → User wants to modify a task

### 2. **Repository Pattern**
Use cases interact with data through repository abstractions:
```rust
// Uses the port, not the implementation
let tasks = self.task_repository.find_all().await?;
```

### 3. **Dependency Injection**
Dependencies are injected through constructor:
```rust
let use_cases = TaskUseCases::new(task_repository);
```

### 4. **DTO Pattern**
Data crossing layer boundaries is transformed:
```rust
// Domain entity → DTO for external consumption
Ok(tasks.into_iter().map(TaskDto::from).collect())
```

## 🧪 Testing Use Cases

Use cases can be tested with mock repositories:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        TaskRepo {}
        
        #[async_trait]
        impl TaskRepository for TaskRepo {
            async fn find_all(&self) -> Result<Vec<Task>, RepositoryError>;
            async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError>;
            // ... other methods
        }
    }

    #[tokio::test]
    async fn test_create_task_success() {
        let mut mock_repo = MockTaskRepo::new();
        mock_repo
            .expect_save()
            .times(1)
            .returning(|_| Ok(TaskId::new(42)));

        let use_cases = TaskUseCases::new(Arc::new(mock_repo));
        
        let request = CreateTaskRequest {
            name: "Test Task".to_string(),
            priority: Some(5),
        };

        let result = use_cases.create_task(request).await;
        assert_eq!(result.unwrap(), 42);
    }
}
```

## 🔄 Application Flow Summary

```
1. External Request → Infrastructure Layer (Web Controller)
2. Controller → Application Layer (Use Case)
3. Use Case → Domain Layer (Entities, Domain Services)
4. Use Case → Infrastructure Layer (Repository Port)
5. Repository → Database/External Service
6. Response flows back through the same layers
```

## 🚀 Next: Infrastructure Adapters

Continue to [Infrastructure Adapters](09_infrastructure_adapters.md) to see how external dependencies are implemented.