use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 팀 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub thumbnail_url: Option<String>,
}

impl Team {
    pub const NAME: &'static str = "modeler_team";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(self.id.to_owned()));
        map.insert("name".to_string(), AttributeValue::S(self.name.to_owned()));
        map.insert(
            "description".to_string(),
            AttributeValue::S(self.description.to_owned()),
        );
        map.insert(
            "owner_id".to_string(),
            AttributeValue::S(self.owner_id.to_owned()),
        );

        if let Some(thumbnail_url) = self.thumbnail_url.clone() {
            map.insert(
                "thumbnail_url".to_string(),
                AttributeValue::S(thumbnail_url),
            );
        }

        Some(map)
    }

    #[allow(dead_code)]
    pub fn from_hashmap(hashmap: Option<&HashMap<String, AttributeValue>>) -> Option<Self> {
        let id = hashmap?.get("id")?.as_s().ok()?.to_owned();
        let name = hashmap?.get("name")?.as_s().ok()?.to_owned();
        let description = hashmap?.get("description")?.as_s().ok()?.to_owned();
        let owner_id = hashmap?.get("owner_id")?.as_s().ok()?.to_owned();
        let thumbnail_url = hashmap?
            .get("thumbnail_url")
            .map(|e| e.as_s().ok().map(|e| e.to_owned()).to_owned())
            .flatten();

        Some(Team {
            id: id.to_owned(),
            name,
            description,
            owner_id,
            thumbnail_url,
        })
    }
}
