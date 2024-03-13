use std::clone::Clone;
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 引入全局变量
use super::DB_POOL;

#[allow(non_snake_case)]
#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct Role {
    pub id          : i64,
    pub code        : i64,
    pub name        : String,
    pub enable      : i64,
}
impl Default for Role {
    fn default() -> Self {
        Role {
            id          :0,
            code        :0,
            name        :String::default(),
            enable      :0,
        }
    }
}
// 查询多条记录
pub async fn fetch_roles_where_id_in(ids:vec![i64]) -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let rows = sqlx::query_as::<_, Role>("SELECT * FROM role where id in (?)")

        .fetch_all(&pool)
        .await?;
    Ok(rows.into_iter().map(|row| row.clone()).collect())
}