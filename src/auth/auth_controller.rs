use std::sync::Arc;
use axum::{Json};
use axum::extract::State;
use axum::response::{Response};
use axum_valid::{Valid};
use crate::auth::auth_service::{get_user_token, register_user};
use crate::auth::dto::{LoginDto, RegisterDto};
use crate::module::AppState;

pub async fn login(
    State(state): State<Arc<AppState>>,

    dto: Valid<Json<LoginDto>>) -> Response {
    get_user_token(&state.database, dto.into_inner(), &state.config.secret).await
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    body: Valid<Json<RegisterDto>>,
) -> Response {
    register_user(&state.database, &body.into_inner()).await
}

pub async fn request_forgot_password() {}

pub async fn set_password() {}

pub async fn get_profile() {}