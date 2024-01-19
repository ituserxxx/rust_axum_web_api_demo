use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRes {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddHelloReq {
    #[validate(length(min = 1, max = 15, message = "用户名必填"))]
    pub name: String,
    #[validate(length(min = 1,max = 11, message = "用户名必填"))]
    pub phone: String,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct AddHelloRes {
    pub name: String,
    pub age: String,
}
