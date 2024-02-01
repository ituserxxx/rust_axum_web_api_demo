use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    db::user_model,
};

// 新增用户 test ok
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct AddUserReq {
    #[serde(default)]
    #[validate(required,length(min=1,message="username 字段必传"))]
    pub username:  Option<String>,
    #[serde(default)]
    #[validate(required,length(min=1,message="password 字段必传"))]
    pub password:  Option<String>,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct AddUserResp {
    pub id: u64,
}

// test ok
// 用户列表
#[derive(Debug, Deserialize, Serialize)]
pub struct UserListRes {
    pub list: Vec<user_model::User>,
}

// 获取用户详情
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserInfoReq {
    #[serde(default)]
    #[validate(required)]
    pub id:  Option<i64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoRes {
    pub info: user_model::User,
}
