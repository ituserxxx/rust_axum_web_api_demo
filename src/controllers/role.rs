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
        user_roles_role_model,
    },
    api::{
        user_api,
        role_api,
        comm_api,
    },
    api::resp::{
        ApiResponse
    },
};


pub async fn permissions_tree( Extension(curr_user): Extension<comm_api::CurrentUser>) -> Json<ApiResponse<role_api::PermissionsTreeResp>> {
    let uid = curr_user.id;
    let mut rp = role_api::PermissionsTreeResp::default();

    let is_admin_result = user_roles_role_model::find_is_admin_role_by_user_id(uid).await;
    let is_admin = match is_admin_result{
        Ok(a)=>a,
        Err(err)=>{
            let error_msg = format!("获取admin权限信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };

    let find_1_level_result = find_1_level = permissions_model::find_1_level().await;
    let one_arr = match find_1_level_result{
        Ok(Some(a))=>a,
        Err(err)=>{
            let error_msg = format!("获取权限信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };

    // todo some code
    return Json( ApiResponse::succ(Some(rp)))
}

