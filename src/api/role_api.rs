use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    db::user_model,
    db::profile_model,
    db::permission_model,
    db::role_model,
};

#[derive(Debug,Default, Deserialize, Serialize)]
pub struct PermissionsTreeResp {
    pub list       : Vec<permission_model::Permission>,
}