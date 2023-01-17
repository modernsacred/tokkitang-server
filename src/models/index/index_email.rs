use serde::{Deserialize, Serialize};

// 사용자 계정 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEmailForUser {
    pub email: String,
    pub user_ids: Vec<String>,
}

impl IndexEmailForUser {
    pub const NAME: &'static str = "__index__modeler_user__email";
}
