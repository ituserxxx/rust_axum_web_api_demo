use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
    extract::State,
};
use tokio::sync::Mutex;
use std::sync::Arc;

// Some shared state used throughout our application
#[derive(Debug)]
struct AppState {
    pub id: String,
}

async fn handler(
    State(state): State<Arc<AppState>>,
) {
    // ...
}

#[tokio::main]
async fn main() {
    let state = Arc::new(State { id: "sssssss".to_string() });
    let app = Router::new()
        .route("/", get(handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8800").await.unwrap();
    println!("server port on {}", "0.0.0.0:8800");
    // 启动服务
    axum::serve(listener, app).await.unwrap();
}
