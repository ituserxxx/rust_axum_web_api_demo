use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, Encode, MySql, MySqlPool, Row, Transaction};
use std::clone::Clone;
// 引入全局变量
use super::DB_POOL;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct RolePermissionsPermission {
    #[allow(non_snake_case)]
    pub roleId: i64,
    #[allow(non_snake_case)]
    pub permissionId: i64,
}
impl Default for RolePermissionsPermission {
    fn default() -> Self {
        RolePermissionsPermission {
            roleId: 0,
            permissionId: 0,
        }
    }
}

// 查询一个字段记录，返回数组值
pub async fn fetch_permission_ids_where_role_id(role_id: i64) -> Result<Vec<i64>, sqlx::Error> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();

    let rows: Vec<RolePermissionsPermission> =
        sqlx::query_as("SELECT permissionId  FROM role_permissions_permission WHERE roleId  = ?")
            .bind(role_id)
            .fetch_all(&pool)
            .await?;
    // 提取 permissionId 列的值并转换为 i64 数组
    let permission_ids: Vec<i64> = rows.iter().map(|row| row.permissionId).collect();
    Ok(permission_ids)
}
