use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
mod routers;
mod controllers;
mod api;
mod db;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routers::init().into_make_service())
        .await
        .unwrap();
}
//     let pool = MySqlPool::connect("mysql://naive_admin:naive_admin_pass@127.0.0.1:33069/naive_admin").await?;
