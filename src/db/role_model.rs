use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use std::clone::Clone;

// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: i64,
}
impl Default for Role {
    fn default() -> Self {
        Role {
            id: 0,
            code: String::default(),
            name: String::default(),
            enable: 0,
        }
    }
}
// 查询多条记录
pub async fn fetch_all_where_user_id(uid: i64) -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let rows = sqlx::query_as::<_, Role>(
        "SELECT * FROM `role` WHERE id IN(SELECT roleId FROM user_roles_role WHERE userId=?)",
    )
    .bind(uid)
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}
// 查询所有
pub async fn fetch_all() -> Result<Vec<Role>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let rows = sqlx::query_as::<_, Role>("SELECT * FROM `role`")
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}
