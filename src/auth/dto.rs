use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 8, max = 24, message = "Password must be between 8 to 24 characters long"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserToken {
    pub id: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct RegisterDto {
    #[validate(length(min = 3, message = "Your full name is required"))]
    pub full_name: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 8, max = 24, message = "Password must be between 8 to 24 characters long"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetForgotPasswordDto {
    #[validate(email(message = "Invalid email"))]
    pub email: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct SetPasswordDto {
    #[validate(length(min = 8, max = 24, message = "Password must be between 8 to 24 characters long"))]
    pub new_password: String,
}