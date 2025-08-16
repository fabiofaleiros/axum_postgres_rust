use axum::{
    routing::get,
    Json, Router,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

// Import our modules
mod domain;
mod application;
mod infrastructure;
mod config;
mod database;
mod responses;

use config::Config;
use database::Database;
use std::sync::Arc;
use domain::TaskRepository;
use application::TaskUseCases;
use infrastructure::adapters::{PostgresTaskRepository, TaskController};
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database connection pool
    let db_pool = Database::connect(&config).await?;

    // Create repository
    let task_repository: Arc<dyn TaskRepository> = Arc::new(PostgresTaskRepository::new(db_pool));
    
    // Create use cases
    let task_use_cases = Arc::new(TaskUseCases::new(task_repository));
    
    // Create controllers
    let task_controller = Arc::new(TaskController::new(task_use_cases));

    // Create TCP listener
    let listener = TcpListener::bind(&config.server_address).await?;
    println!("Server running on {}", listener.local_addr().unwrap());

    // Build router with middleware
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .route("/tasks", 
            get(TaskController::get_tasks)
            .post(TaskController::create_task)
        )
        .route("/tasks/{task_id}", 
            get(TaskController::get_task)
            .patch(TaskController::update_task)
            .delete(TaskController::delete_task)
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .with_state(task_controller);

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}

/// Root endpoint handler
async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to the Axum Postgres Rust API",
        "version": "1.0.0",
        "endpoints": {
            "tasks": "/tasks",
            "health": "/health"
        }
    }))
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
