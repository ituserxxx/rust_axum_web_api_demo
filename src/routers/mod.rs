use axum::{
    Router,
    routing::{get, post},
    Extension,
    middleware::{self, Next},
};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;
use async_session::{MemoryStore, Session};
use std::sync::{Arc, Mutex};

use crate::{
    api::login_api,
    controllers::login,
    controllers::user,
    controllers::hello,
    middleware::auth,
};



pub fn init() -> Router {
    let hello_router = Router::new()
        .route("/jwt_en",  get(hello::jwt_en))
        .route("/jwt_dn",post(hello::jwt_dn))
        .layer(middleware::from_fn(auth::auth_jwt));

    let session_data = Arc::new(Mutex::new(login_api::SessionData::default()));
    let login_router = Router::new()
        .route("/api/auth/captcha",get(login::show_captcha))
        .route("/api/user/login", post(login::verify_captcha))
        .layer(Extension(session_data));


    let user_router = Router::new()
        .route("/list", post(user::list))
        .route("/info", post(user::info))
        .route("/del", post(user::del))
        .route("/add", post(user::add))
        .layer(middleware::from_fn(auth::auth_jwt));


    return Router::new()
        .route("/", get(|| async { "☺ welcome to Rust" }))
        .nest("/hello", hello_router)
        .nest("/", login_router)
        .nest("/user", user_router);
}



