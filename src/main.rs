use std::net::SocketAddr;
mod routers;
mod controllers;
mod api;
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routers::init().into_make_service())
        .await
        .unwrap();
}