use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRes {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct AddUserReq {
    #[validate(length(min = 1, max = 15, message = "用户名必填"))]
    pub name: Option<String>,
    #[validate(required,length(min = 1, message = "用户名必填"))]
    pub phone: Option<String>,
}


