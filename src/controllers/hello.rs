use crate::tools::jwt;
use axum::extract::Json;
use crate::api::resp::{ApiResponse};
use crate::api::hello_api;

pub async fn jwt_en() {
    let token = jwt::en_token(21).await;
     println!("token: {:?}", token);
}

pub async fn jwt_dn(Json(req): Json<hello_api::JwtDnReq>) -> Json<ApiResponse<hello_api::JwtDnRes>> {
    let token = req.token.unwrap_or_default();
    match jwt::dn_token(token).await {
      Ok(uid) => {
          println!("Decoded UID: {}", uid);
           return Json( ApiResponse::succ(Some(hello_api::JwtDnRes{})))
      }
      Err(err) => {
          println!("Error: {}", err);
          return Json( ApiResponse::succ(Some(hello_api::JwtDnRes{})))
      }
    }
}
