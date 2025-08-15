use async_trait::async_trait;
use crate::domain::entities::Task;
use crate::domain::value_objects::TaskId;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    DatabaseError(String),
    ValidationError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Task>, RepositoryError>;
    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError>;
    async fn find_by_priority(&self, priority: i32) -> Result<Vec<Task>, RepositoryError>;
    async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError>;
    async fn update(&self, task: &Task) -> Result<(), RepositoryError>;
    async fn delete(&self, id: TaskId) -> Result<(), RepositoryError>;
}