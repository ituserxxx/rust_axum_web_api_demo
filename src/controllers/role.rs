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
use crate::api::role_api::PermissionItem;


pub async fn permissions_tree( Extension(curr_user): Extension<comm_api::CurrentUser>) ->Json<ApiResponse<Option<Vec<PermissionItem>>>>  {
    let uid = curr_user.id;

    let is_admin_result = user_roles_role_model::find_is_admin_role_by_user_id(uid).await;
    let is_admin = match is_admin_result{
        Ok(a)=>a,
        Err(err)=>{
            let error_msg = format!("获取admin权限信息失败:{:?}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };

    let find_1_level_result = permission_model::find_1_level().await;
    let mut one_arr = match find_1_level_result{
        Ok(a)=>a,
        Err(err)=>{
            let error_msg = format!("获取权限信息失败:{:?}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    };
    let mut rp = role_api::PermissionsTreeResp::default();
    for one in one_arr {
        let m1 = role_api::PermissionItem {
            id: one.id,
            name: one.name,
            code: one.code,
            r#type: one.r#type,
            parentId: one.parentId,
            path: one.path,
            redirect: one.redirect,
            icon: one.icon,
            component: one.component,
            layout: one.layout,
            keepAlive: one.keepAlive,
            method: one.method,
            description: one.description,
            show: one.show,
            enable:one.enable,
            order: one.order,
            children: Some(vec![]),
        };
        // let find_2_result = permission_model::find_all_where_by_p_id(one_arr[i1].id).await;
        // if let Ok(two_arr) = find_2_result {
        //     // for i2 in 0..two_arr.len() {
        //     //     let find_3_result = permission_model::find_all_where_by_p_id(two_arr[i2].id).await;
        //     //     if let Ok(three_arr) = find_3_result {
        //     //         let three_arr_rc: Vec<Rc<Permission>> = three_arr.into_iter().map(Rc::new).collect();
        //     //         // 将三级权限列表赋值给二级权限的子节点
        //     //         two_arr[i2].children = Some(three_arr_rc);
        //     //     }
        //     // }
        //     // 将二级权限列表赋值给一级权限的子节点
        //     one.children = Some(two_arr);
        // }
        if let Some(ref mut list) = rp.list {
            list.push(m1);
        }
        // rp.list.push(Some(m1));
    }

    // let mut rp = role_api::PermissionsTreeResp::default();
    // rp.list = Some(one_arr);

    return Json(ApiResponse::succ(Some(rp.list)));
    // let error_msg = format!("获取权限信息失败:{}", "xx");
    // return Json(ApiResponse::succ( &error_msg))

}

