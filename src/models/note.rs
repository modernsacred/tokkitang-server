use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 엔티티 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub project_id: String,
    pub content: String,
    pub x: String,
    pub y: String,
}

impl Note {
    pub const NAME: &'static str = "modeler_note";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(self.id.to_owned()));
        map.insert(
            "project_id".to_string(),
            AttributeValue::S(self.project_id.to_owned()),
        );
        map.insert(
            "content".to_string(),
            AttributeValue::S(self.content.to_owned()),
        );
        map.insert("x".to_string(), AttributeValue::S(self.x.to_owned()));
        map.insert("y".to_string(), AttributeValue::S(self.y.to_owned()));

        Some(map)
    }

    pub fn from_hashmap(hashmap: HashMap<String, AttributeValue>) -> Option<Self> {
        let id = hashmap.get("id")?.as_s().ok()?.to_owned();
        let project_id = hashmap.get("project_id")?.as_s().ok()?.to_owned();
        let content = hashmap.get("content")?.as_s().ok()?.to_owned();
        let x = hashmap.get("x")?.as_s().ok()?.to_owned();
        let y = hashmap.get("y")?.as_s().ok()?.to_owned();

        Some(Self {
            id,
            project_id,
            content,
            x,
            y,
        })
    }
}
