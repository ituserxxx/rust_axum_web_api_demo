use axum::{
    extract::{Extension, Json, Query, Request},
    middleware::{self, Next},
};
use chrono::Utc;
use std::rc::Rc;
use validator::Validate;

use crate::tools;
use crate::{
    api::resp::ApiResponse,
    api::{comm_api, role_api, user_api},
    db::{
        permission_model, profile_model, role_model, role_permissions_permission, user_model,
        user_roles_role_model,
    },
};

// 所有角色
pub async fn all(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
) -> Json<ApiResponse<Vec<role_model::Role>>> {
    return match role_model::fetch_all_role().await {
        Ok(a) => Json(ApiResponse::succ(Some(a))),
        Err(err) => Json(ApiResponse::err(&format!("获取所有权限失败:{:?}", err))),
    };
}
// 当前用户权限树
pub async fn permissions_tree(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
) -> Json<ApiResponse<Option<Vec<role_api::PermissionItem>>>> {
    let uid = curr_user.id;
    let is_admin_result = user_roles_role_model::find_is_admin_role_by_user_id(uid).await;
    let is_admin = match is_admin_result {
        Ok(a) => a,
        Err(err) => {
            let error_msg = format!("获取用户admin权限信息失败:{:?}", err);
            return Json(ApiResponse::err(&error_msg));
        }
    };
    let mut one_arr: Vec<permission_model::Permission> = Vec::new();
    if is_admin {
        let find_1_level_result = permission_model::find_1_level().await;
        one_arr = match find_1_level_result {
            Ok(a) => a,
            Err(err) => {
                let error_msg = format!("获取所有权限信息失败:{:?}", err);
                return Json(ApiResponse::err(&error_msg));
            }
        };
    } else {
        let find_1_level_result = permission_model::find_1_level_where_by_user_id(uid).await;
        one_arr = match find_1_level_result {
            Ok(a) => a,
            Err(err) => {
                let error_msg = format!("获取用户权限信息失败:{:?}", err);
                return Json(ApiResponse::err(&error_msg));
            }
        };
    }
    let mut rp_arr: Vec<role_api::PermissionItem> = Vec::new();
    for one in one_arr {
        let mut m1 = Box::new(role_api::PermissionItem {
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
            enable: one.enable,
            order: one.order,
            children: Some(Vec::new()),
        });
        let find_2_result = permission_model::find_all_where_by_p_id(one.id).await;
        if let Ok(two_arr) = find_2_result {
            let mut two_children: Vec<role_api::PermissionItem> = Vec::new();
            for two in two_arr {
                let mut m2 = role_api::PermissionItem {
                    id: two.id,
                    name: two.name,
                    code: two.code,
                    r#type: two.r#type,
                    parentId: two.parentId,
                    path: two.path,
                    redirect: two.redirect,
                    icon: two.icon,
                    component: two.component,
                    layout: two.layout,
                    keepAlive: two.keepAlive,
                    method: two.method,
                    description: two.description,
                    show: two.show,
                    enable: two.enable,
                    order: two.order,
                    children: Some(Vec::new()),
                };
                let find_3_result = permission_model::find_all_where_by_p_id(two.id).await;
                if let Ok(three_arr) = find_3_result {
                    let mut three_children: Vec<role_api::PermissionItem> = Vec::new();
                    for three in three_arr {
                        let m3 = role_api::PermissionItem {
                            id: three.id,
                            name: three.name,
                            code: three.code,
                            r#type: three.r#type,
                            parentId: three.parentId,
                            path: three.path,
                            redirect: three.redirect,
                            icon: three.icon,
                            component: three.component,
                            layout: three.layout,
                            keepAlive: three.keepAlive,
                            method: three.method,
                            description: three.description,
                            show: three.show,
                            enable: three.enable,
                            order: three.order,
                            children: Some(Vec::new()),
                        };
                        three_children.push(m3)
                    }
                    m2.children = Some(three_children.into_iter().map(Box::new).collect());
                }
                two_children.push(m2)
            }
            // 将二级权限列表赋值给一级权限的子节点
            m1.children = Some(two_children.into_iter().map(Box::new).collect());
        }
        rp_arr.push(*m1);
    }
    return Json(ApiResponse::succ(Some(Some(rp_arr))));
}

// 角色列表
pub async fn page_list(
    Extension(curr_user): Extension<comm_api::CurrentUser>,
    req: Query<role_api::RolePageReq>,
) -> Json<ApiResponse<role_api::RolePageResp>> {
    let result = role_model::fetch_all_by_req(req).await;
    let all_role = match result {
        Ok(u) => u,
        Err(err) => return Json(ApiResponse::err(&format!("获取列表信息失败:{:?}", err))),
    };
    let mut list_item = Vec::new();
    for ro in all_role {
        let mut tmp = role_api::RolePageItem::default();
        tmp.id = ro.id;
        tmp.name = ro.name;
        tmp.code = ro.code;
        tmp.enable = ro.enable != 0;
        // 获取 permission ids
        let pmids_result =
            role_permissions_permission::fetch_permission_ids_where_role_id(tmp.id).await;
        let perm_ids = match pmids_result {
            Ok(rows) => {
                if !rows.is_empty() {
                    rows
                } else {
                    Vec::new()
                }
            }
            Err(err) => {
                return Json(ApiResponse::err(&format!(
                    "获取角色菜单权限列表失败:{:?}",
                    err
                )))
            }
        };
        tmp.permissionIds = Some(perm_ids);
        list_item.push(tmp)
    }
    let mut rp = role_api::RolePageResp {
        pageData: Some(list_item),
    };
    return Json(ApiResponse::succ(Some(rp)));
}
