use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub cached: bool,
    pub data: Option<T>,
    pub path: String,
}

impl<T> ApiResponse<T> {
    pub fn new(status: String, message: String, path: String) -> Self {
        ApiResponse {
            status,
            message,
            cached: false,
            data: None,
            path
        }
    }

    pub fn with_data(status: String, message: String, cached: bool, data: T, path: String) -> Self {
        ApiResponse {
            status,
            message,
            cached,
            data: Some(data),
            path
        }
    }
}