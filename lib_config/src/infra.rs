use aws_config::meta::region::RegionProviderChain;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::{config::Credentials, primitives::Blob};
use aws_sdk_kms::types::KeyUsageType;
use base64::{engine::general_purpose, Engine as _};

use crate::{
    config::Config,
    constants::{TAG_PROJECT,VALUE_PROJECT, TAG_ENVIRONMENT,TAG_SERVICE, API_DOMAIN}, result::ResultE
};


pub async fn create_secret_manager_with_values(
    secrets_json: &str,
    secret_id: &str,
    config: &Config
) -> ResultE<()> {
    
    let client = aws_sdk_secretsmanager::client::Client::new(config.aws_config());

    let env = config.env_vars().environment().unwrap();

    client
        .create_secret()
        //.name(SECRETS_MANAGER_APP_KEYS.to_string())
        .name(secret_id.to_owned())
        .secret_string(secrets_json)
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key(TAG_PROJECT.to_owned())
                .value(VALUE_PROJECT.to_owned())
                .build(),
        )
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key(TAG_SERVICE.to_owned())
                .value(API_DOMAIN.to_owned())
                .build(),
        )
        .tags(
            aws_sdk_secretsmanager::types::Tag::builder()
                .key(TAG_ENVIRONMENT.to_owned())
                .value( env )
                .build(),
        )
        .send()
        .await?;

    Ok(())
}

// Only useful for testing, creation keys are manual or scripted.
pub async fn create_key(
    config: &Config
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    let client = aws_sdk_kms::client::Client::new(config.aws_config());
    let env = config.env_vars().environment().unwrap();
    let resp = client
        .create_key()
        .description("key used to encryp private key for contract owner")
        .key_usage(KeyUsageType::EncryptDecrypt)
        .tags(
            aws_sdk_kms::types::Tag::builder()
                .tag_key(TAG_PROJECT.to_owned())
                .tag_value(VALUE_PROJECT.to_owned())
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_kms::types::Tag::builder()
                .tag_key(TAG_SERVICE.to_owned())
                .tag_value(API_DOMAIN.to_owned())
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_kms::types::Tag::builder()
                .tag_key(TAG_ENVIRONMENT.to_owned())
                .tag_value( env )
                .build()
                .unwrap(),
        )
        .send()
        .await?;
    let id = resp.key_metadata.unwrap().key_id().to_string();

    Ok(id)
}

pub async fn build_local_stack_connection(host_port: u16) -> SdkConfig {
    let endpoint_url = format!("http://127.0.0.1:{}", host_port);
    //let uri = Uri::from_str(&endpoint_uri).unwrap();
    //let endpoint_resolver = Endpoint::immutable_uri(uri);
    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let creds = Credentials::new("test", "test", None, None, "test");

    let shared_config //= aws_config::from_env()
            = aws_config::defaults(aws_config::BehaviorVersion::v2023_11_09())
        .region(region_provider)
        .endpoint_url(endpoint_url)
        //.endpoint_resolver(endpoint_resolver.unwrap())
        .credentials_provider(creds)
        .load()
        .await;

    //Client::new(&shared_config)
    return shared_config;
}

pub async fn cypher_with_secret_key(
    info_to_be_encrypted: &str,
    kms_key_id: &str,
    config: &Config,
) -> ResultE<String> {
    let aws = config.aws_config();

    let client = aws_sdk_kms::Client::new(aws);

    let blob = Blob::new(info_to_be_encrypted.as_bytes());
    let resp_op = client
        .encrypt()
        .key_id(kms_key_id.clone())
        .plaintext(blob)
        .send()
        .await;
    let resp = resp_op.unwrap();

    let blob = resp.ciphertext_blob.expect("Could not get encrypted text");
    let bytes = blob.as_ref();

    let value = general_purpose::STANDARD.encode(bytes);

    Ok(value)
}

pub async fn uncypher_with_secret_key(
    info_to_be_decyphered: String,
    kms_key_id: &str,
    config: &Config,
) -> ResultE<String> {
    let aws = config.aws_config();

    let value = general_purpose::STANDARD
        .decode(info_to_be_decyphered)
        .unwrap();

    let client2 = aws_sdk_kms::Client::new(&aws);

    let data = aws_sdk_kms::primitives::Blob::new(value);

    let resp;
    let resp_op = client2
        .decrypt()
        .key_id(kms_key_id.to_owned())
        .ciphertext_blob(data.to_owned())
        .send()
        .await;
    match resp_op {
        Err(e) => {
            return Err(e.into());
        }
        Ok(val) => resp = val,
    }

    let inner = resp.plaintext.unwrap();
    let bytes = inner.as_ref();

    let secret_key_raw = String::from_utf8(bytes.to_vec()).unwrap(); // .expect("Could not convert to UTF-8");

    Ok(secret_key_raw)
}
