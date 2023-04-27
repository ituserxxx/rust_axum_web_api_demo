use axum::{response::Json};
use crate::api::hello as hello_api;
use crate::api::resp::ApiResponse;

pub async fn hello() -> Json<ApiResponse<hello_api::HelloRes>> {
    let hp = hello_api::HelloRes {
        name: "Alice".to_string(),
        age: 30,
    };
    let api_response = ApiResponse::new(0,Some(hp),"ok");
    Json(api_response)
}