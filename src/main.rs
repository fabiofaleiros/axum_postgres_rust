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
}

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

    // set variables from the environment variables

    // create the database connection pool

    // create our tcp listener

    // compose the routes

    // serve the application
