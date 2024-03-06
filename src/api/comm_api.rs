use serde::{Deserialize, Serialize};



#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtDnReq {
    #[serde(default)]
    pub name: Option<String>,
}

// Define the state to store captcha and user session
#[derive(Debug, Clone)]
struct State {
    captcha: String,
}

impl State {
    fn new() -> Self {
        Self {
            captcha: String::new(),
        }
    }
}