use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::hello,
    controllers::user,
};

pub fn init() -> Router {
    let hello_router = Router::new()
        .route("/ph", post(hello::hello))
        .route("/gh", get(hello::hello));

    let user_router = Router::new()
        .route("/ph", get(user::hello))
        .route("/user/add", post(user::add));

    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/h", hello_router)
        .nest("/v1", user_router);
}



