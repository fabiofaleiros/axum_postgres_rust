use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::{TaskUseCases, CreateTaskRequest, UpdateTaskRequest, TaskDto, UseCaseError};
use crate::responses::{ApiResponse, TaskListResponse, TaskCreatedResponse};

#[derive(Deserialize)]
pub struct TaskQuery {
    priority: Option<i32>,
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
        controller.task_use_cases.update_task(task_id, request).await?;
        
        let mut data = HashMap::new();
        data.insert("message".to_string(), "Task updated successfully".to_string());
        
        let response = ApiResponse::success(data);
        Ok(Json(response))
    }

    pub async fn delete_task(
        State(controller): State<Arc<TaskController>>,
        Path(task_id): Path<i32>,
    ) -> Result<(StatusCode, Json<ApiResponse<HashMap<String, String>>>), WebError> {
        controller.task_use_cases.delete_task(task_id).await?;
        
        let mut data = HashMap::new();
        data.insert("message".to_string(), "Task deleted successfully".to_string());
        
        let response = ApiResponse::success(data);
        Ok((StatusCode::NO_CONTENT, Json(response)))
    }
}