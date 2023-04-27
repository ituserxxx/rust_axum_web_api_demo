use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    code: u32,
    data: Option<T>,
    msg: String,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u32, data: Option<T>, msg: &str) -> Self {
        ApiResponse {
            code,
            data,
            msg: msg.to_string(),
        }
    }
}