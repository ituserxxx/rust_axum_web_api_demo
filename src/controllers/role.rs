use validator::Validate;
use chrono::Utc;
use axum::{
    middleware::{self, Next},
    extract::{Request, Extension,Json},
};

use crate::tools;
use crate::{
    db::{
        user_model,
        profile_model,
        role_model,
    },
    api::{
        user_api,
        comm_api,
    },
    api::resp::{
        ApiResponse
    },

};

// 获取用户详情
pub async fn permissions_tree( Extension(curr_user): Extension<comm_api::CurrentUser>) -> Json<ApiResponse<user_api::UserDetailRes>> {
    let uid = curr_user.id;
    let mut rp = user_api::UserDetailRes::default();

    // 通过uid获取 user信息
    let uinfo_result = user_model::find_info_by_id(uid).await;
    let uinfo = match uinfo_result {
        Ok(Some(a)) =>a,
        Ok(None) => {
            return Json(ApiResponse::err( &"用户信息不存在"))
        },
        Err(err)=>{
            let error_msg = format!("获取用户信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
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
        Ok(Some(a)) =>a,
        Ok(None) => {
            return Json(ApiResponse::err( &"profile信息不存在"))
        },
        Err(err)=>{
            let error_msg = format!("获取profile信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    rp.profile = pro_info;

    // 通过uid获取用户 role数组

    let roles_result = role_model::fetch_all_where_user_id(uid).await;
    let roles = match roles_result {
        Ok(rows) =>{
            if !rows.is_empty() {
                rows
            } else {
                Vec::new()
            }
        },
        Err(err)=>{
            let error_msg = format!("获取角色信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    rp.roles=roles.clone();
    if roles.len() >0 {
        rp.currentRole=roles[0].clone();
    }
    return Json( ApiResponse::succ(Some(rp)))
}

