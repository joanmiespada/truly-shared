use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_types::region::Region;
use dotenv::dotenv;
use log::{debug, info};
//use opentelemetry::sdk::export::trace;
use std::fmt::Display;

use crate::{
    environment::{EnvironmentVariables, DEV_ENV, ENV_VAR_ENVIRONMENT, PROD_ENV, STAGE_ENV},
    secrets::{Secrets, SECRETS_MANAGER_APP_KEYS},
};

#[derive(Clone, Debug)]
pub struct Config {
    aws_config: Option<SdkConfig>,
    env_variables: Option<EnvironmentVariables>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            aws_config: None,
            env_variables: None,
        }
    }

    pub fn refresh_env_vars(&mut self) {
        let check_env = std::env::var(ENV_VAR_ENVIRONMENT);
        match check_env {
            Err(e) => panic!(
                "Not environment variable found! {}. Error: {}",
                ENV_VAR_ENVIRONMENT,
                e.to_string()
            ),
            Ok(env) => {
                info!("environment var: {}", env);
                if env == DEV_ENV {
                    debug!("loading env vars from .env file");
                    dotenv().ok();
                } else if env == STAGE_ENV {
                    debug!("loading env vars from .env-stage file");
                    dotenv::from_filename(".env-stage").ok();
                }
            }
        }
        match envy::from_env::<EnvironmentVariables>() {
            Ok(env_vars) => {
                self.env_variables = Some(env_vars.clone());
            }
            Err(error) => panic!(
                "some mandatory environment variables are missing {:#?}",
                error
            ),
        }
    }

    pub async fn setup_with_secrets(&mut self) {
        self._setup_basic().await;
        self.load_secrets().await;
    }
    pub async fn setup(&mut self) {
        self._setup_basic().await;
    }
    async fn _setup_basic(&mut self) {
        self.refresh_env_vars();

        let env = self.env_variables.as_ref().unwrap();
        let config: SdkConfig;

        let env_flag = match env.environment() {
            None => panic!("error: environment variable not set up!"),
            Some(env_flag) => env_flag,
        };
        

        if env_flag == DEV_ENV {

            let aws_region_flag = match env.aws_region() {
                None => panic!("error: aws region variable not set up!"),
                Some(value) => value,
            };
            let aws_endpoint_flag = match env.aws_endpoint() {
                None => panic!("error: aws endpoint variable not set up!"),
                Some(value) => value,
            };

            let region_provider = RegionProviderChain::first_try(Region::new(aws_region_flag));
            let creden = aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name("localstack");
            config = aws_config::from_env()
                .credentials_provider(creden.build())
                .region(region_provider)
                .endpoint_url(aws_endpoint_flag)
                //.endpoint_resolver(endpoint_resolver.unwrap())
                .load()
                .await;
        } else if env_flag == PROD_ENV ||  env_flag == STAGE_ENV{
            
            let region_provider;
            if let Some(aws_region_flag) = env.aws_region() {
                region_provider = RegionProviderChain::first_try(Region::new(aws_region_flag));
            }else{
                region_provider = RegionProviderChain::default_provider();
            };

            config = aws_config::from_env().region(region_provider)
                    .load().await;
        }
        // else if env_flag == STAGE_ENV {
         //   let region_provider = RegionProviderChain::first_try(Region::new(aws_region_flag));
         //   config = aws_config::from_env().region(region_provider).load().await;
         //}
        else{
            panic!("environment flag has incorrect value")
        } 
        info!("region enabled: {}", config.region().unwrap()); 

        self.aws_config = Some(config);
    }

    pub fn aws_config(&self) -> &SdkConfig {
        let aux = self.aws_config.as_ref().unwrap();
        return aux;
    }
    pub fn set_aws_config(&mut self, cnf: &SdkConfig) {
        self.aws_config = Some(cnf.clone());
    }
    pub fn env_vars(&self) -> &EnvironmentVariables {
        let aux = self.env_variables.as_ref().unwrap();
        return aux;
    }
    pub fn set_env_vars(&mut self, new_data: &EnvironmentVariables) {
        self.env_variables = Some(new_data.clone())
    }

    pub async fn load_secret(&mut self, secret_id: &str) {
        let client = aws_sdk_secretsmanager::Client::new(self.aws_config());
        match secret_id {
            SECRETS_MANAGER_APP_KEYS => {
                let resp = client
                    .get_secret_value()
                    .secret_id(SECRETS_MANAGER_APP_KEYS)
                    .send()
                    .await;

                match resp {
                    Err(e) => {
                        panic!("secrets couldn't find: {}", e.to_string())
                    }
                    Ok(scr) => {
                        let value = scr.secret_string().unwrap();
                        let m_env = self.env_variables.as_mut().unwrap();
                        let secrets: Secrets = serde_json::from_str(value).unwrap(); //_or( panic!("secrets malformed") );
                        m_env.set_hmac_secret(secrets.hmac_secret);
                        m_env.set_jwt_token_base(secrets.jwt_token_base);
                        //m_env.set_blockchain_gateway_api_key(secrets.blockchain_gateway_api_key);

                        debug!("app secretes found correctly")
                    }
                }
            }
            _ => {
                panic!("secret code {} not found", secret_id)
            }
        }
    }

    pub async fn load_secrets(&mut self) {
        self.load_secret(SECRETS_MANAGER_APP_KEYS).await;
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ 'aws_config': '--', 'environment': '{}' }}",
            self.env_variables.clone().unwrap(),
        )
    }
}
