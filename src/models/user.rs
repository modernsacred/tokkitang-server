use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 사용자 계정 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
    pub github_id: Option<String>,
}

impl User {
    pub const NAME: &'static str = "modeler_user";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(self.id.to_owned()));
        map.insert(
            "nickname".to_string(),
            AttributeValue::S(self.nickname.to_owned()),
        );
        map.insert(
            "email".to_string(),
            AttributeValue::S(self.email.to_owned()),
        );
        map.insert(
            "password".to_string(),
            AttributeValue::S(self.password.to_owned()),
        );
        map.insert(
            "password_salt".to_string(),
            AttributeValue::S(self.password_salt.to_owned()),
        );

        Some(map)
    }

    pub fn from_hashmap(hashmap: Option<&HashMap<String, AttributeValue>>) -> Option<Self> {
        let id = hashmap?.get("id")?.as_s().ok()?;
        let nickname = hashmap?.get("nickname")?.as_s().ok()?;
        let email = hashmap?.get("email")?.as_s().ok()?;
        let password = hashmap?.get("password")?.as_s().ok()?;
        let password_salt = hashmap?.get("password_salt")?.as_s().ok()?;
        let github_id = hashmap?
            .get("github_id")
            .map(|e| e.as_s().ok().map(|e| e.to_owned()))
            .flatten();

        Some(User {
            id: id.to_owned(),
            nickname: nickname.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
            password_salt: password_salt.to_owned(),
            github_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertUser {
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub password_salt: String,
}
