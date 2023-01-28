use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamResponse {
    pub success: bool,
    pub team_id: String,
}
