use std::sync::Arc;
use crate::domain::{Task, TaskId, TaskRepository, TaskDomainService, TaskStatusService, UserRole, RepositoryError};
use crate::application::dto::{TaskDto, CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusDto, TaskWithTransitionsDto};

#[derive(Debug, Clone)]
pub enum UseCaseError {
    ValidationError(String),
    NotFound(String),
    RepositoryError(String),
}

impl From<RepositoryError> for UseCaseError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound(msg) => UseCaseError::NotFound(msg),
            RepositoryError::ValidationError(msg) => UseCaseError::ValidationError(msg),
            RepositoryError::DatabaseError(msg) => UseCaseError::RepositoryError(msg),
        }
    }
}

impl std::fmt::Display for UseCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseCaseError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            UseCaseError::NotFound(msg) => write!(f, "Not found: {}", msg),
            UseCaseError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}

impl std::error::Error for UseCaseError {}

pub struct TaskUseCases {
    task_repository: Arc<dyn TaskRepository>,
    domain_service: TaskDomainService,
    status_service: TaskStatusService,
}

impl TaskUseCases {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self {
            task_repository,
            domain_service: TaskDomainService::new(),
            status_service: TaskStatusService::new(),
        }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<TaskDto>, UseCaseError> {
        let tasks = self.task_repository.find_all().await?;
        Ok(tasks.into_iter().map(TaskDto::from).collect())
    }

    pub async fn get_task_by_id(&self, id: i32) -> Result<TaskDto, UseCaseError> {
        let task_id = TaskId::new(id);
        let task = self.task_repository.find_by_id(task_id).await?
            .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;
        Ok(TaskDto::from(task))
    }

    pub async fn get_tasks_by_priority(&self, priority: i32) -> Result<Vec<TaskDto>, UseCaseError> {
        self.domain_service.validate_priority(Some(priority))
            .map_err(UseCaseError::ValidationError)?;
        
        let tasks = self.task_repository.find_by_priority(priority).await?;
        Ok(tasks.into_iter().map(TaskDto::from).collect())
    }

    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<i32, UseCaseError> {
        self.domain_service.validate_task_name(&request.name)
            .map_err(UseCaseError::ValidationError)?;
        self.domain_service.validate_priority(request.priority)
            .map_err(UseCaseError::ValidationError)?;

        let task = Task::new(TaskId::new(0), request.name, request.priority)
            .map_err(UseCaseError::ValidationError)?;

        let task_id = self.task_repository.save(&task).await?;
        Ok(task_id.value())
    }

    pub async fn update_task(&self, id: i32, request: UpdateTaskRequest) -> Result<(), UseCaseError> {
        let task_id = TaskId::new(id);
        let mut task = self.task_repository.find_by_id(task_id).await?
            .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;

        self.domain_service.can_update_task(&task, request.name.as_deref(), request.priority)
            .map_err(UseCaseError::ValidationError)?;

        if let Some(name) = request.name {
            task.update_name(name).map_err(UseCaseError::ValidationError)?;
        }

        if let Some(priority) = request.priority {
            task.update_priority(Some(priority)).map_err(UseCaseError::ValidationError)?;
        }

        self.task_repository.update(&task).await?;
        Ok(())
    }

    pub async fn delete_task(&self, id: i32) -> Result<(), UseCaseError> {
        let task_id = TaskId::new(id);
        
        // Check if task exists
        self.task_repository.find_by_id(task_id).await?
            .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;

        self.task_repository.delete(task_id).await?;
        Ok(())
    }

    pub async fn update_task_status(&self, id: i32, request: UpdateTaskStatusDto) -> Result<TaskDto, UseCaseError> {
        let task_id = TaskId::new(id);
        let mut task = self.task_repository.find_by_id(task_id).await?
            .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;

        // For now, default to User role. TODO: Extract from JWT token
        let user_role = UserRole::User;

        // Validate the transition using the status service
        self.status_service.can_transition(
            task.status(),
            &request.status,
            task.is_high_priority(),
            &user_role,
        ).map_err(UseCaseError::ValidationError)?;

        // Apply the status transition with role validation
        task.transition_to_with_role(request.status, &user_role).map_err(UseCaseError::ValidationError)?;

        // Save the updated task
        self.task_repository.update(&task).await?;
        
        Ok(TaskDto::from(task))
    }

    pub async fn get_task_with_transitions(&self, id: i32) -> Result<TaskWithTransitionsDto, UseCaseError> {
        let task_id = TaskId::new(id);
        let task = self.task_repository.find_by_id(task_id).await?
            .ok_or_else(|| UseCaseError::NotFound(format!("Task with id {} not found", id)))?;

        // For now, default to User role. TODO: Extract from JWT token
        let user_role = UserRole::User;

        // Use the status service to get valid transitions based on business rules
        let valid_transitions = self.status_service.get_valid_transitions(
            task.status(),
            task.is_high_priority(),
            &user_role,
        );

        Ok(TaskWithTransitionsDto {
            task: TaskDto::from(task),
            valid_transitions,
        })
    }
}

