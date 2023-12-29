use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, TryIntoModel};
use sea_orm::ActiveValue::Set;
use serde_json::{json};
use uuid::Uuid;
use crate::error::ErrorResponse;
use crate::user::dto::UpdateUserDto;
use crate::user::entity::users;

pub async fn get_all_users(
    database: &DatabaseConnection
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let users = users::Entity::find()
        .filter(users::Column::DeletedOn.is_null())
        .order_by_asc(users::Column::CreatedOn)
        .all(database).await.map_err(|error| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: error.to_string(),
                message: "Error finding users",
                status_code: StatusCode::NOT_FOUND.as_u16(),
            })
        )
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({ "data": users }))
    ).into_response())
}

pub async fn get_one_user(
    database: &DatabaseConnection,
    user_id: &String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let user = get(database, user_id).await?;
    Ok((StatusCode::OK, Json(user)).into_response())
}

pub async fn delete_user(
    database: &DatabaseConnection,
    user_id: &String,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let user = get(database, user_id).await?;

    let mut active_user: users::ActiveModel = user.clone().into_active_model();

    active_user.deleted_on = Set(Some(chrono::Utc::now().naive_utc()));
    active_user.update(database).await.map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Error deleting user",
                error: error.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            })
        )
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({ "deleted": user }))
    ).into_response())
}

pub async fn get(
    database: &DatabaseConnection,
    user_id: &String,
) -> Result<users::Model, (StatusCode, Json<ErrorResponse>)> {
    Ok(users::Entity::find()
        .filter(users::Column::DeletedOn.is_null())
        .filter(users::Column::Id.eq(Uuid::parse_str(user_id.as_str()).map_err(|_| {
            (StatusCode::NOT_FOUND, Json(ErrorResponse {
                message: "Invalid user id",
                error: StatusCode::BAD_REQUEST.to_string(),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            }))
        })?))
        .one(database)
        .await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error loading user",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(ErrorResponse {
            message: "User not found",
            error: StatusCode::NOT_FOUND.to_string(),
            status_code: StatusCode::NOT_FOUND.as_u16(),
        }))
    })?)
}

pub async fn edit_user(
    database: &DatabaseConnection,
    user_id: &String,
    dto: Json<UpdateUserDto>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let mut user = get(database, user_id).await?
        .into_active_model();
    user.full_name = Set(dto.full_name.to_string());
    user.clone().update(database).await.map_err(|error| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Error updating user",
            error: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }))
    })?;
    let new_user = user.try_into_model().map_err(|_| {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            message: "Error updating user",
            error: StatusCode::BAD_REQUEST.to_string(),
            status_code: StatusCode::BAD_REQUEST.as_u16(),
        }))
    })?;
    Ok((
        StatusCode::OK,
        Json(new_user)
    ).into_response())
}