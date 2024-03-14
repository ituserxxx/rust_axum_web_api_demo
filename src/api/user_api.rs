use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    db::user_model,
    db::profile_model,
    db::role_model,
};

#[allow(non_snake_case)]
#[derive(Debug,Default, Deserialize, Serialize)]
pub struct UserDetailRes {
    pub id          : i64,
    pub username    : String,
    pub password    : String,
    pub enable      : bool,
    #[allow(non_snake_case)]
    pub createTime  : String,
    #[allow(non_snake_case)]
    pub updateTime  : String,
    pub profile     : profile_model::Profile,
    pub roles       : Vec<role_model::Role>,
    #[allow(non_snake_case)]
    pub currentRole : role_model::Role,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoRes {
    pub info: user_model::User,
}

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


// 删除用户
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserDelReq {
    #[serde(default)]
    #[validate(required)]
    pub id:  Option<i64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDelRes {

}