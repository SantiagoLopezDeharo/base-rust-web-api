use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct UserDto {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub password: String,
}

impl UserDto {
    pub fn new(id: String, username: String, password: String) -> Self {
        Self {
            id,
            username,
            password,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("Invalid user JSON: {}", e))
    }
}

#[derive(Deserialize, Serialize)]

pub struct UserRet {
    pub id: String,
    pub username: String,
}

impl UserRet {
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("Invalid user JSON: {}", e))
    }
}
