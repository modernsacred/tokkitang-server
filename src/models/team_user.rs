use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

// 팀-유저 모델

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum TeamUserAuthority {
    Owner,
    Admin,
    Write,
    Read,
}

impl From<TeamUserAuthority> for String {
    fn from(value: TeamUserAuthority) -> Self {
        match value {
            TeamUserAuthority::Owner => "OWNER".to_string(),
            TeamUserAuthority::Admin => "ADMIN".to_string(),
            TeamUserAuthority::Write => "WRITE".to_string(),
            TeamUserAuthority::Read => "READ".to_string(),
        }
    }
}

impl TryFrom<String> for TeamUserAuthority {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "OWNER" => Ok(TeamUserAuthority::Owner),
            "ADMIN" => Ok(TeamUserAuthority::Admin),
            "WRITE" => Ok(TeamUserAuthority::Write),
            "READ" => Ok(TeamUserAuthority::Read),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUser {
    pub team_id: String,
    pub user_id: String,
    pub authority: TeamUserAuthority,
}

impl TeamUser {
    pub const NAME: &'static str = "modeler_team_user";

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

        Some(map)
    }

    #[allow(dead_code)]
    pub fn from_hashmap(hashmap: Option<&HashMap<String, AttributeValue>>) -> Option<Self> {
        let team_id = hashmap?.get("team_id")?.as_s().ok()?.to_owned();
        let user_id = hashmap?.get("user_id")?.as_s().ok()?.to_owned();
        let authority = hashmap?.get("authority")?.as_s().ok()?.to_owned();

        let authority = authority.try_into().ok()?;

        Some(TeamUser {
            team_id,
            user_id,
            authority,
        })
    }
}
