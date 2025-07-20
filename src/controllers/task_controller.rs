use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::TaskService;
use crate::errors::AppError;
use crate::responses::{ApiResponse, TaskListResponse, TaskCreatedResponse};

/// Query parameters for filtering tasks
#[derive(Deserialize)]
pub struct TaskQuery {
    priority: Option<i32>,
}

/// Controller for handling task-related HTTP requests
pub struct TaskController;

impl TaskController {
    /// Handles GET /tasks - retrieves all tasks or filters by priority
    pub async fn get_tasks(
        State(task_service): State<TaskService>,
        Query(params): Query<TaskQuery>,
    ) -> Result<Json<ApiResponse<TaskListResponse>>, AppError> {
        let tasks = match params.priority {
            Some(priority) => task_service.get_tasks_by_priority(priority).await?,
            None => task_service.get_all_tasks().await?,
        };

        let response = ApiResponse::success(TaskListResponse { tasks });
        Ok(Json(response))
    }

    /// Handles GET /tasks/{id} - retrieves a specific task
    pub async fn get_task(
        State(task_service): State<TaskService>,
        Path(task_id): Path<i32>,
    ) -> Result<Json<ApiResponse<crate::models::Task>>, AppError> {
        let task = task_service.get_task_by_id(task_id).await?;
        let response = ApiResponse::success(task);
        Ok(Json(response))
    }

    /// Handles POST /tasks - creates a new task
    pub async fn create_task(
        State(task_service): State<TaskService>,
        Json(request): Json<CreateTaskRequest>,
    ) -> Result<(StatusCode, Json<ApiResponse<TaskCreatedResponse>>), AppError> {
        let created_task = task_service.create_task(request).await?;
        let response = ApiResponse::success(TaskCreatedResponse {
            task_id: created_task.task_id,
            message: "Task created successfully".to_string(),
        });
        Ok((StatusCode::CREATED, Json(response)))
    }

    /// Handles PATCH /tasks/{id} - updates an existing task
    pub async fn update_task(
        State(task_service): State<TaskService>,
        Path(task_id): Path<i32>,
        Json(request): Json<UpdateTaskRequest>,
    ) -> Result<Json<ApiResponse<HashMap<String, String>>>, AppError> {
        task_service.update_task(task_id, request).await?;
        
        let mut data = HashMap::new();
        data.insert("message".to_string(), "Task updated successfully".to_string());
        
        let response = ApiResponse::success(data);
        Ok(Json(response))
    }

    /// Handles DELETE /tasks/{id} - deletes a task
    pub async fn delete_task(
        State(task_service): State<TaskService>,
        Path(task_id): Path<i32>,
    ) -> Result<(StatusCode, Json<ApiResponse<HashMap<String, String>>>), AppError> {
        task_service.delete_task(task_id).await?;
        
        let mut data = HashMap::new();
        data.insert("message".to_string(), "Task deleted successfully".to_string());
        
        let response = ApiResponse::success(data);
        Ok((StatusCode::NO_CONTENT, Json(response)))
    }
}