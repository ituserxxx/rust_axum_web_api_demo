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

use crate::api::login_api;
use crate::api::resp::ApiResponse;

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

    session.set("captcha", captcha.clone());

    // 构建 SVG 图像的响应
    let svg_response = Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Cache-Control", "no-cache")
        .body(document.to_string())
        .unwrap();

    svg_response

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
    format!("Received form data: username - {:?}, password - {:?}", username, password);

    let mut count: usize = session.get("count").unwrap_or(0);
    println!("count {}",count);
    count += 1;
    session.set("count", count);

    let error_msg = format!("err {}",count);
    return Json(ApiResponse::err(&error_msg))
}