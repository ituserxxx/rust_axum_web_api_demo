use axum::{
    Router,http,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, Extension},
};


#[derive(Clone)]
pub struct CurrentUser {
    id:i64
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
           // insert the current user into a request extension so the handler can
           // extract it
           req.extensions_mut().insert(current_user);

           println!("444");
           Ok(next.run(req).await)
       } else {
           println!("333");
           Err(StatusCode::UNAUTHORIZED)
       }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
     println!("222");
     println!("{}",auth_token);
     return Some(CurrentUser{id:1});
}
async fn handler() -> impl IntoResponse {
   println!("1111");
    "JWT Encode"
}
#[tokio::main]
async fn main() {
// curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8061/hello/jwt_dn

    let handler_r = Router::new()
        .route("/a", get(handler))
        .route_layer(middleware::from_fn(auth));
    let app = Router::new()
            .route("/", get(|| async { "☺ welcome to Rust" }))
            .nest("/h",handler_r);
    let routerInit = app.into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8061").await.unwrap();
    println!("server port on {}", "0.0.0.0:8061");
    // 启动服务
    axum::serve(listener, routerInit).await.unwrap();

}