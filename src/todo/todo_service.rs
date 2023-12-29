use axum::http::StatusCode;
use axum::{Json};
use axum::response::{IntoResponse, Response};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, TryIntoModel};
use serde_json::json;
use uuid::{Uuid};
use crate::error::ErrorResponse;
use crate::todo::dto::{CreateTodoDto, UpdateTodoDto};
use crate::todo::entity::todos;
use crate::user::entity::users;


pub async fn get(
    database: &DatabaseConnection,
    todo_id: &String,
    user_id: &String,
) -> Result<todos::Model, (StatusCode, Json<ErrorResponse>)> {
    Ok(todos::Entity::find()
        .filter(todos::Column::DeletedOn.is_null())
        .filter(todos::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).map_err(|_| {
            (StatusCode::NOT_FOUND, Json(ErrorResponse {
                message: "Invalid record",
                error: StatusCode::BAD_REQUEST.to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            }))
        })?)
        )
        .filter(todos::Column::Id.eq(Uuid::parse_str(todo_id.as_str())
            .map_err(|_| {
                (StatusCode::NOT_FOUND, Json(ErrorResponse {
                    message: "Invalid todo id",
                    error: StatusCode::BAD_REQUEST.to_string(),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                }))
            })?))
        .reverse_join(users::Entity)
        .one(database)
        .await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error loading todo",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(ErrorResponse {
            message: "todo not found",
            error: StatusCode::NOT_FOUND.to_string(),
            status_code: StatusCode::NOT_FOUND.as_u16(),
        }))
    })?)
}


pub async fn get_one_todo(
    database: &DatabaseConnection,
    todo_id: &String,
    user_id: &String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let todo = get(database, todo_id, user_id).await?;
    Ok((StatusCode::OK, Json(json!({
        "data": todo,
     }))).into_response())
}

pub async fn get_all_todo(
    database: &DatabaseConnection,
    user_id: &String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let todos = todos::Entity::find()
        .filter(todos::Column::DeletedOn.is_null())
        .filter(todos::Column::UserId.eq(Uuid::parse_str(user_id.as_str()).map_err(|_| {
            (StatusCode::NOT_FOUND, Json(ErrorResponse {
                message: "Invalid record",
                error: StatusCode::BAD_REQUEST.to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            }))
        })?))
        .all(database)
        .await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error loading todos",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({
            "data": todos
        }))
    ).into_response())
}

pub async fn create(db_pool: &DatabaseConnection, dto: &CreateTodoDto, user: &users::Model)
                    -> Result<Response, (StatusCode, Json<ErrorResponse>)>
{
    let todo = todos::ActiveModel {
        title: Set(dto.title.to_owned().unwrap()),
        description: Set(dto.description.to_owned()),
        user_id: Set(user.id),
        ..Default::default()
    };
    let new_todo = todo.save(db_pool)
        .await
        .map_err(|error| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                message: "Error creating todo",
                error: error.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }))
        })?;
    Ok((
        StatusCode::CREATED,
        Json(new_todo.try_into_model().map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                message: "Error returning saved todo",
                error: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }))
        })?)
    ).into_response())
}

pub async fn update_todo(
    db_pool: &DatabaseConnection,
    todo_id: &String,
    user_id: &String,
    dto: &UpdateTodoDto,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let mut todo = get(db_pool, &todo_id, &user_id).await?
        .into_active_model();
    todo.title = Set(dto.clone().title.unwrap());
    todo.description = Set(dto.clone().description);
    todo.clone().update(db_pool).await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error updating todo",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?;
    let new_todo = todo.try_into_model().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            message: "Error updating todo",
            error: StatusCode::BAD_REQUEST.to_string(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
        }))
    })?;
    Ok((
        StatusCode::OK,
        Json(new_todo)
    ).into_response())
}


pub async fn delete_todo(
    db_pool: &DatabaseConnection,
    todo_id: &String,
    user_id: &String,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let mut todo = get(db_pool, &todo_id, &user_id).await?
        .into_active_model();
    todo.deleted_on = Set(Some(chrono::Utc::now().naive_utc()));
    todo.clone().update(db_pool).await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error deleting todo",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?;
    let deleted = todo.try_into_model().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            message: "Error deleting todo",
            error: StatusCode::BAD_REQUEST.to_string(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
        }))
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({ "deleted": deleted }))
    ).into_response())
}


pub async fn toggle_todo(
    db_pool: &DatabaseConnection,
    todo_id: &String,
    user_id: &String,
)
    -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let mut todo = get(db_pool, &todo_id, &user_id).await?
        .into_active_model();
    if todo.completed.unwrap() ==true {
        todo.completed = Set(false);
        todo.completed_on = Set(None);
    } else {
        todo.completed = Set(true);
        todo.completed_on = Set(Some(chrono::Utc::now().naive_utc()));
    }
    todo.clone().update(db_pool).await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error toggling todo",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?;
    let new_todo = todo.try_into_model().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            message: "Error toggling todo",
            error: StatusCode::BAD_REQUEST.to_string(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
        }))
    })?;
    Ok((
        StatusCode::OK,
        Json(new_todo)
    ).into_response())
}