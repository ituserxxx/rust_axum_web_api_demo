use serde::{Deserialize, Serialize};
use validator::Validate;
mod db {
    pub mod user_model; // 不要在此处加上分号
}
use db::user_model;


//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct HelloRes {
//     pub name: String,
//     pub age: u32,
// }
//
// #[derive(Debug, Validate, Deserialize, Serialize)]
// pub struct AddUserReq {
//     #[validate(length(min = 1, max = 15, message = "用户名必填"))]
//     pub name: String,
//     #[validate(required,length(min = 1, message = "用户名必填"))]
//     pub phone: String,
// }
//
//
//
// #[derive(Debug,Deserialize, Serialize)]
// pub struct AddUserResp {
//     pub name: String,
//     pub phone: String,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct user_list_res {
    pub list: Vec<user_model::User>,
}
