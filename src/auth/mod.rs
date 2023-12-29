use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};
use crate::auth::auth_controller::{get_profile, login, register, request_forgot_password, set_password};
use crate::module::AppState;

mod auth_controller;
mod dto;
mod auth_service;
pub mod guard;

pub fn auth_module(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/request_forgot_password", patch(request_forgot_password))
        .route("/api/auth/set_password", patch(set_password))
        .route("/api/auth/profile", get(get_profile))
        .with_state(state)
}