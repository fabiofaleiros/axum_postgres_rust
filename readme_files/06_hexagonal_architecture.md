# Hexagonal Architecture Overview

This document explains the **Hexagonal Architecture** (also known as Ports and Adapters) implementation in this Rust project.

## 🎯 What is Hexagonal Architecture?

Hexagonal Architecture is a design pattern that aims to create loosely coupled application components that can be easily connected to their software environment through ports and adapters. This architecture divides an application into several loosely-coupled interchangeable components, such as the application core, the database, the user interface, test scripts and interfaces with other systems.

## 🏗️ Architecture Layers

### 1. Domain Layer (Core) 🏛️

The **innermost layer** contains pure business logic with no external dependencies.

**Components:**
- **Entities**: Core business objects (`Task`)
- **Value Objects**: Immutable objects that represent domain concepts (`TaskId`)
- **Domain Services**: Stateless services that contain domain logic
- **Ports**: Interface definitions that define contracts for external dependencies

**Key Principles:**
- ✅ No external dependencies (no database, web framework, etc.)
- ✅ Contains pure business rules and validation
- ✅ Defines interfaces (ports) for external needs
- ✅ Highly testable in isolation

### 2. Application Layer 🔄

The **orchestration layer** that coordinates domain objects and implements use cases.

**Components:**
- **Use Cases**: Application-specific business rules (`TaskUseCases`)
- **DTOs**: Data Transfer Objects for crossing layer boundaries
- **Application Services**: Coordinate domain objects to fulfill use cases

**Key Principles:**
- ✅ Orchestrates domain objects
- ✅ Implements application-specific business rules
- ✅ Depends only on domain layer
- ✅ Uses ports to communicate with infrastructure

### 3. Infrastructure Layer 🔌

The **outermost layer** that implements the ports and handles external concerns.

**Components:**
- **Adapters**: Concrete implementations of domain ports
- **Web Controllers**: HTTP request/response handling
- **Repository Implementations**: Database access logic
- **Configuration**: External configuration management

**Key Principles:**
- ✅ Implements domain ports
- ✅ Handles external dependencies (database, web, file system)
- ✅ Depends on inner layers
- ✅ Can be easily swapped without affecting business logic

## 🔄 Dependency Flow

```
Infrastructure → Application → Domain
```

- **Infrastructure** depends on **Application** and **Domain**
- **Application** depends only on **Domain**
- **Domain** depends on nothing (pure business logic)

## 🎯 Benefits in This Project

### 1. **Testability** 🧪
```rust
// Easy to test use cases with mock repositories
let mock_repository = MockTaskRepository::new();
let use_cases = TaskUseCases::new(Arc::new(mock_repository));
```

### 2. **Flexibility** 🔄
```rust
// Can easily swap PostgreSQL for MongoDB or any other database
let postgres_repo = PostgresTaskRepository::new(pool);
let mongo_repo = MongoTaskRepository::new(client);
// Both implement the same TaskRepository trait
```

### 3. **Maintainability** 🛠️
- Changes in database schema only affect the repository adapter
- Changes in web framework only affect the web adapter
- Business logic remains stable and unchanged

### 4. **Clear Boundaries** 🎯
- Each layer has a single responsibility
- Easy to understand where to place new functionality
- Prevents mixing of concerns

## 🔧 Implementation Patterns

### Dependency Injection
```rust
// main.rs - Wiring everything together
let task_repository: Arc<dyn TaskRepository> = 
    Arc::new(PostgresTaskRepository::new(db_pool));

let task_use_cases = Arc::new(TaskUseCases::new(task_repository));
let task_controller = Arc::new(TaskController::new(task_use_cases));
```

### Port Definition
```rust
// Domain layer defines the contract
#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError>;
    async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError>;
    // ... other methods
}
```

### Adapter Implementation
```rust
// Infrastructure layer implements the contract
impl TaskRepository for PostgresTaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError> {
        // PostgreSQL-specific implementation
    }
}
```

## 🚀 Next Steps

Continue reading the detailed explanations of each layer:

- [Domain Layer Design](07_domain_layer.md)
- [Application Layer & Use Cases](08_application_layer.md)
- [Infrastructure Adapters](09_infrastructure_adapters.md)
- [Dependency Injection & Testing](10_dependency_injection_testing.md)