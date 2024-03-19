use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,Default,  Deserialize, Serialize)]
pub struct PermissionItem {
    pub id          : i64,
    pub name        : String,
    pub code        : String,
    pub r#type      : String,
    #[allow(non_snake_case)]
    pub parentId    : Option<i64>,
    pub path        : Option<String>,
    pub redirect    : Option<String>,
    pub icon        : Option<String>,
    pub component   : Option<String>,
    pub layout      : Option<String>,
    #[allow(non_snake_case)]
    pub keepAlive   : Option<i8>,
    pub method      : Option<String>,
    pub description : Option<String>,
    pub show        : i8,
    pub enable      : i8,
    pub order       : i64,
    pub children: Option<Vec<Box<PermissionItem>>>
}

