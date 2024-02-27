
use axum::{
     Router, http,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, Extension},
    extract::rejection::JsonRejection,
};

use serde::{Deserialize, Serialize};
use axum_extra::extract::WithRejection;
use thiserror::Error;
use axum::Json;

#[derive(Clone,Debug)]
pub struct CurrentUser {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub name: Option<String>,
}
async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);

        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    return Some(CurrentUser{id:1});
}

async fn handler(
    // extract the current user, set by the middleware
    Extension(current_user): Extension<CurrentUser>,
    req: Request,
) {
      println!("req: {:?}", req);
      println!("Current user: {:?}", current_user);
}




#[tokio::main]
async fn main() {
// curl -X GET -H "Content-Type: application/json" -H "Authorization: xxxxxxxxx" -d '{"name":"xx1"}' http://127.0.0.1:8061/
// curl -X GET -H "Content-Type: application/json"  -d '{"name":"xx1"}' http://127.0.0.1:8061/


    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(auth));
    let routerInit = app.into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8061").await.unwrap();
    println!("server port on {}", "0.0.0.0:8061");
    // 启动服务
    axum::serve(listener, routerInit).await.unwrap();

}


