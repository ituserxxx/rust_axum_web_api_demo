use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::hello,
//     controllers::user,
};

pub fn init() -> Router {
    let hello_router = Router::new()
        .route("/post", post(hello::hello))
        .route("/get", get(hello::hello))
        .route("/add", post(hello::add));
//
//     let user_router = Router::new()
//         .route("/ph", get(user::hello))
//         .route("/user/add", post(user::add));
//
    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/hello", hello_router)
//         .nest("/v1", user_router);
}



