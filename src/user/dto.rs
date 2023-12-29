use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, message = "Your full name is required"))]
    pub full_name: String,
}