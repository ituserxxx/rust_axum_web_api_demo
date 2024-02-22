use axum::{
    response::{IntoResponse, Json},
    routing::get,
    BoxError,
    extract::Extension,
    Router,
    http::{HeaderMap, Request, Response, StatusCode, HeaderValue},
};
use crate::tools::jwt;
use crate::api::resp::{ApiResponse};
async fn auth_middleware(req: Request<()>) -> Result<Request<()>, Response<Json<&'static str>>> {
    // 从请求头中提取 token 参数
    let token_value = if let Some(header) = req.headers().get("Authorization") {
        header.to_str().unwrap_or_default()
    } else {
        ""
    };



    // 判断 token 是否等于 ""，如果等于则返回 401 状态码
    if token_value == "" {
        return Err(Response::new(Json("Unauthorized")).with_status(StatusCode::UNAUTHORIZED));
    }

   match jwt::dn_token(token_value).await {
      Err(err) => {
          let error_msg = format!("err {}", err);
          return Json(ApiResponse::err(&error_msg))
      }
    }

    // 将 header 参数追加到请求体中
    let body = format!("{} uid={}", req.body(), header_value);
    let new_req = req.method("POST").body(Bytes::from(body));
    Ok(new_req)
}
