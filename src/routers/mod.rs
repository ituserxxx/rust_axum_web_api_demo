use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::user,
};

pub fn init() -> Router {

    let user_router = Router::new()
        .route("/list", post(user::list))
        .route("/info", post(user::info))
        .route("/del", post(user::del))
        .route("/add", post(user::add));

    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/user", user_router);
}



