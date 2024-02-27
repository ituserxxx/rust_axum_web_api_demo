//
// mod api;
// mod controllers;
// mod db;
// mod middleware;
// mod routers;
// mod tools;
//
// #[tokio::main]
// async fn main() {
//
//     // 初始化数据库连接
//     let _ = db::mysql_connect().await;
//
//     // 初始化路由00
//     let routerInit = routers::init().into_make_service();
//
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
//     println!("server port on {}", "0.0.0.0:8001");
//     // 启动服务
//     axum::serve(listener, routerInit).await.unwrap();
//
//     // 关闭数据库连接
//     let _ = db::mysql_disconnect().await;
// }
