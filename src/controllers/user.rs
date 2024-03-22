use crate::tools;
use crate::{
    api::resp::ApiResponse,
    api::{comm_api, user_api},
    db::{profile_model, role_model, user_model, user_roles_role_model, DB_POOL},
};
use axum::{
    extract::{Extension, Json, Path, Query, Request},
    middleware::{self, Next},
};
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use std::sync::Arc;
use time::OffsetDateTime;
use validator::Validate;

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
pub async fn state_patch(
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
    let update_result = user_model::update_enable_by_id(req.enable, id).await;
    let result = match update_result {
        Ok(_) => {}
        Err(err) => return Json(ApiResponse::err(&format!("获取用户信息失败:{:?}", err))),
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

    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Json(ApiResponse::err(&format!("开启事务失败:{:?}", err))),
    };
    // 新增用户
    let user_data = user_model::User {
        id: 0,
        username: req.username.clone(),
        password: tools::md5_crypto(req.password),
        enable: req.enable as i8,
        createTime: Utc::now(),
        updateTime: Utc::now(),
    };
    let add_u_id = match user_model::add_user_by_struct(&mut tx, user_data.clone()).await {
        Ok(id) => id,
        Err(err) => {
            if let Err(rollback_err) = tx.rollback().await {
                return Json(ApiResponse::err(&format!(
                    "事务提交失败: {:?}",
                    rollback_err
                )));
            }
            return Json(ApiResponse::err(&format!("新增用户信息失败:{:?}", err)));
        }
    };

    // 新增用户权限关联
    for roid in req.roleIds {
        let add_data = user_roles_role_model::UserRolesRole {
            userId: add_u_id as i64,
            roleId: roid,
        };
        match user_roles_role_model::add_user_role_by_struct(&mut tx, add_data.clone()).await {
            Ok(_) => {}
            Err(err) => {
                if let Err(rollback_err) = tx.rollback().await {
                    return Json(ApiResponse::err(&format!(
                        "事务提交失败: {:?}",
                        rollback_err
                    )));
                }
                return Json(ApiResponse::err(&format!("新增用户权限失败:{:?}", err)));
            }
        };
    }
    // 新增用户信息 profile
    let profile_data = profile_model::Profile {
        id: 0,
        gender: Some(0),
        avatar: String::default(),
        address: Some(String::default()),
        email: Some(String::default()),
        userId: add_u_id as i64,
        nickName: Some(req.username.clone()),
    };
    match profile_model::add_profile_by_struct(&mut tx, profile_data.clone()).await {
        Ok(_) => {}
        Err(err) => {
            if let Err(rollback_err) = tx.rollback().await {
                return Json(ApiResponse::err(&format!(
                    "事务提交失败: {:?}",
                    rollback_err
                )));
            }
            return Json(ApiResponse::err(&format!(
                "新增用户profile信息失败:{:?}",
                err
            )));
        }
    };

    if let Err(commit_err) = tx.commit().await {
        return Json(ApiResponse::err(&format!("事务提交失败: {:?}", commit_err)));
    }
    return Json(ApiResponse::succ(Some("ok".to_string())));
}

// 用户删除
pub async fn del(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<String>> {
    let pool = DB_POOL
        .lock()
        .unwrap()
        .as_ref()
        .expect("DB pool not initialized")
        .clone();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(err) => return Json(ApiResponse::err(&format!("开启事务失败:{:?}", err))),
    };
    // 从删除 user 表删除
    let del_user_ok = match user_model::delete_user_by_id(&mut tx, id).await {
        Ok(is) => is,
        Err(err) => {
            if let Err(rollback_err) = tx.rollback().await {
                return Json(ApiResponse::err(&format!(
                    "事务回滚失败: {:?}",
                    rollback_err
                )));
            }
            return Json(ApiResponse::err(&format!("从user表删除失败:{:?}", err)));
        }
    };

    // 从 user_roles 表删除关系
    let del_user_roles_ok =
        match user_roles_role_model::delete_user_roles_by_user_id(&mut tx, id).await {
            Ok(is) => is,
            Err(err) => {
                if let Err(rollback_err) = tx.rollback().await {
                    return Json(ApiResponse::err(&format!(
                        "事务回滚失败: {:?}",
                        rollback_err
                    )));
                }
                return Json(ApiResponse::err(&format!("删除用户权限失败:{:?}", err)));
            }
        };
    // 从 profile 表删除
    let del_user_profile_ok = match profile_model::delete_profile_by_user_id(&mut tx, id).await {
        Ok(is) => is,
        Err(err) => {
            if let Err(rollback_err) = tx.rollback().await {
                return Json(ApiResponse::err(&format!(
                    "事务回滚失败: {:?}",
                    rollback_err
                )));
            }
            return Json(ApiResponse::err(&format!(
                " 从 profile 表删除失败:{:?}",
                err
            )));
        }
    };
    // 这里统一判断是否删除成功，回滚
    if !del_user_ok || !del_user_roles_ok || !del_user_profile_ok {
        if let Err(rollback_err) = tx.rollback().await {
            return Json(ApiResponse::err(&format!(
                "事务回滚失败: {:?}",
                rollback_err
            )));
        }
        return Json(ApiResponse::err(&"删除失败"));
    }

    if let Err(commit_err) = tx.commit().await {
        return Json(ApiResponse::err(&format!("事务提交失败: {:?}", commit_err)));
    }
    return Json(ApiResponse::succ(Some("ok".to_string())));
}

// 重置密码
pub async fn reset_pwd(){}


// 个人资料修改
pub async fn update_profile(){}


// 分配角色
pub async fn set_user_role(){}