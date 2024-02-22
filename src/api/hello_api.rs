use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub token:  Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct JwtDnRes {

}
