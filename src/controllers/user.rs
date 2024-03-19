use validator::Validate;
use chrono::Utc;
use axum::{
    middleware::{self, Next},
    extract::{Request, Extension,Json,Query},
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
pub async fn detail(
    Extension(curr_user): Extension<comm_api::CurrentUser>
) -> Json<ApiResponse<user_api::UserDetailRes>>
{
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
            let error_msg = format!("获取用户信息失败:{:?}", err);
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
            let error_msg = format!("获取profile信息失败:{:?}", err);
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
            let error_msg = format!("获取角色信息失败:{:?}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    rp.roles=roles.clone();

    if roles.len() >0 {
        rp.currentRole=roles[0].clone();
    }
    return Json( ApiResponse::succ(Some(rp)))
}

// 获取用户列表
pub async fn list(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    req: Query<user_api::UserListReq>,
) -> Json<ApiResponse<Vec<user_api::UserListItem>>>
{

    let result = user_model::fetch_all_users(req).await;
    let all_user = match result {
        Ok(u) => u,
        Err(err) => return Json(ApiResponse::err( &format!("获取角色信息失败:{:?}", err)))
    };
    let mut rp = Vec::new();
    println!("lllll ->{:?}",all_user.len());
    for u in all_user{
        // 循环里面的逻辑其实跟 detail详情一样的，只是少了一个 currentRole 字段而已
        let mut tmp = user_api::UserListItem::default();
        let uid = u.id;
        // 通过uid获取 user信息
        let uinfo_result = user_model::find_info_by_id(uid).await;
        let uinfo = match uinfo_result {
            Ok(Some(a)) =>a,
            Ok(None) =>return Json(ApiResponse::err( &"用户信息不存在")),
            Err(err)=>return Json(ApiResponse::err( &format!("获取用户信息失败:{:?}", err)))
        };

        tmp.id = uinfo.id;
        tmp.username = uinfo.username;
        tmp.password = uinfo.password;
        tmp.enable = uinfo.enable != 0;
        tmp.createTime = uinfo.createTime.to_string();
        tmp.updateTime = uinfo.updateTime.to_string();

        // 通过uid获取用户 Profile信息
        let pro_info_result = profile_model::find_info_by_user_id(uid).await;
        let pro_info = match pro_info_result {
            Ok(Some(a)) =>a,
            Ok(None) => return Json(ApiResponse::err( &"用户信息不存在")),
            Err(err)=>return Json(ApiResponse::err( &format!("获取profile信息失败:{:?}", err)))
        };
        tmp.profile = pro_info;
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
            Err(err)=>return Json(ApiResponse::err( &format!("获取角色信息失败:{:?}", err)))
        };
        tmp.roles=roles.clone();
        rp.push(tmp)
    }
    return Json( ApiResponse::succ(Some(rp)))
}