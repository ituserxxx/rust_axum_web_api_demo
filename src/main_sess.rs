use std::net::SocketAddr;
use axum_session::{Session, SessionNullPool, SessionConfig, SessionStore, SessionLayer};
use axum::{
    Router,
    routing::get,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default().with_table_name("sessions_table");

    // create SessionStore and initiate the database tables
    let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/greet2", get(greet))
        .layer(SessionLayer::new(session_store));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("server port on {}", "0.0.0.0:8800");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greet(session: Session<SessionNullPool>) -> String {
    let mut count: usize = session.get("count").unwrap_or(0);
    println!("count {}",count);
    count += 1;
    session.set("count", count);
    count.to_string()
}