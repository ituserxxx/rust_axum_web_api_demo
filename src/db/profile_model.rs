use std::clone::Clone;
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 引入全局变量
use super::DB_POOL;

#[allow(non_snake_case)]
#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct Profile {
    pub id          : i64,
    pub gender      : i64,
    pub avatar      : String,
    pub address     : String,
    pub email       : String,
    pub userId      : i64,
    pub nickName    : String,
}
impl Default for Profile {
    fn default() -> Self {
        Profile {
            id          :0,
            gender      :0,
            avatar      :String::default(),
            address     :String::default(),
            email       :String::default(),
            userId      :0,
            nickName    :String::default(),
        }
    }
}
pub async fn fetch_one_by_user_id(user_id: i64) -> Result<Option<Profile>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let result = sqlx::query_as::<_, User>("SELECT * FROM Profile where userId = ? ")
        .bind(id)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

