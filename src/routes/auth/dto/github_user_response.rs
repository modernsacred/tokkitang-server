#[derive(serde::Deserialize)]
pub struct GithubUserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}
