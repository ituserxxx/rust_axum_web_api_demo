use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::user,
    controllers::hello,
    middleware::auth,
};

pub fn init() -> Router {


    let hello_router = Router::new()
        .route("/jwt_en",  get(hello::jwt_en))
        .route("/jwt_dn",  post(hello::jwt_dn).layer_fn(auth::auth_middleware));


    let user_router = Router::new()
        .route("/list", post(user::list))
        .route("/info", post(user::info))
        .route("/del", post(user::del))
        .route("/add", post(user::add));



    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/hello", hello_router)
        .nest("/user", user_router);
}



