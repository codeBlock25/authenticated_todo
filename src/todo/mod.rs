use std::sync::Arc;
use axum::{middleware, Router};
use axum::routing::{post, get, patch, put, delete};
use crate::auth::guard::auth;
use crate::module::AppState;
use crate::todo::todo_controller::{create_todo, delete_todo, get_todo, get_todos, toggle_todo, update_todo};

mod todo_service;
mod todo_controller;

pub(crate) mod entity;

mod dto;

pub fn todo_module(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/todo", post(create_todo))
        .route("/api/todo", get(get_todos))
        .route("/api/todo/:user_id", get(get_todo))
        .route("/api/todo/:user_id", patch(update_todo))
        .route("/api/todo/:user_id", put(toggle_todo))
        .route("/api/todo/:user_id", delete(delete_todo))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}