use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AlertExternalPayload {
    #[serde(rename = "type")]  
    source_type: Option<String>,
    origin_hash_id: Option<Uuid>,
    origin_hash_type: Option<String>,
    origin_frame_id: Option<Uuid>,
    origin_frame_second: Option<f64>,
    origin_frame_url: Option<String>,
    origin_asset_id: Option<Uuid>,
    similar_frame_id: Option<Uuid>,
    similar_frame_second: Option<f64>,
    similar_frame_url: Option<String>,
    similar_asset_id: Option<Uuid>,
}