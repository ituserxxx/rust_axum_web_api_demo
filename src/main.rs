use std::net::SocketAddr;
mod tools;
mod routers;
mod controllers;
mod api;
mod db;
#[tokio::main]
async fn main() {
    db::mysql_connect();
    let routerInit = routers::init().into_make_service();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("server port on {}", addr);
    axum::Server::bind(&addr)
        .serve(routerInit)
        .await
        .unwrap();

    db::mysql_disconnect();
}
