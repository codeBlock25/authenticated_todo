mod user_service;
mod user_controller;
mod dto;

pub(crate) mod entity;

use std::sync::Arc;
use axum::{middleware, Router};
use axum::routing::{delete, get, patch};
use crate::auth::guard::auth;
use crate::module::AppState;
use crate::user::user_controller::{delete_user, edit_user, get_user, get_users};


pub fn user_module(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/user", get(get_users))
        .route("/api/user/:id", patch(edit_user))
        .route("/api/user/:id", get(get_user))
        .route("/api/user/:id", delete(delete_user))
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state(state)
}