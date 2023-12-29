use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    pub(crate) message: &'static str,
    pub(crate) error: String,
    pub(crate) status_code: u16,
}