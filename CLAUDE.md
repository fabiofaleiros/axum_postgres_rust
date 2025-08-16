# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust REST API for task management built using **Hexagonal Architecture** (Ports and Adapters pattern). The project demonstrates clean architecture principles with clear separation between domain logic, application use cases, and infrastructure concerns.

## Common Development Commands

### Building and Running
- `cargo run` - Run the application (requires PostgreSQL running)
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version
- `cargo check` - Fast compile check without producing executables

### Testing
- `cargo test` - Run all tests
- `cargo test unit` - Run unit tests only  
- `cargo test integration` - Run integration tests only
- `cargo test domain` - Run domain layer tests only

### Database Operations
- `docker-compose up -d postgres` - Start PostgreSQL database only
- `docker-compose up` - Start both database and application
- `docker-compose down` - Stop all services

### Development Workflow
- `cargo watch -x run` - Auto-restart on file changes (if cargo-watch is installed)
- `cargo clippy` - Run linter for code quality
- `cargo fmt` - Format code

## Architecture Overview

The project follows **Hexagonal Architecture** with three main layers:

### Domain Layer (`src/domain/`)
- **Entities**: Core business objects (`task.rs` - Task entity with validation)
- **Value Objects**: Domain primitives (`task_id.rs` - TaskId wrapper)
- **Ports**: Interface definitions (`task_repository.rs` - Repository trait)
- **Services**: Domain logic (`task_domain_service.rs` - Business rules)

### Application Layer (`src/application/`)
- **Use Cases**: Application orchestration (`task_use_cases.rs` - CRUD operations)
- **DTOs**: Data transfer objects (`task_dto.rs` - API data structures)

### Infrastructure Layer (`src/infrastructure/`)
- **Web Adapters**: HTTP handling (`task_controller.rs` - Axum routes)
- **Repository Adapters**: Database implementations (`postgres_task_repository.rs` - SQLx)
- **Persistence**: Database schema and migrations

## Key Design Patterns

### Dependency Injection
The application uses constructor injection with `Arc<dyn Trait>` for shared ownership:
```rust
// Repository is injected into use cases
let task_use_cases = TaskUseCases::new(task_repository);
// Use cases are injected into controllers  
let task_controller = TaskController::new(task_use_cases);
```

### Port-Adapter Pattern
- Domain defines `TaskRepository` trait (port)
- Infrastructure provides `PostgresTaskRepository` implementation (adapter)
- Easy to swap implementations for testing or different databases

### Error Handling
- Domain uses `Result<T, String>` for business rule violations
- Repository uses `Result<T, RepositoryError>` for data access errors
- Controllers map errors to appropriate HTTP responses

## Testing Strategy

The project includes comprehensive test coverage:
- **Unit Tests**: Test individual components in isolation with mocks
- **Integration Tests**: Test complete workflows with real database
- **Domain Tests**: Verify business rules and entity validation

Use `mockall` crate for mocking repository implementations in tests.

## Database Configuration

PostgreSQL connection configured via environment variables:
- `DATABASE_URL`: Connection string (default: postgres://root:1234@localhost:5432/axum_postgres)
- `SERVER_ADDRESS`: Server bind address (default: 127.0.0.1:7878)

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | API information |
| GET | `/health` | Health check |
| GET | `/tasks` | Get all tasks |
| GET | `/tasks?priority=N` | Filter tasks by priority |
| GET | `/tasks/{id}` | Get task by ID |
| POST | `/tasks` | Create new task |
| PATCH | `/tasks/{id}` | Update task |
| DELETE | `/tasks/{id}` | Delete task |

## Development Guidelines

### Adding New Features
1. Start with domain layer - define entities and business rules
2. Add repository methods to the trait if data access is needed
3. Implement use cases in application layer
4. Create/update DTOs for data transfer
5. Add controller endpoints in infrastructure layer
6. Write tests at each layer

### Code Organization
- Keep domain layer pure - no external dependencies
- Use `async-trait` for async trait methods
- Follow existing error handling patterns
- Maintain clear boundaries between layers

### Dependencies
- **Web**: Axum framework with Tower middleware
- **Database**: SQLx with PostgreSQL driver and compile-time query verification
- **Async**: Tokio runtime
- **Serialization**: Serde for JSON handling
- **Testing**: Mockall for mocking, tokio-test for async tests
- **Error Handling**: thiserror for custom error types

This architecture ensures testability, maintainability, and flexibility when evolving the codebase.