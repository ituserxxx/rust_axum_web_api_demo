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
    // todo some code
    return Json( ApiResponse::succ(Some(rp)))
}

