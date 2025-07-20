use axum::{
    extract::{
        Path,
        State
    },
    http::StatusCode,
    routing::{
        get,
        patch
    },
    Json,
    Router,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use sqlx::{
    postgres::PgPoolOptions,
    PgPool
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // expose the environment variables
    dotenvy::dotenv().expect("Failed to read .env file");

    // set variables from the environment variables

    let server_address = std::env::var("SERVER_ADDRESS")
        .unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not found in the env file");

    // create the database connection pool

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");   

    // create our tcp listener

    let listener = TcpListener::bind(&server_address)
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", listener.local_addr().unwrap());

    // compose the routes

    let app = Router::new()
        .route("/", get(|| async {Json(json!({"message": "Welcome to the Axum Postgres Rust API"}))}))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/{task_id}", patch(update_task).delete(delete_task))
        .with_state(db_pool);

    // serve the application

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[derive(Serialize)]
struct TaskRow {
    task_id: i32,
    name: String,
    priority: Option<i32>,
}

async fn get_tasks(
    State(pg_pool): State<PgPool>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TaskRow, "SELECT task_id, name, priority FROM tasks order by task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
    })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "tasks": rows}).to_string(),
    ))
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    name: String,
    priority: Option<i32>,
}

#[derive(Serialize)]
struct CreateTaskResponse {
    task_id: i32,
}

async fn create_task(
    State(pg_pool): State<PgPool>,
    Json(task): Json<CreateTaskRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        CreateTaskResponse,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        task.name,
        task.priority
    ).fetch_one(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "task_id": row}).to_string(),
    ))
}

#[derive(Deserialize)]
struct UpdateTaskRequest {
  name: Option<String>,
  priority: Option<i32>,
}

async fn update_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskRequest>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query_as!(
        UpdateTaskRequest,
        "UPDATE tasks SET name = COALESCE($1, name), priority = COALESCE($2, priority) WHERE task_id = $3",
        task.name,
        task.priority,
        task_id
    ).execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "message": "Task updated successfully"}).to_string(),
    ))
}

async fn delete_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM tasks WHERE task_id = $1", task_id)
        .execute(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::NO_CONTENT,
        json!({"success": true, "message": "Task deleted successfully"}).to_string(),
    ))
}
