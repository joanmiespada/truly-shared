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
    pub static ref SECRETS_MANAGER_SMTP: String = format!("{}_{}_smtp2", VALUE_PROJECT, API_DOMAIN);
}

#[derive(Deserialize, Debug)]
pub struct ApiSecrets {
    #[serde(rename = "HMAC_SECRET")]
    pub hmac_secret: String,
    #[serde(rename = "JWT_TOKEN_BASE")]
    pub jwt_token_base: String,
    #[serde(rename = "PAGINATION_TOKEN")]
    pub pagination_token: String,
    #[serde(rename = "YOUTUBE_API_KEY")]
    pub youtube_api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct SMTPSecret {
    #[serde(rename = "username")]
    pub user: String,
    #[serde(rename = "password")]
    pub pass: String,
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
