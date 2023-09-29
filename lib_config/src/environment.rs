use std::fmt::Display;

use serde::Deserialize;
use url::Url;

pub static ENV_VAR_ENVIRONMENT: &str = "ENVIRONMENT";
pub static ENV_VAR_PROJECT_LABEL: &str = "PROJECT";
pub static ENV_VAR_SERVICE_LABEL: &str = "SERVICE";
pub static ENV_VAR_PROJECT: &str = "truly";
pub static DEV_ENV: &str = "development";
pub static PROD_ENV: &str = "production";
pub static STAGE_ENV: &str = "stage";

#[derive(Deserialize, Clone, Debug)]
pub struct EnvironmentVariables {
    jwt_token_base: Option<String>,
    jwt_token_time_exp_hours: Option<String>,
    environment: Option<String>,
    hmac_secret: Option<String>,
    rust_log: Option<String>,
    aws_region: Option<String>,
    aws_profile: Option<String>,
    aws_endpoint: Option<String>,

    contract_id: Option<u16>,

    kms_key_id: Option<String>,
    queue_mint_async: Option<String>,
    topic_arn_mint_async: Option<String>,
    dead_letter_queue_mint: Option<String>,

    shorter_video_in_topic: Option<String>,
    shorter_video_out_topic: Option<String>,
    hashes_similar_video_in_topic: Option<String>,

    minting_fails_topic: Option<String>,

    bucket_video_temp: Option<String>,
    bucket_video_permanent: Option<String>,

    video_result_topic: Option<String>,

    telemetry: Option<bool>,
    telemetry_endpoint: Option<Url>,

    api_stage: Option<String>,
}

impl EnvironmentVariables {
    pub fn new() -> EnvironmentVariables {
        EnvironmentVariables {
            jwt_token_base: None,
            jwt_token_time_exp_hours: None,
            environment: None,
            hmac_secret: None,
            rust_log: None,
            aws_region: None,
            aws_profile: None, 
            aws_endpoint: None,
            contract_id: None,
            kms_key_id: None,
            queue_mint_async: None,
            topic_arn_mint_async: None,
            dead_letter_queue_mint: None,
            shorter_video_in_topic: None,
            shorter_video_out_topic: None,
            hashes_similar_video_in_topic: None,
            minting_fails_topic: None,
            bucket_video_temp: None,
            bucket_video_permanent: None,
            video_result_topic: None,
            telemetry: None,
            telemetry_endpoint: None,
            api_stage: None,
        }
    }
    pub fn rust_log(&self) -> Option<String> {
        self.rust_log.clone()
    }
    pub fn set_rust_log(&mut self, log: String) {
        self.rust_log = Some(log)
    }

    pub fn environment(&self) -> Option<String> {
        self.environment.clone()
    }
    pub fn set_environment(&mut self, env: String) {
        self.environment = Some(env);
    }

    pub fn hmac_secret(&self) -> Option<String> {
        self.hmac_secret.clone()
    }

    pub fn set_hmac_secret(&mut self, value: String) {
        self.hmac_secret = Some(value.clone());
    }

    pub fn jwt_token_base(&self) -> Option<String> {
        self.jwt_token_base.clone()
    }

    pub fn set_jwt_token_base(&mut self, value: String) {
        self.jwt_token_base = Some(value.clone());
    }

    pub fn aws_region(&self) -> Option<String> {
        self.aws_region.clone()
    }
    pub fn aws_profile(&self) -> Option<String> {
        self.aws_profile.clone()
    }

    pub fn aws_endpoint(&self) -> Option<String> {
        self.aws_endpoint.clone()
    }

    pub fn jwt_token_time_exp_hours(&self) -> Option<String> {
        self.jwt_token_time_exp_hours.clone()
    }

    pub fn contract_id(&self) -> Option<u16> {
        self.contract_id
    }
    pub fn set_contract_id(&mut self, val: u16) {
        self.contract_id = Some(val);
    }

    pub fn kms_key_id(&self) -> Option<String> {
        self.kms_key_id.clone()
    }
    pub fn set_kms_key_id(&mut self, value: String) {
        self.kms_key_id = Some(value.clone());
    }

    pub fn queue_mint_async(&self) -> Option<String> {
        self.queue_mint_async.clone()
    }
    pub fn set_queue_mint_async(&mut self, value: String) {
        self.queue_mint_async = Some(value.clone());
    }
    pub fn dead_letter_queue_mint(&self) -> Option<String> {
        self.dead_letter_queue_mint.clone()
    }
    pub fn set_dead_letter_queue_mint(&mut self, value: String) {
        self.dead_letter_queue_mint = Some(value.clone());
    }

    pub fn topic_arn_mint_async(&self) -> Option<String> {
        self.topic_arn_mint_async.clone()
    }
    pub fn set_topic_arn_mint_async(&mut self, value: String) {
        self.topic_arn_mint_async = Some(value.clone());
    }

    pub fn topic_arn_shorter_video_start(&self) -> Option<String> {
        self.shorter_video_in_topic.clone()
    }
    pub fn set_topic_arn_shorter_video_start(&mut self, value: String) {
        self.shorter_video_in_topic = Some(value.clone());
    }

    pub fn topic_arn_hashes_similars_start(&self) -> Option<String> {
        self.hashes_similar_video_in_topic.clone()
    }
    pub fn set_topic_arn_hashes_similars_start(&mut self, value: String) {
        self.hashes_similar_video_in_topic = Some(value.clone());
    }

    pub fn topic_arn_shorter_video_result(&self) -> Option<String> {
        self.shorter_video_out_topic.clone()
    }
    pub fn set_topic_arn_shorter_video_result(&mut self, value: String) {
        self.shorter_video_out_topic = Some(value.clone());
    }

    pub fn topic_arn_mint_fails(&self) -> Option<String> {
        self.minting_fails_topic.clone()
    }
    pub fn set_topic_arn_mint_fails(&mut self, value: String) {
        self.minting_fails_topic = Some(value.clone());
    }

    pub fn bucket_video_temp(&self) -> Option<String> {
        self.bucket_video_temp.clone()
    }

    pub fn set_bucket_video_temp(&mut self, value: String) {
        self.bucket_video_temp = Some(value.clone());
    }

    pub fn bucket_video_permanent(&self) -> Option<String> {
        self.bucket_video_permanent.clone()
    }

    pub fn set_bucket_video_permanent(&mut self, value: String) {
        self.bucket_video_permanent = Some(value.clone());
    }

    pub fn video_result_topic(&self) -> Option<String> {
        self.video_result_topic.clone()
    }

    pub fn set_video_result_topic(&mut self, value: String) {
        self.video_result_topic = Some(value.clone());
    }

    pub fn telemetry(&self) -> Option<bool> {
        self.telemetry.clone()
    }
    pub fn set_telemetry(&mut self, value: bool) {
        self.telemetry = Some(value);
    }

    pub fn telemetry_endpoint(&self) -> Option<Url> {
        self.telemetry_endpoint.clone()
    }
    pub fn set_telemetry_endpoint(&mut self, value: Url) {
        self.telemetry_endpoint = Some(value);
    }
    pub fn set_api_stage(&mut self, value: String) {
        self.api_stage = Some(value);
    }
    pub fn api_stage(&self)-> Option<String> {
        self.api_stage.clone()
    }
}

impl Display for EnvironmentVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ 'env': '{:?}', 'rust_log': '{:?}', 'aws': '{:?}', }}",
            self.environment,
            self.rust_log,
            self.aws_region 
            //TODO add other fields
        )
    }
}

impl Default for EnvironmentVariables {
    fn default() -> EnvironmentVariables {
        EnvironmentVariables::new()
    }
}

pub struct EnvironmentVariablesBuilder {
    environment: String,
    rust_log: String,
    aws_region: String,
}

impl EnvironmentVariablesBuilder {
    pub fn new(env: String, rust_log: String, aws_region: String) -> EnvironmentVariablesBuilder {
        EnvironmentVariablesBuilder {
            environment: env,
            rust_log,
            aws_region
        }
    }
    pub fn environment(&mut self, env: String) -> &mut EnvironmentVariablesBuilder {
        self.environment = env;
        self
    }
    pub fn rust_log(&mut self, rust_log: String) -> &mut EnvironmentVariablesBuilder {
        self.rust_log = rust_log;
        self
    }
    pub fn aws_region(&mut self, aws_region: String) -> &mut EnvironmentVariablesBuilder {
        self.aws_region= aws_region;
        self
    }

    pub fn build(&self) -> EnvironmentVariables {
        let mut aux = EnvironmentVariables::default();
        aux.set_environment(self.environment.clone());
        aux.set_rust_log(self.rust_log.clone());
        aux
    }
}
