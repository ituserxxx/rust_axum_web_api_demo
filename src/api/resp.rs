use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    code: u32,
//     #[serde(default)]
    data: Option<T>,
    msg: String,
}

impl<T> ApiResponse<T> {
    pub fn new(code: u32, data: Option<T>, msg: &str) -> Self {
        ApiResponse {
            code:code,
            data:data,
            msg: msg.to_string(),
        }
    }
    pub fn succ( data: Option<T>) -> Self {
        ApiResponse {
            code:0,
            data:data,
            msg: "ok".to_string(),
        }
    }
    pub fn err(msg: &str) -> Self {
        ApiResponse {
            code:500,
            data:None,
            msg: msg.to_string(),
        }
    }
}