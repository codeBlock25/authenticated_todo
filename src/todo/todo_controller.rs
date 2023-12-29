use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::Response;
use axum_valid::Valid;
use crate::error::ErrorResponse;
use crate::module::AppState;
use crate::todo::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todo::todo_service;
use crate::user::entity::users;

pub async fn get_todos(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)>
{ todo_service::get_all_todo(&state.database, &user.id.to_string()).await }

pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
    Path(user_id): Path<String>,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)>
{ todo_service::get_one_todo(&state.database, &user_id, &user.id.to_string()).await }

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
    dto: Valid<Json<CreateTodoDto>>,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)>
{
    todo_service::create(&state.database, &dto, &user).await
}

pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
    Path(todo_id): Path<String>,
    dto: Valid<Json<UpdateTodoDto>>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    todo_service::update_todo(&state.database, &todo_id, &user.id.to_string(), &dto).await
}

pub async fn toggle_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
    Path(todo_id): Path<String>,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    todo_service::toggle_todo(&state.database, &todo_id, &user.id.to_string()).await
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<users::Model>,
    Path(todo_id): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    todo_service::delete_todo(&state.database, &todo_id, &user.id.to_string()).await
}