use axum::{
    Router, http,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, Extension, Json},
};

use serde::{Deserialize, Serialize};
use axum::BoxError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrentUser {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub name: Option<String>,
}

async fn handler( current_user: Extension<CurrentUser>) -> Result<impl IntoResponse, BoxError>{
    println!("1111");
//     println!("998--{:?}", req);
    println!("Current user: {:?}", current_user);
    Ok("xxxxxxxxxxxxxx")
}
#[tokio::main]
async fn main() {
    // curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"name":"xx1"}' http://127.0.0.1:8061/
    let handler_r = Router::new()
        .route("/a", get(handler))
        .route_layer(middleware::from_fn(auth));

    let app = Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/h", handler_r);
    let routerInit = app.into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8061").await.unwrap();
    println!("server port on {}", "0.0.0.0:8061");
    // 启动服务
    axum::serve(listener, routerInit).await.unwrap();
}

async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
   let auth_header = if let Some(auth_header) = auth_header {
           println!("666");
           auth_header
       } else {
           println!("555");
           return Err(StatusCode::UNAUTHORIZED);
       };
       if let Some(current_user) = authorize_current_user(&auth_header).await {

            req.extensions_mut().insert(current_user);
            println!("444");
            Ok(next.run(req).await)
       } else {
           println!("333");
           Err(StatusCode::UNAUTHORIZED)
       }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
     println!("authorize_current_user={}",auth_token);
     return Some(CurrentUser{id:1});
}