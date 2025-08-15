use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::{Task, TaskId, TaskRepository, RepositoryError};

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
            
            let task = Task::new(
                TaskId::new(task_id),
                name,
                priority,
            ).map_err(RepositoryError::ValidationError)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, RepositoryError> {
        let row = sqlx::query("SELECT task_id, name, priority FROM tasks WHERE task_id = $1")
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let task_id: i32 = row.get("task_id");
                let name: String = row.get("name");
                let priority: Option<i32> = row.get("priority");
                
                let task = Task::new(
                    TaskId::new(task_id),
                    name,
                    priority,
                ).map_err(RepositoryError::ValidationError)?;
                Ok(Some(task))
            }
            None => Ok(None),
        }
    }

    async fn find_by_priority(&self, priority: i32) -> Result<Vec<Task>, RepositoryError> {
        let rows = sqlx::query("SELECT task_id, name, priority FROM tasks WHERE priority = $1 ORDER BY task_id")
            .bind(priority)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut tasks = Vec::new();
        for row in rows {
            let task_id: i32 = row.get("task_id");
            let name: String = row.get("name");
            let priority: Option<i32> = row.get("priority");
            
            let task = Task::new(
                TaskId::new(task_id),
                name,
                priority,
            ).map_err(RepositoryError::ValidationError)?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    async fn save(&self, task: &Task) -> Result<TaskId, RepositoryError> {
        let row = sqlx::query("INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id")
            .bind(&task.name)
            .bind(task.priority)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let task_id: i32 = row.get("task_id");
        Ok(TaskId::new(task_id))
    }

    async fn update(&self, task: &Task) -> Result<(), RepositoryError> {
        let result = sqlx::query("UPDATE tasks SET name = $1, priority = $2 WHERE task_id = $3")
            .bind(&task.name)
            .bind(task.priority)
            .bind(task.id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(
                format!("Task with id {} not found", task.id.value())
            ));
        }

        Ok(())
    }

    async fn delete(&self, id: TaskId) -> Result<(), RepositoryError> {
        let result = sqlx::query("DELETE FROM tasks WHERE task_id = $1")
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(
                format!("Task with id {} not found", id.value())
            ));
        }

        Ok(())
    }
}