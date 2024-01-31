use axum::extract::Json;
use validator::Validate;
use crate::api::resp::{ApiResponse};
use crate::api::user_api;

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
// pub async fn hello() -> Json<ApiResponse<user_api::HelloRes>> {
//     let uresp = user_api::HelloRes {
//         name: "Alice".to_string(),
//         age: 30,
//     };
//     let api_response = ApiResponse::new(0, Some(uresp), "ok");
//     Json(api_response)
// }
// //
// pub async fn add(Json(req): Json<user_api::AddUserReq>) -> Json<ApiResponse<user_api::AddUserResp>> {
//     if let Err(error) = req.validate() {
//         let error_msg = format!("{}", error);
//         let resp = ApiResponse::new(400, None, &error_msg);
//         return Json(resp)
//     }
//     let uid = user_model::insert_user();
//     match uid {
//         Ok(rows_affected) => {
//             if rows_affected > 0 {
//                 println!("用户插入成功");
//             } else {
//                 let error_msg = "没有插入任何行";
//                 return Json(ApiResponse::err( &error_msg))
//             }
//         }
//         Err(err) => {
//
//             let error_msg = format!("插入操作失败:{}", err);
//             return Json(ApiResponse::err( &error_msg))
//         }
//     }
//      // 初始化返回结构体
//         let rp = user_api::AddUserResp {
// //             id：uid.clone(),
//             name:req.name.clone(),
//             phone: req.phone.clone(),
//         };
//
//     let resp = ApiResponse::succ(Some(rp));
//     Json(resp)
// }
//
