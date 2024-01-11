pub mod user_model;
pub mod schema;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use lazy_static::lazy_static;

// 定义全局数据库连接变量
lazy_static::lazy_static! {
    static ref CONNECTION: MysqlConnection = establish_connection();
}

fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:mysql-xxx@tcp(192.168.3.214:3310)/sg_back_test";
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

