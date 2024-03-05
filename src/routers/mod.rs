use axum::{
    Router,
    routing::{get, post},
    Extension,

};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;
use axum::{
    middleware::{self, Next},
};

use crate::{
    controllers::user,
    controllers::hello,
    middleware::auth,
};

#[derive(Clone)]
struct State {}

pub fn init() -> Router {

    let hello_router = Router::new()
        .route("/jwt_en",  get(hello::jwt_en))
        .route("/jwt_dn",post(hello::jwt_dn))
        .layer(middleware::from_fn(auth::auth_jwt));

    let user_router = Router::new()
        .route("/list", post(user::list))
        .route("/info", post(user::info))
        .route("/del", post(user::del))
        .route("/add", post(user::add));

    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .route("/user/login", post(user::list)).layer(middleware::from_fn(auth::auth_jwt))
        .nest("/hello", hello_router)
        .nest("/user", user_router);
}



