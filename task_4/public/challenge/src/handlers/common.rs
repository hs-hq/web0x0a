use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Error {
    pub message: String,
    // pub code: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserStruct {
    pub username: String,
    pub email: String,
}
