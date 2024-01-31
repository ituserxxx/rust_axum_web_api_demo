mod tools;
mod routers;
mod controllers;
mod api;
mod db;
use std::net::SocketAddr;
#[tokio::main]
async fn main() {

    // 初始化数据库连接
    db::mysql_connect().await;

    // 初始化路由00
    let routerInit = routers::init().into_make_service();


    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    println!("server port on {}", addr);

    // 启动服务
    axum::Server::bind(&addr)
        .serve(routerInit)
        .await
        .unwrap();

    // 关闭数据库连接
    db::mysql_disconnect().await;
}
