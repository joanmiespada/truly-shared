use serde::Deserialize;

pub const SECRETS_MANAGER_APP_KEYS: &str = "truly_api_secrets";

#[derive(Deserialize, Debug)]
pub struct Secrets {
    #[serde(rename = "HMAC_SECRET")] //candidate to be removed from here and use the KMS_ID
    pub hmac_secret: String,
    #[serde(rename = "JWT_TOKEN_BASE")]
    pub jwt_token_base: String,
}
