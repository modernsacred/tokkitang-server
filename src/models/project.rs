use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 팀 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub team_id: String,
    pub description: String,
    pub name: String,
    pub thumbnail_url: Option<String>,
    pub x: String,
    pub y: String,
}

impl Project {
    pub const NAME: &'static str = "modeler_project";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(self.id.to_owned()));
        map.insert("name".to_string(), AttributeValue::S(self.name.to_owned()));
        map.insert(
            "description".to_string(),
            AttributeValue::S(self.description.to_owned()),
        );
        map.insert(
            "team_id".to_string(),
            AttributeValue::S(self.team_id.to_owned()),
        );
        map.insert("x".to_string(), AttributeValue::S(self.x.to_owned()));
        map.insert("y".to_string(), AttributeValue::S(self.y.to_owned()));

        if let Some(thumbnail_url) = self.thumbnail_url.clone() {
            map.insert(
                "thumbnail_url".to_string(),
                AttributeValue::S(thumbnail_url),
            );
        }

        Some(map)
    }

    #[allow(dead_code)]
    pub fn from_hashmap(hashmap: HashMap<String, AttributeValue>) -> Option<Self> {
        let id = hashmap.get("id")?.as_s().ok()?.to_owned();
        let name = hashmap.get("name")?.as_s().ok()?.to_owned();
        let description = hashmap.get("description")?.as_s().ok()?.to_owned();
        let team_id = hashmap.get("team_id")?.as_s().ok()?.to_owned();
        let x = hashmap.get("x")?.as_s().ok()?.to_owned();
        let y = hashmap.get("y")?.as_s().ok()?.to_owned();
        let thumbnail_url = hashmap
            .get("thumbnail_url")
            .map(|e| e.as_s().ok().map(|e| e.to_owned()).to_owned())
            .flatten();

        Some(Self {
            id: id.to_owned(),
            name,
            description,
            team_id,
            thumbnail_url,
            x,
            y,
        })
    }
}
