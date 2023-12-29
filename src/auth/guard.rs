use std::sync::Arc;
use axum::{http::StatusCode, response::{Response}, middleware::{Next}, extract::{Request}, Json};
use axum::extract::State;
use axum::http::header;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::{EntityTrait};
use crate::auth::dto::UserToken;
use crate::error::ErrorResponse;
use crate::module::AppState;
use crate::user::entity::users;

pub async fn auth(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let token = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            message: "You are not logged in, please provide token",
            error: StatusCode::UNAUTHORIZED.to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(state.config.secret.as_ref()),
        &Validation::default(),
    )
        .map_err(|error| {
            let json_error = ErrorResponse {
                message: "Invalid token",
                error: error.to_string(),
                status_code: StatusCode::UNAUTHORIZED.as_u16(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?
        .claims;

    let user_id = uuid::Uuid::parse_str(&claims.id).map_err(|error| {
        let json_error = ErrorResponse {
            message: "Invalid token",
            error: error.to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let user = users::Entity::find_by_id(user_id).one(&state.database).await;

    let user = user.unwrap().ok_or_else(|| {
        let json_error = ErrorResponse {
            message: "The user belonging to this token no longer exists",
            error: "".to_string(),
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}