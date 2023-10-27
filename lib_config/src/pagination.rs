use std::collections::HashMap;
use std::option::Option;
use maplit::hashmap;
use serde_json::{Value, Map};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use crate::environment::EnvironmentVariables;

pub const PAGINATION_TOKEN_ENCODER: &str = "PAGINATION_TOKEN_ENCODER";
//aws_sdk_dynamodb::types::AttributeValue
pub fn pagination_encode_token<T: Serialize>( env_vars: &EnvironmentVariables, data: Option<HashMap<String,T>>) -> Option<String> {
    match data {
        Some(d) if d.is_empty() => None,
        Some(d) => {
            // Convert data to JSON
            let data_str = serde_json::to_string(&d).expect("Failed to serialize data");
            // Create a hash checksum of the data
            let h_1= env_vars.pagination_token_encoder().expect("PAGINATION_TOKEN_ENCODER not set in environment");
            // let h_1 = env::var(PAGINATION_TOKEN_ENCODER).expect("PAGINATION_TOKEN_ENCODER not set in environment");
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", data_str, h_1));
            let checksum = format!("{:x}", hasher.finalize());
            // Combine data and checksum
             let combined = serde_json::to_string(&hashmap!{
                 "data" => data_str, // Value::String(data_str)
                 "checksum" => checksum   //   Value::String(checksum)
             }).expect("Failed to serialize combined data");
            // Encode as base64
            let combined_bytes: Vec<u8> = combined.as_bytes().to_vec();
            let b64 =general_purpose::STANDARD_NO_PAD.encode(combined_bytes);
            //Some(base64::encode(combined))
            Some(b64)
        },
        None => None
    }
}

pub fn pagination_decode_token<T: DeserializeOwned >(env_vars: &EnvironmentVariables, token: Option<String>) -> Result<Option<HashMap<String, T>>, &'static str> {
    match token {
        Some(t) if t.is_empty() => Ok(None),
        Some(t) => {
            // Decode from base64
            let b64 =general_purpose::STANDARD_NO_PAD.decode(t).unwrap();
            //let decoded = base64::decode(&t).map_err(|_| "Failed to decode base64")?;
            let combined_str = String::from_utf8(b64).map_err(|_| "Failed to convert decoded bytes to string")?;
            let combined: Map<String, Value> = serde_json::from_str(&combined_str).map_err(|_| "Failed to deserialize combined data")?;
            
            // Validate checksum
            let original_checksum = combined.get("checksum")
                .and_then(|v| v.as_str())
                .ok_or("Checksum missing or not a string")?;
            let data_str = combined.get("data")
                .and_then(|v| v.as_str())
                .ok_or("Data missing")?;

            //let data_str = serde_json::to_string(data).map_err(|_| "Failed to serialize data")?;
            
            //let h_1 = env::var(PAGINATION_TOKEN_ENCODER).map_err(|_| "PAGINATION_TOKEN_ENCODER not set in environment")?;
            let h_1= env_vars.pagination_token_encoder().expect("PAGINATION_TOKEN_ENCODER not set in environment");
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", data_str, h_1));
            let expected_checksum = format!("{:x}", hasher.finalize());
            
            if &expected_checksum != original_checksum {
                return Err("Pagination decode token checksum does not match. Token may have been tampered!");
            }

            Ok(Some(serde_json::from_str(data_str.clone()).map_err(|_| "Failed to deserialize data into HashMap")?))
        },
        None => Ok(None)
    }
}
