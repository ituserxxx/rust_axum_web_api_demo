use axum::extract::Json;
use validator::Validate;
use chrono::Utc;

use crate::api::resp::{ApiResponse};
use crate::api::user_api;
use crate::tools;
use crate::{
    db::user_model,
};

// 获取列表
pub async fn list() -> Json<ApiResponse<user_api::UserListRes>> {
    match  user_model::fetch_all_users().await {
        Ok(list) => {
            // 处理成功获取用户信息的情况
            return Json(ApiResponse::succ(Some(user_api::UserListRes {list:list})))
        }
        Err(err) => {
            // 处理查询失败的情况
            let error_msg = format!("err {}", err);
            return Json(ApiResponse::err(&error_msg))
        }
    }
}

pub async fn add(Json(req): Json<user_api::AddUserReq>) -> Json<ApiResponse<user_api::AddUserResp>> {
    if let Err(error) = req.validate() {
        return Json( ApiResponse::new(400, None, &format!("{}", error)))
    }
    let username = req.username.unwrap_or_default();
    let password = req.password.unwrap_or_default();
    let new_time = Utc::now();
    let insert_user = user_model::User{
        username    : username.to_string(),
        password    : tools::md5_crypto(password.to_string()),
        enable      :1,
        createTime  : new_time,
        updateTime  : new_time,
    };
    match user_model::add_user_by_struct(insert_user).await {
        Ok(insert_res) => {
            if insert_res.rows_affected() > 0 {
                println!("用户插入成功");
                // 初始化返回结构体
                let rp = user_api::AddUserResp {
                    id:insert_res.last_insert_id(),
                };
                return Json( ApiResponse::succ(Some(rp)))
            }
            return Json(ApiResponse::err( &"没有插入任何行"))
        }
        Err(err) => {
            let error_msg = format!("插入操作失败:{}", err);
            return Json(ApiResponse::err( &error_msg))
        }
    }
}
