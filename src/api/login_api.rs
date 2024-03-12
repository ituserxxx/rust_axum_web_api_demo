use serde::{Deserialize, Serialize};
use validator::Validate;
// 会话数据结构体
#[derive(Debug, Default)]
pub struct SessionData {
    pub captcha: Option<String>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginReq {
    #[serde(default)]
    pub captcha: String,
    #[serde(default)]
    pub password:  String,
    #[serde(default)]
    pub username:  String,
}

// #[derive(Debug, Validate, Deserialize, Serialize)]
// pub struct LoginReq2 {
//     #[serde(default)]
//     #[validate(required,length(min=1,message="验证码必填"))]
//     pub captcha: Option<String>,
//     #[serde(default)]
//     #[validate(required,length(min=1,message="密码必填"))]
//     pub password:  Option<String>,
//     #[serde(default)]
//     #[validate(required,length(min=1,message="用户名必填"))]
//     pub username:  Option<String>,
// }

#[derive(Debug,Deserialize, Serialize)]
pub struct LoginResp {
    pub accessToken: String,
}