use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 사용자 계정 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEmailForUser {
    pub email: String,
    pub user_id: String,
}

impl IndexEmailForUser {
    pub const NAME: &'static str = "__index__modeler_user__email";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert(
            "email".to_string(),
            AttributeValue::S(self.email.to_owned()),
        );
        map.insert(
            "user_id".to_string(),
            AttributeValue::S(self.user_id.to_owned()),
        );

        Some(map)
    }
}
