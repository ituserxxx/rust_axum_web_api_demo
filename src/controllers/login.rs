use axum::{
    extract::{Extension,Json},
    extract::State,
    response::{Html,IntoResponse},
    http::Response,
    http::StatusCode,
};
use validator::Validate;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use svg::node::element::Text;
use svg::Document;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::body::{to_bytes, Body};
use axum_session::{Session, SessionNullPool, SessionConfig, SessionStore, SessionLayer};

use crate::{
    db::user_model,
    api::login_api,
    api::resp::ApiResponse,
    tools::jwt,
};


pub async fn show_captcha(  session: Session<SessionNullPool>) -> impl IntoResponse {
    let captcha: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    let text = svg::node::element::Text::new(captcha.clone())
        .set("x", 10)
        .set("y", 30)
        .set("font-size", 20);

    let document = Document::new().add(text);

    session.set("captcha", captcha.to_string());
    println!("curr captche is =: {}", captcha.clone());
    // 构建 SVG 图像的响应
   return  Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Cache-Control", "no-cache")
        .body(document.to_string())
        .unwrap();
}


pub  async fn verify_captcha(
    session: Session<SessionNullPool>,
    Json(req): Json<login_api::LoginReq>
) -> Json<ApiResponse<login_api::LoginResp>>  {
    if let Err(error) = req.validate() {
        return Json( ApiResponse::new(400, None, &format!("{}", error)))
    }
    let username = &req.username;
    let password = &req.password;

    println!("Received form data: username - {:?}, password - {:?}", username, password);

    if let Some(true_captcha) = session.get::<String>("captcha") {
        if true_captcha != req.captcha.to_string() {
            let error_msg = "验证码错误".to_string();
            return Json(ApiResponse::err(&error_msg));
        }
    } else {
        return Json(ApiResponse::err("验证码错误"));
    }

    let query_result = user_model::fetch_user_by_username_password(username,password).await;

    let uinfo = match query_result {
        Ok(Some(user)) =>user,
        Ok(None) => {
            return Json(ApiResponse::err( &"用户信息不存在"))
        },
        Err(err)=>{
            let error_msg = format!("获取用户信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    let token = jwt::en_token(uinfo.id).await;
    let rp = login_api::LoginResp {
        accessToken: token,
    };
    return Json( ApiResponse::succ(Some(rp)))
}