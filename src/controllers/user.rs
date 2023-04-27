use axum::extract::Json;
use validator::Validate;

use crate::api::resp::{ApiResponse};
use crate::api::user as user_api;

pub async fn hello() -> Json<ApiResponse<user_api::HelloRes>> {
    let uresp = user_api::HelloRes {
        name: "Alice".to_string(),
        age: 30,
    };
    let api_response = ApiResponse::new(0, Some(uresp), "ok");
    Json(api_response)
}
//
pub async fn add(Json(req): Json<user_api::AddUserReq>) -> Json<ApiResponse<user_api::AddUserReq>> {
    if let Err(error) = req.validate() {
        let error_msg = format!("{}", error);
        let resp = ApiResponse::new(400, None, &error_msg);
        return Json(resp)
    }

    let resp = ApiResponse::new(200, Some(req), "User added successfully");
    Json(resp)
}

