use std::clone::Clone;
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 引入全局变量
use super::DB_POOL;

#[derive(Debug,Clone, Deserialize, Serialize,  sqlx::FromRow)]
pub struct Permission {
    pub id          : i64,
    pub name      : String,
    pub code       : String,
    pub r#type: String,
    #[allow(non_snake_case)]
    pub parentId       : Option<i64>,
    pub path       : Option<String>,
    pub redirect       : Option<String>,
    pub icon       : Option<String>,
    pub component       : Option<String>,
    pub layout       : Option<String>,
    #[allow(non_snake_case)]
    pub keepAlive       : Option<i8>,
    pub method       : Option<String>,
    pub description       : Option<String>,
    pub show       : i8,
    pub enable       : i8,
    pub order       : i64,

}
impl Default for Permission {
    fn default() -> Self {
        Permission {
            id          :0,
            name:String::default(),
            code:String::default(),
            r#type: (),
            parentId: (),
            path: (),
            redirect: (),
            icon: (),
            component: (),
            layout: (),
            keepAlive: (),
            method: (),
            description: (),
            show: 1,
            enable: 1,
            order: 0,
        }
    }
}
// 查询多条记录
pub async fn find_1_level_where_by_user_id(user_id:i64) -> Result<Vec<Permission>, sqlx::Error> {
    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
    let rows = sqlx::query_as::<_, Role>("SELECT * FROM `permission` WHERE id (select permissionId from role_permissions_permission where roleId IN(SELECT roleId FROM user_roles_role WHERE userId=?))")
        .bind(user_id)
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}
