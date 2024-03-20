use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, MySqlPool, Encode, Row, Transaction, MySql};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserRolesRole {
    #[allow(non_snake_case)]
    pub userId: i64,
    #[allow(non_snake_case)]
    pub roleId: i64,
}
impl Default for UserRolesRole {
    fn default() -> Self {
        UserRolesRole {
            userId: 0,
            roleId: 0,
        }
    }
}
// 查询一个字段记录，返回数组值
pub async fn fetch_role_id_where_user_id(uid: i64) -> Result<Vec<i64>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();

    let rows: Vec<UserRolesRole> =
        sqlx::query_as("SELECT roleId FROM user_roles_role WHERE userId = ?")
            .bind(uid)
            .fetch_all(&pool)
            .await?;
    // 提取 roleId 列的值并转换为 i64 数组
    let role_ids: Vec<i64> = rows.iter().map(|row| row.roleId).collect();
    Ok(role_ids)
}

// 查询一个字段记录，返回数组值
pub async fn find_is_admin_role_by_user_id(uid: i64) -> Result<bool, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    // 执行 count 查询
    let result: Option<i64> =
        sqlx::query_scalar("SELECT roleId FROM user_roles_role WHERE roleId=1 and userId = ?")
            .bind(uid)
            .fetch_optional(&pool)
            .await?;
    // 检查查询结果是否为 Some，并且值等于 1
    let count_equals_one = match result {
        Some(count) => count == 1,
        None => false, // 如果查询结果为 None，则认为 count 不等于 1
    };
    Ok(count_equals_one)
}
// 新增用户权限关系（需要加事务，所以pool从外面传进来）
pub async fn add_user_role_by_struct(mut pool: Transaction<MySql>, data: UserRolesRole) -> Result<MySqlQueryResult, sqlx::Error> {

    let insert_sql = "INSERT INTO user_roles_role (userId, roleId) VALUES (?, ?)";
    let result = sqlx::query(&insert_sql)
        .bind(&data.userId)
        .bind(&data.roleId)
        .execute(&mut *pool)
        .await?;
    Ok(result)
    // MySqlQueryResult { rows_affected: 1, last_insert_id: 3 }
}