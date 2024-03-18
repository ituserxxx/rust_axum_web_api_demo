use std::clone::Clone;
use sqlx::mysql::MySqlQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::rc::Rc;

// 引入全局变量
use super::DB_POOL;

#[derive(Debug,Clone, Deserialize, Serialize, sqlx::FromRow)]
#[sqlx(default)]
pub struct Permission {
    pub id          : i64,
    pub name        : String,
    pub code        : String,
    pub r#type      : String,
    #[allow(non_snake_case)]
    pub parentId    : Option<i64>,
    pub path        : Option<String>,
    pub redirect    : Option<String>,
    pub icon        : Option<String>,
    pub component   : Option<String>,
    pub layout      : Option<String>,
    #[allow(non_snake_case)]
    pub keepAlive   : Option<i8>,
    pub method      : Option<String>,
    pub description : Option<String>,
    pub show        : i8,
    pub enable      : i8,
    pub order       : i64,
    #[sqlx(default)]
    pub children: Option<Vec<Permission>>,
}
impl Default for Permission {
    fn default() -> Self {
        Permission {
            id          :   0,
            name        :   String::default(),
            code        :   String::default(),
            r#type      :   String::default(),
            parentId    :   Some(0),
            path        :   Some(String::default()),
            redirect    :   Some(String::default()),
            icon        :   Some(String::default()),
            component   :   Some(String::default()),
            layout      :   Some(String::default()),
            keepAlive   :   Some(0),
            method      :   Some(String::default()),
            description :   Some(String::default()),
            show        :   1,
            enable      :   1,
            order       :   0,
            children: Some(Vec::new()),
        }
    }
}
//
pub async fn find_1_level() -> Result<Vec<Permission>, sqlx::Error> {

    let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();

    let rows = sqlx::query_as::<_, Permission>("SELECT * FROM `permission` WHERE parentId is NULL ORDER BY `order` ASC ")
        .fetch_all(&pool)
        .await?;
    Ok(rows)
}

// 查询1级权限通过 user_id
// pub async fn find_1_level_where_by_user_id(user_id:i64) ->  Result<Vec<Permission>, sqlx::Error> {
//     let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
//     let rows: Vec<Permission> = sqlx::query_as::<_, Permission>("SELECT * FROM `permission` WHERE parentId is NULL and id in (select permissionId from role_permissions_permission where roleId IN(SELECT roleId FROM user_roles_role WHERE userId=?)) ORDER BY `order` ASC ")
//         .bind(user_id)
//         .fetch_all(&pool)
//         .await?;
//
//     Ok(permissions_with_rc)
// }
// 查询下级权限通过 p_id
// pub async fn find_all_where_by_p_id(p_id:i64) ->  Result<Vec<Permission>, sqlx::Error> {
//     let pool = DB_POOL.lock().unwrap().as_ref().expect("DB pool not initialized").clone();
//     let rows = sqlx::query_as::<_, Permission>("SELECT * FROM `permission` WHERE parentId = ? ORDER BY `order` ASC ")
//         .bind(p_id)
//         .fetch_all(&pool)
//         .await?;
//
//     Ok(rows)
// }