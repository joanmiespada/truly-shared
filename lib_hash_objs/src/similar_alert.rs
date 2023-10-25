use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AlertExternalPayload {
    #[serde(rename = "type")]  
    pub source_type: Option<String>,
    pub origin_hash_id: Option<Uuid>,
    pub origin_hash_type: Option<String>,
    pub origin_frame_id: Option<Uuid>,
    pub origin_frame_second: Option<f64>,
    pub origin_frame_url: Option<String>,
    pub origin_asset_id: Option<Uuid>,
    pub similar_frame_id: Option<Uuid>,
    pub similar_frame_second: Option<f64>,
    pub similar_frame_url: Option<String>,
    pub similar_asset_id: Option<Uuid>,
}