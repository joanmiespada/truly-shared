//Data structure must match with data coming from matchapi

//use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashResult {
    pub asset_id: Uuid,
    #[serde(rename = "type")]  
    pub ttype: String,
    pub result: String,
}

