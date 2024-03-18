use validator::Validate;
use chrono::Utc;
use axum::{
    middleware::{self, Next},
    extract::{Request, Extension,Json},
};
use std::rc::Rc;

use crate::tools;
use crate::{
    db::{
        user_model,
        profile_model,
        role_model,
        user_roles_role_model,
        permission_model,
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


pub async fn permissions_tree( Extension(curr_user): Extension<comm_api::CurrentUser>) ->Json<ApiResponse<Vec<permission_model::Permission>>>  {
    let uid = curr_user.id;

    let is_admin_result = user_roles_role_model::find_is_admin_role_by_user_id(uid).await;
    let is_admin = match is_admin_result{
        Ok(a)=>a,
        Err(err)=>{
            let error_msg = format!("获取admin权限信息失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };


    // let find_1_level_result = permission_model::find_1_level().await;
    // let mut one_arr = match find_1_level_result{
    //     Ok(a)=>a,
    //     Err(err)=>{
    //         let error_msg = format!("获取权限信息失败:{}", err);
    //         return Json(ApiResponse::err( &error_msg))
    //     }
    // };


    // for one in one_arr {
    //     let find_2_result = permission_model::find_all_where_by_p_id(one_arr[i1].id).await;
    //     if let Ok(two_arr) = find_2_result {
    //         // for i2 in 0..two_arr.len() {
    //         //     let find_3_result = permission_model::find_all_where_by_p_id(two_arr[i2].id).await;
    //         //     if let Ok(three_arr) = find_3_result {
    //         //         let three_arr_rc: Vec<Rc<Permission>> = three_arr.into_iter().map(Rc::new).collect();
    //         //         // 将三级权限列表赋值给二级权限的子节点
    //         //         two_arr[i2].children = Some(three_arr_rc);
    //         //     }
    //         // }
    //         // 将二级权限列表赋值给一级权限的子节点
    //         one.children = Some(two_arr);
    //     }
    // }

    // let mut rp = role_api::PermissionsTreeResp::default();
    // rp.list = Some(one_arr);
    let error_msg = format!("获取权限信息失败:{}", "xx");
    return Json(ApiResponse::err( &error_msg))

}

