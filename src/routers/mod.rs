use axum::{
    Router,
    routing::{get, post},
    extract::Extension,
    middleware::{self, Next},
};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;

use std::sync::{Arc, Mutex};
use axum_session::{Session, SessionNullPool, SessionConfig, SessionStore, SessionLayer};

use crate::{
    api::login_api,
    controllers::login,
    controllers::user,
    controllers::hello,
    middleware::auth,
};



pub async fn init() -> Router {
    let hello_router = Router::new()
        .route("/jwt_en",  get(hello::jwt_en))
        .route("/jwt_dn",post(hello::jwt_dn))
        .layer(middleware::from_fn(auth::auth_jwt));

    let session_config = SessionConfig::default()
        .with_table_name("sessions_table");

    // create SessionStore and initiate the database tables
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await.unwrap();


    let login_router = Router::new()
        .route("/api/auth/captcha",get(login::show_captcha))
        .route("/api/user/login", post(login::verify_captcha))
        .layer(SessionLayer::new(session_store));

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



