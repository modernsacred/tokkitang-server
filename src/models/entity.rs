use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 엔티티 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub project_id: String,
    pub logical_name: String,
    pub physical_name: String,
    pub comment: String,
    pub columns: Vec<Column>,
}

impl Entity {
    pub const NAME: &'static str = "modeler_entity";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(self.id.to_owned()));
        map.insert(
            "project_id".to_string(),
            AttributeValue::S(self.project_id.to_owned()),
        );
        map.insert(
            "logical_name".to_string(),
            AttributeValue::S(self.logical_name.to_owned()),
        );
        map.insert(
            "physical_name".to_string(),
            AttributeValue::S(self.physical_name.to_owned()),
        );
        map.insert(
            "comment".to_string(),
            AttributeValue::S(self.comment.to_owned()),
        );

        if let Ok(colmns) = serde_json::to_string(&self.columns) {
            map.insert("colmns".to_string(), AttributeValue::S(colmns));
        }

        Some(map)
    }

    pub fn from_hashmap(hashmap: Option<&HashMap<String, AttributeValue>>) -> Option<Self> {
        let id = hashmap?.get("id")?.as_s().ok()?.to_owned();
        let project_id = hashmap?.get("project_id")?.as_s().ok()?.to_owned();
        let logical_name = hashmap?.get("logical_name")?.as_s().ok()?.to_owned();
        let physical_name = hashmap?.get("physical_name")?.as_s().ok()?.to_owned();
        let comment = hashmap?.get("comment")?.as_s().ok()?.to_owned();
        let columns = hashmap?
            .get("columns")?
            .as_s()
            .ok()
            .map(|e| e.to_owned())
            .unwrap_or("".to_string());
        let columns = serde_json::from_str(columns.as_str()).unwrap_or(vec![]);

        Some(Self {
            id,
            project_id,
            logical_name,
            physical_name,
            comment,
            columns,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: String,
    pub is_primary_key: bool,
    pub logical_name: String,
    pub physical_name: String,
    pub data_type: String,
    pub nullable: bool,
    pub comment: String,
}
