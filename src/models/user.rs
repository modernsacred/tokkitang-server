use serde::{Deserialize, Serialize};

// 사용자 계정 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
}

impl User {
    pub const NAME: &'static str = "modeler_user";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUser {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
}
