
use std::fmt::Display;

use derive_builder::Builder;
use serde::Deserialize;
use url::Url;

pub static ENV_VAR_ENVIRONMENT: &str = "ENVIRONMENT";
pub static DEV_ENV: &str = "development";
pub static PROD_ENV: &str = "production";
pub static STAGE_ENV: &str = "stage";

#[derive(Deserialize, Clone, Debug, Default, Builder)]
pub struct EnvironmentVariables {
    #[builder(default)]
    jwt_token_base: Option<String>,
    #[builder(default)]
    jwt_token_time_exp_hours: Option<String>,
    #[builder(default)]
    environment: Option<String>,
    #[builder(default)]
    hmac_secret: Option<String>,
    #[builder(default)]
    rust_log: Option<String>,
    #[builder(default)]
    aws_region: Option<String>,
    #[builder(default)]
    aws_profile: Option<String>,
    #[builder(default)]
    aws_endpoint: Option<String>,
    #[builder(default)]
    contract_id: Option<u16>,

    #[builder(default)]
    kms_key_id: Option<String>,
    #[builder(default)]
    queue_mint_async: Option<String>,
    #[builder(default)]
    topic_arn_mint_async: Option<String>,
    #[builder(default)]
    dead_letter_queue_mint: Option<String>,

    #[builder(default)]
    shorter_video_in_topic: Option<String>,
    #[builder(default)]
    shorter_video_out_topic: Option<String>,
    #[builder(default)]
    hashes_similar_video_in_topic: Option<String>,
    #[builder(default)]
    matchapi_endpoint: Option<Url>,

    #[builder(default)]
    minting_fails_topic: Option<String>,

    #[builder(default)]
    bucket_video_temp: Option<String>,
    #[builder(default)]
    bucket_video_permanent: Option<String>,

    #[builder(default)]
    video_result_topic: Option<String>,

    #[builder(default)]
    telemetry: Option<bool>,
    #[builder(default)]
    telemetry_endpoint: Option<Url>,

    #[builder(default)]
    api_stage: Option<String>,
    #[builder(default)]
    trace_level: Option<String>,
    
    #[builder(default)]
    url_base_permanent_images: Option<String>,

    #[builder(default)]
    smtp_host: Option<String>,
    #[builder(default)]
    smtp_user: Option<String>,
    #[builder(default)]
    smtp_passw: Option<String>,
    #[builder(default)]
    smtp_from_email: Option<String>,
    
    #[builder(default)]
    pagination_token_encoder: Option<String>,
    #[builder(default="Some(25)")]
    default_page_size: Option<u32>,

    #[builder(default)]
    youtube_api_key: Option<String>,

    #[builder(default)]
    twitch_client_id: Option<String>,
    #[builder(default)]
    twitch_client_secret: Option<String>,
}

impl EnvironmentVariables {
    pub fn new() -> EnvironmentVariables {
        EnvironmentVariablesBuilder::default().build().unwrap()
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

    pub fn matchapi_endpoint(&self) -> Option<Url>{
        self.matchapi_endpoint.clone()
    }
    pub fn set_matchapi_endpoint(&mut self, value:Url) {
        self.matchapi_endpoint = Some(value)
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

    pub fn trace_level(&self) -> Option<String> {
        self.trace_level.clone()
    }
    pub fn set_trace_level(&mut self, value: String) {
        self.trace_level = Some(value);
    }

    pub fn url_base_permanent_images(&self) -> Option<String> {
        self.url_base_permanent_images.clone()
    }
    pub fn set_url_base_permanent_images(&mut self, value: String) {
        self.url_base_permanent_images = Some(value);
    }

    pub fn smtp_host(&self) -> Option<String> {
        self.smtp_host.clone()
    }

    pub fn set_smtp_host(&mut self, value: String) {
        self.smtp_host = Some(value);
    }

    pub fn smtp_user(&self) -> Option<String> {
        self.smtp_user.clone()
    }

    pub fn set_smtp_user(&mut self, value: String) {
        self.smtp_user = Some(value);
    }

    pub fn smtp_passw(&self) -> Option<String> {
        self.smtp_passw.clone()
    }

    pub fn set_smtp_passw(&mut self, value: String) {
        self.smtp_passw = Some(value);
    }

    pub fn pagination_token_encoder(&self) -> Option<String> {
        self.pagination_token_encoder.clone()
    }

    pub fn set_pagination_token_encoder(&mut self, value: String) {
        self.pagination_token_encoder = Some(value);
    }

    pub fn default_page_size(&self) -> Option<u32> {
        self.default_page_size
    }

    pub fn set_default_page_size(&mut self, value: u32) {
        self.default_page_size = Some(value);
    }

    pub fn smtp_from_email(&self) -> Option<String> {
        self.smtp_from_email.clone()
    }

    pub fn set_smtp_from_email(&mut self, value: String) {
        self.smtp_from_email = Some(value);
    }
    
    pub fn youtube_api_key(&self) -> Option<String> {
        self.youtube_api_key.clone()
    }

    pub fn set_youtube_api_key(&mut self, value: String) {
        self.youtube_api_key = Some(value);
    }
    pub fn twitch_client_id(&self) -> Option<String> {
        self.twitch_client_id.clone()
    }
    pub fn set_twitch_client_id(&mut self, value: String) {
        self.twitch_client_id = Some(value);
    }
    pub fn twitch_client_secret(&self) -> Option<String> {
        self.twitch_client_secret.clone()
    }
    pub fn set_twitch_client_secret(&mut self, value: String) {
        self.twitch_client_secret = Some(value);
    }

}

impl Display for EnvironmentVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ 
                'jwt_token_base': '{:?}', 
                'jwt_token_time_exp_hours': '{:?}', 
                'environment': '{:?}', 
                'hmac_secret': '{:?}', 
                'rust_log': '{:?}', 
                'aws_region': '{:?}', 
                'aws_profile': '{:?}', 
                'aws_endpoint': '{:?}', 
                'contract_id': '{:?}', 
                'kms_key_id': '{:?}', 
                'queue_mint_async': '{:?}', 
                'topic_arn_mint_async': '{:?}', 
                'dead_letter_queue_mint': '{:?}', 
                'shorter_video_in_topic': '{:?}', 
                'shorter_video_out_topic': '{:?}', 
                'hashes_similar_video_in_topic': '{:?}', 
                'matchapi_endpoint': '{:?}', 
                'minting_fails_topic': '{:?}', 
                'bucket_video_temp': '{:?}', 
                'bucket_video_permanent': '{:?}', 
                'video_result_topic': '{:?}', 
                'telemetry': '{:?}', 
                'telemetry_endpoint': '{:?}', 
                'api_stage': '{:?}', 
                'trace_level': '{:?}', 
                'url_base_permanent_images': '{:?}', 
                'smtp_host': '{:?}', 
                'smtp_user': '{:?}', 
                'smtp_passw': '****', 
                'smtp_from_email': '{:?}',
                'pagination_token_encoder': '{:?}', 
                'default_page_size': '{:?}',
                'youtube_api_key': '***' 
                'twitch_client_id': '{:?}',
                'twitch_client_secret': '***'
            }}",
            self.jwt_token_base,
            self.jwt_token_time_exp_hours,
            self.environment,
            self.hmac_secret,
            self.rust_log,
            self.aws_region,
            self.aws_profile,
            self.aws_endpoint,
            self.contract_id,
            self.kms_key_id,
            self.queue_mint_async,
            self.topic_arn_mint_async,
            self.dead_letter_queue_mint,
            self.shorter_video_in_topic,
            self.shorter_video_out_topic,
            self.hashes_similar_video_in_topic,
            self.matchapi_endpoint,
            self.minting_fails_topic,
            self.bucket_video_temp,
            self.bucket_video_permanent,
            self.video_result_topic,
            self.telemetry,
            self.telemetry_endpoint,
            self.api_stage,
            self.trace_level,
            self.url_base_permanent_images,
            self.smtp_host,
            self.smtp_user,
            //self.smtp_passw,
            self.smtp_from_email,
            self.pagination_token_encoder,
            self.default_page_size,
            //self.youtube_api_key,
            self.twitch_client_id,
            //self.twitch_client_secret,
        )
    }
}



