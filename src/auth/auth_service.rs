use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel};
use sea_orm::prelude::{DateTime};
use crate::auth::dto::{LoginDto, LoginResponse, RegisterDto, UserToken};
use crate::error::ErrorResponse;
use crate::response::OkResponse;
use crate::user::entity::users;

fn generate_tokens(user_id: &str, secret: &str) -> (String, String) {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let access_exp = (now + chrono::Duration::days(7)).timestamp() as usize;
    let refresh_exp = (now + chrono::Duration::days(14)).timestamp() as usize;

    // Encode tokens
    let access_token = encode(&Header::default(), &UserToken {
        id: user_id.to_string(),
        iat,
        exp: access_exp,
    }, &EncodingKey::from_secret(&secret.as_bytes()), ).unwrap();
    let refresh_token = encode(&Header::default(), &UserToken {
        id: user_id.to_string(),
        iat,
        exp: refresh_exp,
    }, &EncodingKey::from_secret(&secret.as_bytes()), ).unwrap();

    (access_token, refresh_token)
}

pub async fn register_user(db_pool: &DatabaseConnection, dto: &Json<RegisterDto>) -> Response {
    let new_user = users::ActiveModel {
        full_name: Set(dto.full_name.to_owned()),
        password: Set(dto.password.to_owned()),
        email: Set(dto.email.to_owned()),
        ..Default::default()
    };

    let user = new_user.save(db_pool).await;

    match user {
        Ok(u) => {
            (
                StatusCode::CREATED,
                Json(OkResponse {
                    query_time: DateTime::default(),
                    data: u.try_into_model().unwrap(),
                })
            ).into_response()
        }
        Err(err) => {
            if err.to_string().contains("user_email_index") {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        message: "email has already been used",
                        error: err.to_string(),
                        status_code: StatusCode::BAD_REQUEST.as_u16(),
                    })
                ).into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        message: "Error registering user",
                        error: err.to_string(),
                        status_code: StatusCode::BAD_REQUEST.as_u16(),
                    })
                ).into_response()
            }
        }
    }
}

pub async fn get_user_token(db_conn: &DatabaseConnection, dto: Json<LoginDto>, secret: &String) -> Response {
    let user = users::Entity::find()
        .filter(users::Column::DeletedOn.is_null())
        .filter(users::Column::Email.eq(&dto.email))
        .one(db_conn).await;
    match user {
        Ok(record) => {
            match record {
                Some(user_record) => {
                    if user_record.password == (&dto.password).to_string() {
                        let token = generate_tokens(&user_record.id.to_string().as_str(), secret.as_str());
                        (
                            StatusCode::OK,
                            Json(LoginResponse {
                                access_token: token.0,
                                refresh_token: token.1,
                            })
                        ).into_response()
                    } else {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(
                                ErrorResponse {
                                    message: "Invalid credential",
                                    error: "Not Found".to_string(),
                                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                                }
                            )
                        ).into_response()
                    }
                }
                None => {
                    (
                        StatusCode::NOT_FOUND,
                        Json(ErrorResponse {
                            message: "Invalid credential",
                            error: "Not Found".to_string(),
                            status_code: StatusCode::NOT_FOUND.as_u16(),
                        })
                    ).into_response()
                }
            }
        }
        Err(err) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Error logging user",
                    error: err.to_string(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                })
            ).into_response()
        }
    }
}