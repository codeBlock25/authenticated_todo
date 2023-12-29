use std::sync::Arc;
use axum::http::Method;
use axum::{Json, Router};
use axum::routing::get;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use crate::auth::auth_module;
use crate::user::user_module;
use serde_json::{json, Value};
use crate::config::AppEnv;
use crate::todo::todo_module;

#[derive(Clone)]
pub struct AppState {
    pub config: AppEnv,
    pub database: DatabaseConnection,
}

pub async fn app_module(config: &AppEnv) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::PUT])
        .allow_origin(Any);
    let mut db_options = ConnectOptions::new(&config.database);
    db_options.max_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let database_pool = Database::connect(db_options).await.expect("Ensure database url is active");
    let state = Arc::new(AppState {
        database: database_pool.clone(),
        config: config.clone(),
    });
    info!("Database connected successfully");
    Router::new()
        .route("/api/status", get(app_status))
        .route_layer(cors)
        .with_state(state.clone())
        .merge(user_module(state.clone()))
        .merge(auth_module(state.clone()))
        .merge(todo_module(state.clone()))
}

async fn app_status() -> Json<Value> {
    Json(json!({
    "message": "server running well"
    }))
}