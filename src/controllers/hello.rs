use axum::{response::Json};
use crate::api::hello_api;
use crate::api::resp::ApiResponse;
use validator::Validate;

pub async fn hello() -> Json<ApiResponse<hello_api::HelloRes>> {
    let hp = hello_api::HelloRes {
        name: "Alice".to_string(),
        age: 30,
    };
    let api_response = ApiResponse::new(0,Some(hp),"ok");
    Json(api_response)
}
pub async fn add(Json(req): Json<hello_api::AddHelloReq>) -> Json<ApiResponse<hello_api::AddHelloRes>>{
    // 参数验证
    if let Err(error) = req.validate() {
        let error_msg = format!("{}", error);
        return Json(ApiResponse::err( &error_msg))
    }

    // 初始化返回结构体
    let rp = hello_api::AddHelloRes {
        name:req.name.clone(),
        age: req.phone.clone(),
    };

    // 返回响应数据
    Json(ApiResponse::succ(Some(rp)))
}