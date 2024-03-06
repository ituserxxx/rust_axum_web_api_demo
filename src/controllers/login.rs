use axum::{
    extract::{Extension,Json},
    response::{Html,IntoResponse},
    http::Response,
    http::StatusCode,
};
use validator::Validate;
use async_session::{MemoryStore, Session, SessionStore};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use svg::node::element::Text;
use svg::Document;
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::Body;

use crate::api::login_api;
use crate::api::resp::ApiResponse;

pub async fn show_captcha( session: Extension<Arc<Mutex<login_api::SessionData>>>) -> impl IntoResponse {
    let captcha: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let text = svg::node::element::Text::new(captcha.clone())
        .set("x", 10)
        .set("y", 30)
        .set("font-size", 20);

    let document = Document::new().add(text);

    {
        let mut state = session.lock().await;
        state.captcha = Some(captcha.clone());
    }

    // 将 SVG 数据转换为 Bytes
    let body = Bytes::from(svg_data);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .body(BoxBody::new(body))
        .unwrap()

}


pub  async fn verify_captcha(
    Json(req): Json<login_api::LoginReq>,
    session: Extension<Arc<Mutex<login_api::SessionData>>>
) -> Json<ApiResponse<login_api::LoginResp>>  {
    if let Err(error) = req.validate() {
        return Json( ApiResponse::new(400, None, &format!("{}", error)))
    }
    // 获取 req 中的 captcha 字段的值
    let req_captcha = req.captcha.unwrap_or_default();
    let session_data = session.lock().await;
    if let Some(captcha_code) = &session_data.captcha {
        if captcha_code == &req_captcha {
            Json( ApiResponse::succ(Some(login_api::LoginResp{id:1})))
        } else {
            Json( ApiResponse::succ(Some(login_api::LoginResp{id:2})))
        }
    } else {
        Json( ApiResponse::succ(Some(login_api::LoginResp{id:3})))
    }
    // let error_msg = format!("err {}", "no pass");
    // return Json(ApiResponse::err(&error_msg))
}