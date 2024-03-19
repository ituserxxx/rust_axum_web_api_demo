use std::clone::Clone;
use sqlx::{ Encode,mysql::MySqlQueryResult,Row};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use axum::{
    extract::{Query},
};
// 引入全局变量
use super::DB_POOL;

use crate::api::user_api;
#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct User {
    pub id          : i64,
    pub username    : String,
    pub password    : String,
    pub enable      : i8,
    #[allow(non_snake_case)]
    pub createTime  : DateTime<Utc>,
    #[allow(non_snake_case)]
    pub updateTime  : DateTime<Utc>,
}
impl Default for User {
    fn default() -> Self {
        User {
            id          :   0,
            username    :   String::default(),
            password    :   String::default(),
            enable      :   0,
            createTime  :   Utc::now(),
            updateTime  :   Utc::now(),
        }
    }
}
// 查询一条记录-通过 username and password
pub async fn fetch_user_by_username_password(username: String,password: String) -> Result<Option<User>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where username = ? and password = ? ")
        .bind(&username)
        .bind(&password)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

// 查询一条记录-通过 id
pub async fn find_info_by_id(id: i64) -> Result<Option<User>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM user where id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await?;
    Ok(result)
}

// 查询多条记录
pub async fn fetch_all_users(req: Query<user_api::UserListReq>) -> Result<Vec<User>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    // 构建 SQL 查询语句
    let mut sql_str = "SELECT * FROM user".to_string();
    let mut params: Vec<String> = Vec::new();
    if req.enable.is_some() || req.gender.is_some() || req.username.is_some() {
        sql_str.push_str(" WHERE");
        let mut conditions: Vec<String> = Vec::new();

        if let Some(enable) = req.enable {
            conditions.push(" enable = ?".to_string());
            params.push((&enable).to_string());
        }
        if let Some(gender) = req.gender {
            conditions.push(" gender = ?".to_string());
            params.push((&gender).to_string());
        }
        if let Some(username) = req.username.as_ref() {
            conditions.push(" username = ?".to_string());
            params.push((&username).to_string());
        }
        sql_str.push_str(&conditions.join(" AND"));
    }
    sql_str.push_str(" order by id desc LIMIT ? OFFSET ? ");
    println!("sql-->{:?}",sql_str);
    let limit = req.pageSize.unwrap_or(10);
    let offset = (req.pageNo.unwrap_or(1)-1)*10;

    let mut query_builder = sqlx::query(&sql_str);

    for (index, param) in params.iter().enumerate() {
        query_builder = query_builder.bind_named(format!("param{}", index), param);
    }

    let result = query_builder
        .bind(("limit", &limit))
        .bind(("offset", &offset))
        .fetch_all(&pool)
        .await?;

    let mut users: Vec<User> = Vec::new();
    for row in result {
        let user = User {
            // 从数据库行中提取用户信息并创建 User 对象
            id: row.get("id"),
            username: row.get("username"),
            // 其他字段类似地从数据库行中提取
            password:row.get("password"),
            enable:row.get("enable"),
            createTime:row.get("createTime"),
            updateTime: row.get("updateTime"),
        };
        users.push(user);
    }

    Ok(users)
}

// 更新记录-通过 id
pub async fn update_username_by_id( username: String, id: i64) -> Result<MySqlQueryResult, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let  result = sqlx::query("update user set username = ? where id = ?")
            .bind(&username)
            .bind(id)
            .execute(&pool)
            .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}

// 删除记录-通过 id
pub async fn delete_user_by_id( id: i64) -> Result< MySqlQueryResult,sqlx::Error> {
     let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
     let result = sqlx::query("delete from user where id = ?")
            .bind(id)
            .execute(&pool)
            .await?;
     Ok(result)
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
