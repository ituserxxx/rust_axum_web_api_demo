use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 建立 MySQL 连接池
    let pool = MySqlPool::builder()
        .max_size(5)
        .build(&"mysql://user:password@localhost/database")
        .await?;

    // 在此处执行增删改查操作
    get_users(pool)
    Ok(())
}
use sqlx::mysql::MySqlQueryAs;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    username : String,
    password: String,
    enable: i8,
    createTime: String,
    updateTime: String,
}

async fn get_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(pool)
        .await?;

    Ok(users)
}
