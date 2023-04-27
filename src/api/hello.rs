use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRes {
    pub name: String,
    pub age: u32,
}