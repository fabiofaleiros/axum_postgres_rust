# Domain Layer Design

The **Domain Layer** is the heart of our hexagonal architecture, containing pure business logic with no external dependencies.

## Structure Overview

```
src/domain/
├── entities/          # Core business objects
│   └── task.rs       # Task entity with business rules
├── value_objects/     # Immutable domain concepts
│   └── task_id.rs    # TaskId value object
├── services/          # Domain services
│   └── task_domain_service.rs  # Business validation rules
└── ports/            # Interface definitions
    └── repositories/
        └── task_repository.rs  # Repository contract
```

## Entities

Entities are the core business objects that have identity and lifecycle.

### Task Entity

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub priority: Option<i32>,
}

impl Task {
    pub fn new(id: TaskId, name: String, priority: Option<i32>) -> Result<Self, String> {
        // Business validation rules
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }

        Ok(Task {
            id,
            name: name.trim().to_string(),
            priority,
        })
    }

    // Business methods
    pub fn update_name(&mut self, name: String) -> Result<(), String> { /* ... */ }
    pub fn update_priority(&mut self, priority: Option<i32>) -> Result<(), String> { /* ... */ }
}
```

**Key Characteristics:**
- Contains business validation rules
- Encapsulates business behavior
- Immutable creation through constructor
- No external dependencies

## Value Objects

Value objects are immutable objects that represent domain concepts without identity.

### TaskId Value Object

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(i32);

impl TaskId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
```

**Key Characteristics:**
- Immutable
- Equality based on value, not identity
- Type safety (prevents mixing up IDs)
- Lightweight and copyable

## Domain Services

Domain services contain business logic that doesn't naturally belong to any specific entity.

### TaskDomainService

```rust
pub struct TaskDomainService;

impl TaskDomainService {
    pub fn validate_task_name(&self, name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Task name cannot be empty".to_string());
        }
        if name.len() > 255 {
            return Err("Task name cannot exceed 255 characters".to_string());
        }
        Ok(())
    }

    pub fn validate_priority(&self, priority: Option<i32>) -> Result<(), String> {
        if let Some(priority) = priority {
            if priority < 1 || priority > 10 {
                return Err("Priority must be between 1 and 10".to_string());
            }
        }
        Ok(())
    }
}
```

**When to Use Domain Services:**
- Business logic that involves multiple entities
- Validation that spans multiple objects
- Domain calculations that don't belong to a specific entity
- Stateless operations

## Ports (Interfaces)

Ports define contracts for external dependencies without knowing their implementation.

### TaskRepository Port

```rust
#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError>;
    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError>;
    async fn find_by_priority(&self, priority: i32) -> Result<Vec<Task>, RepositoryError>;
    async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError>;
    async fn update(&self, task: &Task) -> Result<(), RepositoryError>;
    async fn delete(&self, id: TaskId) -> Result<(), RepositoryError>;
}
```

**Key Characteristics:**
- Defines what operations are needed
- Does not specify how they are implemented
- Uses domain types (Task, TaskId)
- Async-friendly for modern Rust applications

### Domain-Specific Errors

```rust
#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    DatabaseError(String),
    ValidationError(String),
}
```

**Benefits:**
- Domain-specific error types
- No external library dependencies
- Clear error semantics

## Design Principles Applied

### 1. **Single Responsibility Principle**
Each domain object has one clear purpose:
- `Task` → Represents a task with business rules
- `TaskId` → Represents task identity
- `TaskDomainService` → Handles cross-cutting validation

### 2. **Dependency Inversion Principle**
- Domain defines interfaces (ports)
- Infrastructure implements these interfaces
- Domain never depends on infrastructure

### 3. **Domain-Driven Design**
- Rich domain model with behavior
- Ubiquitous language reflected in code
- Business rules centralized in domain

### 4. **Immutability & Validation**
- Value objects are immutable
- Entities validate invariants on creation
- Controlled mutation through business methods

## Testing Domain Logic

The domain layer is easily testable in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_creation_validates_name() {
        let result = Task::new(TaskId::new(1), "".to_string(), None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task name cannot be empty");
    }

    #[test]
    fn task_creation_validates_priority() {
        let result = Task::new(TaskId::new(1), "Valid Name".to_string(), Some(15));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Priority must be between 1 and 10");
    }

    #[test]
    fn valid_task_creation_succeeds() {
        let task = Task::new(TaskId::new(1), "Valid Task".to_string(), Some(5)).unwrap();
        assert_eq!(task.name, "Valid Task");
        assert_eq!(task.priority, Some(5));
    }
}
```

## Next: Application Layer

Continue to [Application Layer & Use Cases](08_application_layer.md) to see how the domain is orchestrated.