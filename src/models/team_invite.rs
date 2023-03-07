use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

use super::TeamUserAuthority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInvite {
    pub team_id: String,
    pub user_id: String,
    pub authority: TeamUserAuthority,
    pub code: String,
}

impl TeamInvite {
    pub const NAME: &'static str = "modeler_team_invite";

    pub fn to_hashmap(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut map = HashMap::new();
        map.insert(
            "team_id".to_string(),
            AttributeValue::S(self.team_id.to_owned()),
        );
        map.insert(
            "user_id".to_string(),
            AttributeValue::S(self.user_id.to_owned()),
        );
        map.insert(
            "authority".to_string(),
            AttributeValue::S(self.authority.to_owned().into()),
        );
        map.insert(
            "code".to_string(),
            AttributeValue::S(self.code.to_owned().into()),
        );

        Some(map)
    }

    pub fn from_hashmap(hashmap: HashMap<String, AttributeValue>) -> Option<Self> {
        let team_id = hashmap.get("team_id")?.as_s().ok()?.to_owned();
        let user_id = hashmap.get("user_id")?.as_s().ok()?.to_owned();
        let authority = hashmap.get("authority")?.as_s().ok()?.to_owned();
        let code = hashmap.get("code")?.as_s().ok()?.to_owned();

        let authority = authority.try_into().ok()?;

        Some(Self {
            team_id,
            user_id,
            authority,
            code,
        })
    }
}
