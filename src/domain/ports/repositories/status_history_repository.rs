use async_trait::async_trait;
use crate::domain::{StatusHistory, TaskAnalytics, RepositoryError};
use chrono::{DateTime, Utc};

#[async_trait]
pub trait StatusHistoryRepository: Send + Sync {
    /// Get all status history entries for a specific task
    async fn find_by_task_id(&self, task_id: i32) -> Result<Vec<StatusHistory>, RepositoryError>;
    
    /// Get status history entries within a date range
    async fn find_by_date_range(
        &self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>
    ) -> Result<Vec<StatusHistory>, RepositoryError>;
    
    /// Get the most recent status change for a task
    async fn find_latest_by_task_id(&self, task_id: i32) -> Result<Option<StatusHistory>, RepositoryError>;
    
    /// Get analytics for a specific task
    async fn get_task_analytics(&self, task_id: i32) -> Result<Option<TaskAnalytics>, RepositoryError>;
    
    /// Get analytics for all completed tasks within a date range
    async fn get_completion_analytics(
        &self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>
    ) -> Result<Vec<TaskAnalytics>, RepositoryError>;
    
    /// Get average completion times by priority level
    async fn get_average_completion_times(&self) -> Result<Vec<(i32, chrono::Duration)>, RepositoryError>;
    
    /// Manual entry for status history (for corrections or bulk imports)
    async fn save(&self, history: &StatusHistory) -> Result<String, RepositoryError>;
    
    /// Delete status history (admin operation)
    async fn delete(&self, id: String) -> Result<(), RepositoryError>;
}