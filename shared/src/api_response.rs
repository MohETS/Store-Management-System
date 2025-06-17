use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(message: String) -> Self {
        ApiResponse {
            message,
            data: None,
        }
    }

    pub fn with_data(message: String, data: T) -> Self {
        ApiResponse {
            message,
            data: Some(data),
        }
    }
}