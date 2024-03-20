use axum::{
    extract::{Extension, Json, Path, Query, Request},
    middleware::{self, Next},
};
use chrono::{DateTime, Utc};
use validator::Validate;
use time::OffsetDateTime;
use std::sync::Arc;
use sqlx::MySqlPool;
use crate::tools;
// use crate::db::DB_POOL;
use crate::{
    api::resp::ApiResponse,
    api::{comm_api, user_api},
    db::{DB_POOL,profile_model, role_model, user_model,user_roles_role_model},
};

// 获取用户详情
pub async fn detail(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
) -> Json<ApiResponse<user_api::UserDetailRes>> {
    let uid = curr_user.id;
    let mut rp = user_api::UserDetailRes::default();

    // 通过uid获取 user信息
    let uinfo_result = user_model::find_info_by_id(uid).await;
    let uinfo = match uinfo_result {
        Ok(Some(a)) => a,
        Ok(None) => return Json(ApiResponse::err(&"用户信息不存在")),
        Err(err) => {
            let error_msg = format!("获取用户信息失败:{:?}", err);
            return Json(ApiResponse::err(&error_msg));
        }
    };

    rp.id = uinfo.id;
    rp.username = uinfo.username;
    rp.password = uinfo.password;
    rp.enable = uinfo.enable != 0;
    rp.createTime = uinfo.createTime.to_string();
    rp.updateTime = uinfo.updateTime.to_string();

    // 通过uid获取用户 Profile信息
    let pro_info_result = profile_model::find_info_by_user_id(uid).await;
    let pro_info = match pro_info_result {
        Ok(Some(a)) => a,
        Ok(None) => return Json(ApiResponse::err(&"profile信息不存在")),
        Err(err) => {
            let error_msg = format!("获取profile信息失败:{:?}", err);
            return Json(ApiResponse::err(&error_msg));
        }
    };
    rp.profile = pro_info;

    // 通过uid获取用户 role数组
    let roles_result = role_model::fetch_all_where_user_id(uid).await;
    let roles = match roles_result {
        Ok(rows) => {
            if !rows.is_empty() {
                rows
            } else {
                Vec::new()
            }
        }
        Err(err) => {
            let error_msg = format!("获取角色信息失败:{:?}", err);
            return Json(ApiResponse::err(&error_msg));
        }
    };
    rp.roles = roles.clone();

    if roles.len() > 0 {
        rp.currentRole = roles[0].clone();
    }
    return Json(ApiResponse::succ(Some(rp)));
}

// 获取用户列表
pub async fn list(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    req: Query<user_api::UserListReq>,
) -> Json<ApiResponse<Vec<user_api::UserListItem>>> {
    let result = profile_model::fetch_all_profile(req).await;
    let all_user_profile = match result {
        Ok(u) => u,
        Err(err) => return Json(ApiResponse::err(&format!("获取角色信息失败:{:?}", err))),
    };
    let mut rp = Vec::new();

    for u_profile in all_user_profile {
        // 循环里面的逻辑其实跟 detail详情一样的，只是少了一个 currentRole 字段而已
        let mut tmp = user_api::UserListItem::default();
        let uid = u_profile.userId;
        println!("uuuu-->{:?}", uid);
        // 通过uid获取 user信息
        let uinfo_result = user_model::find_info_by_id(uid).await;
        let uinfo = match uinfo_result {
            Ok(Some(a)) => a,
            Ok(None) => return Json(ApiResponse::err(&"用户信息不存在")),
            Err(err) => return Json(ApiResponse::err(&format!("获取用户信息失败:{:?}", err))),
        };

        tmp.id = uinfo.id;
        tmp.username = uinfo.username;
        tmp.password = uinfo.password;
        tmp.enable = uinfo.enable != 0;
        tmp.createTime = uinfo.createTime.to_string();
        tmp.updateTime = uinfo.updateTime.to_string();

        // 用户 Profile信息
        tmp.profile = u_profile;

        // 通过uid获取用户 role数组
        let roles_result = role_model::fetch_all_where_user_id(uid).await;
        let roles = match roles_result {
            Ok(rows) => {
                if !rows.is_empty() {
                    rows
                } else {
                    Vec::new()
                }
            }
            Err(err) => return Json(ApiResponse::err(&format!("获取角色信息失败:{:?}", err))),
        };
        tmp.roles = roles.clone();
        rp.push(tmp)
    }
    return Json(ApiResponse::succ(Some(rp)));
}

// 状态停用/启用
pub async fn statePatch(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    Path(id): Path<i64>,
    Json(req): Json<user_api::UserStatePatchReq>,
) -> Json<ApiResponse<String>> {
    if let Err(error) = req.validate() {
        return Json(ApiResponse::new(400, None, &format!("{}", error)));
    }
    // println!("req->{:?}", req);
    // println!("id->{:?}", id);
    // println!("curr_user->{:?}", curr_user);
    let update_result = user_model::update_enable_by_id(req.enable,id).await;
    let result = match update_result {
        Ok(_) => {  },
        Err(err) => return Json(ApiResponse::err(&format!("获取用户信息失败:{:?}", err)))
    };
    return Json(ApiResponse::succ(Some("ok".to_string())));
}

// 新增用户
pub async fn add(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    Json(req): Json<user_api::UserAddReq>,
) -> Json<ApiResponse<String>> {
    if let Err(error) = req.validate() {
        return Json(ApiResponse::new(400, None, &format!("{}", error)));
    }
    println!("req->{:?}", req);
    println!("curr_user->{:?}", curr_user);
    let pool = Arc::clone(&DB_POOL)
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .unwrap();
    let mut tx = pool.begin().await?;
    match(tx){
        Ok(tx)=>{
            // 新增用户
            let user_data = user_model::User {
                id:0,
                username: req.username,
                password: tools::md5_crypto(req.password),
                enable: req.enable,
                createTime: Utc::now(),
                updateTime: Utc::now(),
            };
            let add_result = user_model::add_user_by_struct(tx,user_data.clone()).await;
            let mut add_u_id = 0;
            match add_result {
                Ok(res) => {
                    add_u_id = res.last_insert_id();
                },
                Err(err) => {
                    tx.rollback().await?;
                    return Json(ApiResponse::err(&format!("新增用户信息失败:{:?}", err)))
                }
            };

            // 新增用户权限关联
            for roid in req.roleIds{
                let add_data = user_roles_role_model::UserRolesRole {
                    userId: add_u_id,
                    roleId: roid

                };
                let add_result = user_roles_role_model::add_user_role_by_struct(tx,add_data.clone()).await;
                let result = match add_result {
                    Ok(_) => {  },
                    Err(err) => {
                        tx.rollback().await?;
                        return Json(ApiResponse::err(&format!("新增用户权限失败:{:?}", err)))
                    }
                };
            }
            tx.commit().await?;
        },
        Err(err) => {
            return Json(ApiResponse::err(&format!("开启事务失败:{:?}", err)))
        }
    };
    return Json(ApiResponse::succ(Some("ok".to_string())));
}