#[derive(serde::Serialize, Debug)]
pub struct UploadImageResponse {
    pub success: bool,
    pub image_url: String,
}
