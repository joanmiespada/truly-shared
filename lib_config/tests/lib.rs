use lib_config::{
    config::Config,
    infra::{
        build_local_stack_connection, create_key, create_secret_manager_with_values,
        uncypher_with_secret_key, cypher_with_secret_key,
    }, environment::{DEV_ENV, EnvironmentVariablesBuilder}, environment::ENV_VAR_ENVIRONMENT, stage::remove_stage_prefix, pagination::{pagination_encode_token, pagination_decode_token, PAGINATION_TOKEN_ENCODER}
};
use maplit::hashmap;
use std::env;
use testcontainers::*;

#[tokio::test]
async fn set_up_secret() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "debug");
    env::set_var(ENV_VAR_ENVIRONMENT, DEV_ENV);

    env_logger::builder().is_test(true).init();

    let docker = clients::Cli::default();

    let mut local_stack = images::local_stack::LocalStack::default();
    local_stack.set_services("secretsmanager,kms");
    let node = docker.run(local_stack);
    let host_port = node.get_host_port_ipv4(4566);

    let shared_config = build_local_stack_connection(host_port).await;

    let mut config = Config::new();
    config.setup().await;
    config.set_aws_config(&shared_config);

    //let keys_client = aws_sdk_kms::client::Client::new(&shared_config);
    let kms_id = create_key(&config).await?;
    //let secrets_client = aws_sdk_secretsmanager::client::Client::new(&shared_config);

    let secrets_json = r#"
    {
        "HMAC_SECRET" : "localtest_hmac_1234RGsdfg#$%",
        "JWT_TOKEN_BASE": "localtest_jwt_sd543ERGds235$%^",
        "PAGINATION_TOKEN": "localtest_pag_1234RGsdfg#$%"
    }
    "#;
    create_secret_manager_with_values(secrets_json, &config).await?;
    //create_secret_manager_secret_key(&secrets_client).await?;

    let secret: &str = "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"; // secret key example

    let cyphered = cypher_with_secret_key(secret, kms_id.as_str(), &config).await?;
    let res = uncypher_with_secret_key(cyphered, kms_id.as_str(), &config).await?;

    assert_eq!(secret, res);

    Ok(())
}

#[tokio::test]
async fn test_remove_api_prefix() {

    let pattern = "v1".to_string();

    let value= "/v1/v1/abc/cvf".to_string();
    let aux = remove_stage_prefix(value, pattern.clone());
    assert_eq!(aux,"/abc/cvf");

    let value= "/abc/cvf".to_string();
    let aux = remove_stage_prefix(value, pattern.clone());
    assert_eq!(aux,"/abc/cvf");

    let value= "/v1/abc/cvf".to_string();
    let aux = remove_stage_prefix(value, pattern.clone());
    assert_eq!(aux,"/abc/cvf");

    let value= "/v1/v1/v1/abc/cvf".to_string();
    let aux = remove_stage_prefix(value, pattern.clone());
    assert_eq!(aux,"/abc/cvf");


}

#[tokio::test]
async fn test_serialize_deserialize_pagination_token() {

    env::set_var("RUST_LOG", "debug");
    env::set_var(ENV_VAR_ENVIRONMENT, DEV_ENV);
    env::set_var(PAGINATION_TOKEN_ENCODER, "abcdfg");


    let aux = hashmap!{
        "name".to_string() => "pepe".to_string(), //Value::from_str("pepe").unwrap(),
        "surname".to_string() => "joseph".to_string() //Value::from_str("joseph").unwrap()
    };
    let aux_clone = aux.clone();

    let mut config = Config::new();
    config.setup().await;

    let res = pagination_encode_token::<String>(&config.env_vars(), Some(aux));

    let res2 = pagination_decode_token::<String>(&config.env_vars(), res).unwrap().unwrap();

    assert_eq!(res2["name"], aux_clone["name"]);
    assert_eq!(res2["surname"], aux_clone["surname"]);


}

#[tokio::test]
async fn test_env_vars() {

    let aux = EnvironmentVariablesBuilder::default()
        .rust_log(Some("RUST_LOG".to_string()))
        .build()
        .unwrap();


    assert_eq!(aux.rust_log().unwrap(), "RUST_LOG".to_string());

}

 