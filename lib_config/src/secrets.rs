use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Error as SerdeJsonError;

use crate::{
    config::Config,
    constants::{API_DOMAIN, VALUE_PROJECT},
    result::ResultE,
};

lazy_static! {
    pub static ref SECRETS_MANAGER_APP_KEYS: String =
        format!("{}_{}_secrets", VALUE_PROJECT, API_DOMAIN);
}

#[derive(Deserialize, Debug)]
pub struct ApiSecrets {
    #[serde(rename = "HMAC_SECRET", default)]
    pub hmac_secret: String,
    #[serde(rename = "JWT_TOKEN_BASE", default)]
    pub jwt_token_base: String,
    #[serde(rename = "PAGINATION_TOKEN", default)]
    pub pagination_token: String,
    #[serde(rename = "YOUTUBE_API_KEY", default)]
    pub youtube_api_key: String,
    #[serde(rename = "TWITCH_CLIENT_ID", default)]
    pub twitch_client_id: String,
    #[serde(rename = "TWITCH_CLIENT_SECRET", default)]
    pub twitch_client_secret: String,
    #[serde(rename = "SMTP_USERNAME", default)]
    pub smtp_user: String,
    #[serde(rename = "SMTP_PASSWORD", default)]
    pub smtp_pass: String,
}

pub async fn get_secret<T: DeserializeOwned>(config: &Config, secret_id: &String) -> ResultE<T> {
    fn deserialize_json<T: DeserializeOwned>(text: &str) -> Result<T, SerdeJsonError> {
        serde_json::from_str(text)
    }

    let client = aws_sdk_secretsmanager::Client::new(config.aws_config());

    let resp = client
        .get_secret_value()
        .secret_id(secret_id.clone())
        .send()
        .await;

    match resp {
        Err(e) => {
            panic!(
                "secret: {}  couldn't find it. Error: {}",
                secret_id,
                e.to_string()
            )
        }
        Ok(scr) => {
            let value = scr.secret_string().unwrap();
            //let secrets: Secrets = serde_json::from_str(value).unwrap(); //_or( panic!("secrets malformed") );
            match deserialize_json::<T>(value) {
                Ok(s) => Ok(s),
                Err(e) => {
                    log::error!("error deserializing {}", e.to_string());
                    Err(Box::new(e))
                }
            }
        }
    }
}
