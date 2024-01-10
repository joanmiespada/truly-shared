use std::collections::HashMap;
use std::option::Option;
use maplit::hashmap;
use serde_json::{Value, Map};
use serde::de::DeserializeOwned;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use aws_sdk_dynamodb::types::AttributeValue;

pub const PAGINATION_TOKEN_ENCODER: &str = "PAGINATION_TOKEN_ENCODER";

pub fn pagination_encode_token<T: Serialize>( encoder: Option<String>, data: Option<HashMap<String,T>>) -> Option<String> {
    match data {
        Some(d) if d.is_empty() => None,
        Some(d) => {
            // Convert data to JSON
            let data_str = serde_json::to_string(&d).expect("Failed to serialize data");
            // Create a hash checksum of the data
            let h_1 = encoder.expect("pagination encoder not set");
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

pub fn pagination_decode_token<T: DeserializeOwned >(encoder:Option<String>, token: Option<String>) -> Result<Option<HashMap<String, T>>, &'static str> {
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

            let h_1 = encoder.expect("pagination encoder not set");
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", data_str, h_1));
            let expected_checksum = format!("{:x}", hasher.finalize());
            
            if &expected_checksum != original_checksum {
                return Err("Pagination decode token checksum does not match. Token may have been tampered!");
            }

            Ok(Some(serde_json::from_str(data_str).map_err(|_| "Failed to deserialize data into HashMap")?))
        },
        None => Ok(None)
    }
}



#[derive(Debug)]
pub struct AttributeValueWrapper{
    att: AttributeValue
}

impl AttributeValueWrapper {

    pub fn new(value: &AttributeValue) -> Self {
        AttributeValueWrapper {
            att: value.clone()
        }
    }
    pub fn get(self) -> AttributeValue {
        self.att
    }
    pub fn set(mut self, value: &AttributeValue ) {
        self.att = value.clone();
    }
}

impl<'de> Deserialize<'de> for AttributeValueWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(AttributeValueWrapper { att: AttributeValue::S(s) })
    }
}

impl Serialize for AttributeValueWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer + serde::ser::Serializer,
    {
        match &self.att {
            //AttributeValue::S(s) => s.serialize(serializer),

            AttributeValue::S(s) => serializer.serialize_str(s),
            AttributeValue::N(n) => serializer.serialize_str(n),
            //AttributeValue::B(b) => serializer.serialize_bytes(b ),
            AttributeValue::Ss(ss) => serializer.serialize_str(&ss.join(",")),
            AttributeValue::Ns(ns) => serializer.serialize_str(&ns.join(",")),
            //AttributeValue::Bs(bs) => serializer.serialize_bytes(&bs.concat()),
            //AttributeValue::L(l) => serializer.serialize_seq(SeqWrapper::new(l)),
            //AttributeValue::M(m) => serializer.serialize_map(MapWrapper::new(m)),
            AttributeValue::Null(_) => serializer.serialize_none(),
            AttributeValue::Bool(b) => serializer.serialize_bool(*b),

            // If there are other variants you can handle them similarly
            _ => Err(serde::ser::Error::custom("Unsupported/unimplemented AttributeValue type in AttributeValueWrapper")),
        }
    }
}
