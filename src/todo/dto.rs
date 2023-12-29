use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
pub struct CreateTodoDto {
    #[validate(length(min = 3, max = 225), required())]
    pub title: Option<String>,

    #[validate(length(max = 1024))]
    pub description: Option<String>,
}

#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTodoDto {
    #[validate(length(min = 3, max = 225))]
    pub title: Option<String>,

    #[validate(length(max = 1024))]
    pub description: Option<String>,
}