use lib_async_ops::sqs::{create as sqs_create, send as sqs_send, SQSMessage};
use lib_async_ops::sns::{create as sns_create, send as sns_send, SNSMessage};
use lib_config::{config::Config, infra::build_local_stack_connection, environment::{DEV_ENV, ENV_VAR_ENVIRONMENT}};
use std::env;
use testcontainers::*;
use uuid::Uuid;

#[tokio::test]
async fn test_queues() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    env::set_var(ENV_VAR_ENVIRONMENT, DEV_ENV);
   
    //env::set_var(ENV_VAR_ENVIRONMENT, STAGE_ENV);
    //env::set_var("AWS_REGION", "eu-central-1");
    //env::set_var("AWS_PROFILE", "truly");



    env_logger::builder().is_test(true).init();

    let docker = clients::Cli::default();

    let mut local_stack = images::local_stack::LocalStack::default();
    local_stack.set_services("sqs");
    let node = docker.run(local_stack);
    let host_port = node.get_host_port_ipv4(4566);

    let shared_config = build_local_stack_connection(host_port).await;
    // set up config for truly app
    let mut config = Config::new();
    config.setup().await;
    config.set_aws_config(&shared_config); //rewrite configuration to use our current testcontainer instead

    let queue_url_op = sqs_create(&config, "deleteme".to_string()).await;
    assert_eq!(queue_url_op.is_ok(), true );
    let queue_url = queue_url_op.unwrap();
    env::set_var("QUEUE_MINT_ASYNC", queue_url.to_string());

    // config.setup().await;
    // config.set_aws_config(&shared_config); //rewrite configuration to use our current testcontainer instead

    let content = SQSMessage {
        id: Uuid::new_v4().to_string(),
        body: "body".to_string(),
    };

    let sent_op = sqs_send(&config, &content, queue_url.clone()).await;

    assert_eq!(sent_op.is_ok(),true);
    let sent = sent_op.unwrap();
    println!("{}", sent);
    //let response =
    //let recv_op = recieve(&config, queue_url.clone()).await;

    //assert_eq!(recv_op.is_ok(),true);
    //let recv = recv_op.unwrap();

    //assert_eq!(content.body, recv);

    Ok(())
}

#[tokio::test]
async fn test_topics() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    env::set_var(ENV_VAR_ENVIRONMENT, DEV_ENV);

    env_logger::builder().is_test(true).init();

    let docker = clients::Cli::default();

    let mut local_stack = images::local_stack::LocalStack::default();
    local_stack.set_services("sns");
    let node = docker.run(local_stack);
    let host_port = node.get_host_port_ipv4(4566);
    
    //let host_port = 4566;

    let shared_config = build_local_stack_connection(host_port).await;
    // set up config for truly app
    let mut config = Config::new();
    config.setup().await;
    config.set_aws_config(&shared_config); //rewrite configuration to use our current testcontainer instead

    let topic_arn_op = sns_create(&config, "deleteme".to_string()).await;
    assert_eq!(topic_arn_op.is_ok(),true);
    let topic_arn = topic_arn_op.unwrap();
    env::set_var("TOPIC_SNS_ASYNC", topic_arn.to_string());

    let content = SNSMessage {
        body: "body".to_string(),
    };

    let sent_op = sns_send(&config, &content, topic_arn.clone()).await;

    assert_eq!(sent_op.is_ok(),true);
    let sent = sent_op.unwrap();
    println!("{}", sent);

    Ok(())
}