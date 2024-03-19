use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    db::user_model,
    db::profile_model,
    db::role_model,
};

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

#[derive(Debug,Default, Deserialize, Serialize)]
pub struct UserListReq {
    pub enable: Option<u32>,  // 可选-状态：1-启用（默认）0-停用
    pub gender: Option<u32>,  // 可选-性别：1-男，2-女
    pub username: Option<String>,  // 可选-用户名搜索
    #[allow(non_snake_case)]
    pub pageNo: Option<u32>,  // 可选-页码 默认1
    #[allow(non_snake_case)]
    pub pageSize: Option<u32>,  // 可选-数量 默认10
}

#[derive(Debug,Default, Deserialize, Serialize)]
pub struct UserListItem {
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
}
