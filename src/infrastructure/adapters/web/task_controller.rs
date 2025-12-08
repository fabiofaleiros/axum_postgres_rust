use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::{TaskUseCases, CreateTaskRequest, UpdateTaskRequest, UpdateTaskStatusDto, TaskDto, TaskWithTransitionsDto, TaskHistoryDto, TaskAnalyticsDto, CompletionAnalyticsDto, UseCaseError};
use chrono::{DateTime, Utc};
use crate::responses::{ApiResponse, TaskListResponse, TaskCreatedResponse};

#[derive(Deserialize)]
pub struct TaskQuery {
    priority: Option<i32>,
}

impl TaskQuery {
    /// Validates the query parameters at the web boundary
    pub fn validate(&self) -> Result<(), WebError> {
        if let Some(priority) = self.priority {
            // Validate priority range (1-5 is typical for task priorities)
            if priority < 1 || priority > 5 {
                return Err(WebError::ValidationError(
                    format!("Priority must be between 1 and 5, got {}", priority)
                ));
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct AnalyticsQuery {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

impl AnalyticsQuery {
    /// Validates the analytics query parameters at the web boundary
    pub fn validate(&self) -> Result<(), WebError> {
        // If both dates are provided, ensure start_date is before end_date
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if start > end {
                return Err(WebError::ValidationError(
                    "start_date must be before or equal to end_date".to_string()
                ));
            }
        }

        // Validate dates are not in the future (analytics for future doesn't make sense)
        let now = Utc::now();
        if let Some(end) = self.end_date {
            if end > now {
                return Err(WebError::ValidationError(
                    "end_date cannot be in the future".to_string()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum WebError {
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl From<UseCaseError> for WebError {
    fn from(error: UseCaseError) -> Self {
        match error {
            UseCaseError::ValidationError(msg) => WebError::ValidationError(msg),
            UseCaseError::NotFound(msg) => WebError::NotFound(msg),
            UseCaseError::RepositoryError(msg) => WebError::InternalError(msg),
        }
    }
}

impl axum::response::IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            WebError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            WebError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            WebError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let error_response = ApiResponse::<()>::error(message);
        (status, Json(error_response)).into_response()
    }
}

/// Validates that a task ID is positive (basic sanity check at web boundary)
fn validate_task_id(task_id: i32) -> Result<(), WebError> {
    if task_id <= 0 {
        return Err(WebError::ValidationError(
            format!("Task ID must be a positive integer, got {}", task_id)
        ));
    }
    Ok(())
}

pub struct TaskController {
    task_use_cases: Arc<TaskUseCases>,
}

impl TaskController {
    pub fn new(task_use_cases: Arc<TaskUseCases>) -> Self {
        Self { task_use_cases }
    }

    pub async fn get_tasks(
        State(controller): State<Arc<TaskController>>,
        Query(params): Query<TaskQuery>,
    ) -> Result<Json<ApiResponse<TaskListResponse>>, WebError> {
        // Validate query parameters at web boundary
        params.validate()?;

        let tasks = match params.priority {
            Some(priority) => controller.task_use_cases.get_tasks_by_priority(priority).await?,
            None => controller.task_use_cases.get_all_tasks().await?,
        };

        let response = ApiResponse::success(TaskListResponse { tasks });
        Ok(Json(response))
    }

    pub async fn get_task(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<Json<ApiResponse<TaskDto>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        let task = controller.task_use_cases.get_task_by_id(task_id).await?;
        let response = ApiResponse::success(task);
        Ok(Json(response))
    }

    pub async fn create_task(
        State(controller): State<Arc<TaskController>>,
        Json(request): Json<CreateTaskRequest>,
    ) -> Result<(StatusCode, Json<ApiResponse<TaskCreatedResponse>>), WebError> {
        let task_id = controller.task_use_cases.create_task(request).await?;
        let response = ApiResponse::success(TaskCreatedResponse {
            task_id,
            message: "Task created successfully".to_string(),
        });
        Ok((StatusCode::CREATED, Json(response)))
    }

    pub async fn update_task(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
        Json(request): Json<UpdateTaskRequest>,
    ) -> Result<Json<ApiResponse<HashMap<String, String>>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        controller.task_use_cases.update_task(task_id, request).await?;

        let mut data = HashMap::new();
        data.insert("message".to_string(), "Task updated successfully".to_string());

        let response = ApiResponse::success(data);
        Ok(Json(response))
    }

    pub async fn delete_task(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<StatusCode, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        controller.task_use_cases.delete_task(task_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    pub async fn update_task_status(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
        Json(request): Json<UpdateTaskStatusDto>,
    ) -> Result<Json<ApiResponse<TaskDto>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        let task = controller.task_use_cases.update_task_status(task_id, request).await?;
        let response = ApiResponse::success(task);
        Ok(Json(response))
    }

    pub async fn get_task_with_transitions(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<Json<ApiResponse<TaskWithTransitionsDto>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        let result = controller.task_use_cases.get_task_with_transitions(task_id).await?;
        let response = ApiResponse::success(result);
        Ok(Json(response))
    }

    pub async fn get_task_history(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<Json<ApiResponse<TaskHistoryDto>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        let history = controller.task_use_cases.get_task_history(task_id).await?;
        let response = ApiResponse::success(history);
        Ok(Json(response))
    }

    pub async fn get_task_analytics(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<Json<ApiResponse<TaskAnalyticsDto>>, WebError> {
        // Validate path parameter at web boundary
        validate_task_id(task_id)?;

        let analytics = controller.task_use_cases.get_task_analytics(task_id).await?;
        let response = ApiResponse::success(analytics);
        Ok(Json(response))
    }

    pub async fn get_completion_analytics(
        State(controller): State<Arc<TaskController>>,
        Query(params): Query<AnalyticsQuery>,
    ) -> Result<Json<ApiResponse<CompletionAnalyticsDto>>, WebError> {
        // Validate query parameters at web boundary
        params.validate()?;

        let analytics = controller.task_use_cases
            .get_completion_analytics(params.start_date, params.end_date)
            .await?;
        let response = ApiResponse::success(analytics);
        Ok(Json(response))
    }
}

