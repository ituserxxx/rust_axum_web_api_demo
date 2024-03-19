use std::clone::Clone;
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 引入全局变量
use super::DB_POOL;

#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct Profile {
    pub id          : i64,
    pub gender      : Option<i64>,
    pub avatar      : String,
    pub address     : Option<String>,
    pub email       : Option<String>,
    #[allow(non_snake_case)]
    pub userId      : i64,
    #[allow(non_snake_case)]
    pub nickName    : Option<String>,
}
impl Default for Profile {
    fn default() -> Self {
        Profile {
            id          :0,
            gender      :Some(0),
            avatar      :String::default(),
            address     :Some(String::default()),
            email       :Some(String::default()),
            userId      :0,
            nickName    :Some(String::default()),
        }
    }
}
pub async fn find_info_by_user_id(user_id: i64) -> Result<Option<Profile>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let result = sqlx::query_as::<_, Profile>("SELECT * FROM profile where userId = ? ")
        .bind(user_id)
        .fetch_optional(&pool)
        .await?;
    Ok(result)
}

