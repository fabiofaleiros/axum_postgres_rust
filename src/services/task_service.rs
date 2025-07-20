use sqlx::PgPool;
use crate::models::{Task, CreateTaskRequest, UpdateTaskRequest, CreateTaskResponse};
use crate::errors::AppError;

/// Service layer for task-related business logic
#[derive(Clone)]
pub struct TaskService {
    db_pool: PgPool,
}

impl TaskService {
    /// Creates a new TaskService instance
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    /// Retrieves all tasks from the database
    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query_as!(
            Task,
            "SELECT task_id, name, priority FROM tasks ORDER BY task_id"
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(AppError::Database)?;

        // Validate each task before returning
        for task in &rows {
            task.validate().map_err(AppError::ValidationError)?;
        }

        Ok(rows)
    }

    /// Creates a new task in the database
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<CreateTaskResponse, AppError> {
        // Validate the request
        request.validate().map_err(AppError::ValidationError)?;

        let row = sqlx::query_as!(
            CreateTaskResponse,
            "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
            request.name,
            request.priority
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row)
    }

    /// Updates an existing task in the database
    pub async fn update_task(&self, task_id: i32, request: UpdateTaskRequest) -> Result<(), AppError> {
        // Check if task exists first
        let exists = sqlx::query!("SELECT task_id FROM tasks WHERE task_id = $1", task_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(AppError::Database)?;

        if exists.is_none() {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        // Validate update data if present
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError("Task name cannot be empty".to_string()));
            }
        }

        if let Some(priority) = request.priority {
            if priority < 1 || priority > 10 {
                return Err(AppError::ValidationError("Priority must be between 1 and 10".to_string()));
            }
        }

        let result = sqlx::query!(
            "UPDATE tasks SET name = COALESCE($1, name), priority = COALESCE($2, priority) WHERE task_id = $3",
            request.name,
            request.priority,
            task_id
        )
        .execute(&self.db_pool)
        .await
        .map_err(AppError::Database)?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        Ok(())
    }

    /// Deletes a task from the database
    pub async fn delete_task(&self, task_id: i32) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM tasks WHERE task_id = $1", task_id)
            .execute(&self.db_pool)
            .await
            .map_err(AppError::Database)?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        Ok(())
    }

    /// Gets a specific task by ID
    pub async fn get_task_by_id(&self, task_id: i32) -> Result<Task, AppError> {
        let task = sqlx::query_as!(
            Task,
            "SELECT task_id, name, priority FROM tasks WHERE task_id = $1",
            task_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(AppError::Database)?;

        match task {
            Some(task) => {
                // Validate the task before returning
                task.validate().map_err(AppError::ValidationError)?;
                Ok(task)
            },
            None => Err(AppError::NotFound(format!("Task with id {} not found", task_id)))
        }
    }

    /// Gets tasks filtered by priority
    pub async fn get_tasks_by_priority(&self, priority: i32) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query_as!(
            Task,
            "SELECT task_id, name, priority FROM tasks WHERE priority = $1 ORDER BY task_id",
            priority
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(AppError::Database)?;

        // Validate each task before returning
        for task in &rows {
            task.validate().map_err(AppError::ValidationError)?;
        }

        Ok(rows)
    }
}
