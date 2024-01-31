use std::clone::Clone;
use sqlx::mysql::{ MySqlPool,MySqlQueryResult};
// 引入全局变量
use super::DB_POOL;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct User {
    username    : String,
    password    : String,
    enable      : i8,
    createTime  : DateTime<Utc>,
    updateTime  : DateTime<Utc>,
}

// 查询一条记录-通过 id
pub async fn fetch_user_by_id(id: i64) -> Result<User, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?;
   Ok(result)
}

// 查询多条记录
pub async fn fetch_all_users() -> Result<Vec<User>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let rows = sqlx::query_as::<_, User>("SELECT * FROM user")
            .fetch_all(&pool)
            .await?;
    Ok(rows.into_iter().map(|row| row.clone()).collect())
}

// 更新记录-通过 id
pub async fn update_username_by_id( username: &str, id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let  result = sqlx::query("update user set username = ? where id = ?")
            .bind(username)
            .bind(id)
            .execute(&pool)
            .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 删除记录-通过 id
pub async fn delete_user_by_id( id: i64) -> Result<(),sqlx::Error> {
     let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
     let _ = sqlx::query("delete from user where id = ?")
            .bind(id)
            .execute(&pool)
            .await;
     Ok(())
     // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 新增
pub async fn add_user_by_struct(data: User) -> Result< MySqlQueryResult,sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let insert_sql = "INSERT INTO user (username, password, enable, createTime, updateTime) VALUES (?, ?, ?, ?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.username)
        .bind(&data.password)
        .bind(&data.enable)
        .bind(&data.createTime)
        .bind(&data.updateTime)
        .execute(&pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}
