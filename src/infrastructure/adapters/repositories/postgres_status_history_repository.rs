use async_trait::async_trait;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::{StatusHistory, StatusHistoryRepository, TaskAnalytics, TaskStatus, UserRole, RepositoryError};

pub struct PostgresStatusHistoryRepository {
    pool: PgPool,
}

impl PostgresStatusHistoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_status_history(&self, row: &sqlx::postgres::PgRow) -> Result<StatusHistory, RepositoryError> {
        let id: Uuid = row.get("id");
        let task_id: i32 = row.get("task_id");
        let from_status_str: Option<String> = row.get("from_status");
        let to_status_str: String = row.get("to_status");
        let changed_at: DateTime<Utc> = row.get("changed_at");
        let changed_by: String = row.get("changed_by");
        let comment: Option<String> = row.get("comment");
        let user_role_str: String = row.get("user_role");

        let from_status = if let Some(status_str) = from_status_str {
            Some(TaskStatus::from_str(&status_str)
                .map_err(|e| RepositoryError::ValidationError(e))?)
        } else {
            None
        };

        let to_status = TaskStatus::from_str(&to_status_str)
            .map_err(|e| RepositoryError::ValidationError(e))?;

        let user_role = UserRole::from_str(&user_role_str)
            .map_err(|e| RepositoryError::ValidationError(e))?;

        Ok(StatusHistory::new(
            id.to_string(),
            task_id,
            from_status,
            to_status,
            changed_at,
            changed_by,
            comment,
            user_role,
        ))
    }
}

#[async_trait]
impl StatusHistoryRepository for PostgresStatusHistoryRepository {
    async fn find_by_task_id(&self, task_id: i32) -> Result<Vec<StatusHistory>, RepositoryError> {
        let rows = sqlx::query(
            "SELECT id, task_id, from_status, to_status, changed_at, changed_by, comment, user_role 
             FROM status_history 
             WHERE task_id = $1 
             ORDER BY changed_at ASC"
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut histories = Vec::new();
        for row in rows {
            let history = self.row_to_status_history(&row)?;
            histories.push(history);
        }

        Ok(histories)
    }

    async fn find_by_date_range(
        &self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>
    ) -> Result<Vec<StatusHistory>, RepositoryError> {
        let rows = sqlx::query(
            "SELECT id, task_id, from_status, to_status, changed_at, changed_by, comment, user_role 
             FROM status_history 
             WHERE changed_at >= $1 AND changed_at <= $2 
             ORDER BY changed_at ASC"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut histories = Vec::new();
        for row in rows {
            let history = self.row_to_status_history(&row)?;
            histories.push(history);
        }

        Ok(histories)
    }

    async fn find_latest_by_task_id(&self, task_id: i32) -> Result<Option<StatusHistory>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, task_id, from_status, to_status, changed_at, changed_by, comment, user_role 
             FROM status_history 
             WHERE task_id = $1 
             ORDER BY changed_at DESC 
             LIMIT 1"
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(self.row_to_status_history(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_task_analytics(&self, task_id: i32) -> Result<Option<TaskAnalytics>, RepositoryError> {
        let histories = self.find_by_task_id(task_id).await?;
        Ok(TaskAnalytics::from_history(histories))
    }

    async fn get_completion_analytics(
        &self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>
    ) -> Result<Vec<TaskAnalytics>, RepositoryError> {
        // Get all completed tasks in the date range
        let rows = sqlx::query(
            "SELECT DISTINCT task_id 
             FROM status_history 
             WHERE to_status = 'Completed' 
             AND changed_at >= $1 AND changed_at <= $2"
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut analytics = Vec::new();
        for row in rows {
            let task_id: i32 = row.get("task_id");
            if let Some(task_analytics) = self.get_task_analytics(task_id).await? {
                analytics.push(task_analytics);
            }
        }

        Ok(analytics)
    }

    async fn get_average_completion_times(&self) -> Result<Vec<(i32, chrono::Duration)>, RepositoryError> {
        let rows = sqlx::query(
            "SELECT t.priority, 
                    AVG(EXTRACT(EPOCH FROM (sh_completed.changed_at - sh_created.changed_at))) as avg_seconds
             FROM tasks t
             JOIN status_history sh_created ON t.task_id = sh_created.task_id AND sh_created.from_status IS NULL
             JOIN status_history sh_completed ON t.task_id = sh_completed.task_id AND sh_completed.to_status = 'Completed'
             WHERE t.priority IS NOT NULL
             GROUP BY t.priority
             ORDER BY t.priority"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut results = Vec::new();
        for row in rows {
            let priority: i32 = row.get("priority");
            let avg_seconds: Option<f64> = row.get("avg_seconds");
            
            if let Some(seconds) = avg_seconds {
                let duration = chrono::Duration::seconds(seconds as i64);
                results.push((priority, duration));
            }
        }

        Ok(results)
    }

    async fn save(&self, history: &StatusHistory) -> Result<String, RepositoryError> {
        let id = Uuid::parse_str(&history.id)
            .map_err(|e| RepositoryError::ValidationError(format!("Invalid UUID: {}", e)))?;

        let from_status_str = history.from_status.as_ref().map(|s| s.as_str());

        let result = sqlx::query(
            "INSERT INTO status_history (id, task_id, from_status, to_status, changed_at, changed_by, comment, user_role)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET
                 from_status = EXCLUDED.from_status,
                 to_status = EXCLUDED.to_status,
                 changed_at = EXCLUDED.changed_at,
                 changed_by = EXCLUDED.changed_by,
                 comment = EXCLUDED.comment,
                 user_role = EXCLUDED.user_role
             RETURNING id"
        )
        .bind(id)
        .bind(history.task_id)
        .bind(from_status_str)
        .bind(history.to_status.as_str())
        .bind(history.changed_at)
        .bind(&history.changed_by)
        .bind(&history.comment)
        .bind(history.user_role.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let saved_id: Uuid = result.get("id");
        Ok(saved_id.to_string())
    }

    async fn delete(&self, id: String) -> Result<(), RepositoryError> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| RepositoryError::ValidationError(format!("Invalid UUID: {}", e)))?;

        let result = sqlx::query("DELETE FROM status_history WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound(
                format!("Status history with id {} not found", id)
            ));
        }

        Ok(())
    }
}