use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::Response;
use axum_valid::Valid;
use crate::error::ErrorResponse;
use crate::module::AppState;
use crate::user::dto::UpdateUserDto;
use crate::user::user_service;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    user_service::get_one_user(&state.database, &user_id).await
}


pub async fn get_users(
    State(state): State<Arc<AppState>>
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    user_service::get_all_users(&state.database).await
}


pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    user_service::delete_user(&state.database, &user_id).await
}

pub async fn edit_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    dto: Valid<Json<UpdateUserDto>>,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    user_service::edit_user(&state.database, &user_id, dto.into_inner()).await
}