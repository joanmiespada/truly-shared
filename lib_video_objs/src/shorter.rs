use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShorter {
    pub url_file: Url,
    pub asset_id: Uuid,
    pub user_id: String,
    pub hash: String,
    pub hash_algorithm: String,
    pub keep_original: bool,
}

impl std::fmt::Display for CreateShorter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "url: {} asset_id: {}",
            self.url_file.to_string(),
            self.asset_id.to_string()
        )
    }
}