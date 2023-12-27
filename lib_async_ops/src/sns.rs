use aws_sdk_sns::types::Topic;
use lib_config::{constants::{TAG_PROJECT, VALUE_PROJECT, TAG_SERVICE, API_DOMAIN, TAG_ENVIRONMENT}, result::ResultE};
use tracing::log::info;

use crate::errors::AsyncOpError;


#[derive(Debug)]
pub struct SNSMessage {
    pub body: String,
}

async fn _check_if_exist(
    client: &aws_sdk_sns::client::Client,
    _config: &lib_config::config::Config,
    topic_arn: String,
) -> ResultE<String> {
    let topics_ops = client.list_topics().send().await;

    let topics = match topics_ops {
        Err(e) => return Err(AsyncOpError { 0: e.to_string() }.into()),
        Ok(rsp) => rsp,
    };

    let topics_arns = topics.topics();
    let my_topic = Topic::builder().topic_arn(topic_arn.to_owned()).build();

    let res = topics_arns.into_iter().filter(|x| **x == my_topic).count();

    match res {
        1 => Ok(topic_arn.to_owned()),
        _ => Err(AsyncOpError {
            0: "no queue found".to_string(),
        }
        .into()),
    }
}

#[tracing::instrument()]
pub async fn send(
    config: &lib_config::config::Config,
    message: &SNSMessage,
    topic_arn: String,
) -> ResultE<String> {
    let shared_config = config.aws_config();

    let client = aws_sdk_sns::client::Client::new(shared_config);

    info!("****************sending request {}", message.body);
    let rsp_op = client
        .publish()
        .topic_arn(topic_arn)
        .message(&message.body)
        .send()
        .await;
    match rsp_op {
        Err(e) => Err(AsyncOpError {
            0: e.into_service_error().meta().message().unwrap().to_string(),
        }
        .into()),
        Ok(rsp) => {
            let result = format!(
                "scheduled async op for id {:?} successfully.",
                rsp.message_id().unwrap()
            );
            Ok(result)
        }
    }
}

pub async fn create(config: &lib_config::config::Config, name: String) -> ResultE<String> {
    let shared_config = config.aws_config();

    let client = aws_sdk_sns::client::Client::new(shared_config);

    let env = config.env_vars().environment().unwrap();
    let res_op = client
        .create_topic()
        .name(name)
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key(TAG_PROJECT.to_owned())
                .value(VALUE_PROJECT.to_owned())
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key(TAG_SERVICE.to_owned())
                .value(API_DOMAIN.to_owned())
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_sns::types::Tag::builder()
                .key(TAG_ENVIRONMENT.to_owned())
                .value( env )
                .build()
                .unwrap(),
        )


        .send()
        .await;
    match res_op {
        Err(e) => panic!("{}",e ), //Err(AsyncOpError { 0: e.to_string() }.into()),
        Ok(v) => {
            let aux = v.topic_arn().unwrap_or_default().to_string();
            Ok(aux)
        }
    }
}
pub async fn delete(config: &lib_config::config::Config, topic_arn: String) -> ResultE<()> {
    let shared_config = config.aws_config();

    let client = aws_sdk_sns::client::Client::new(shared_config);

    let res_op = client.delete_topic().topic_arn(topic_arn).send().await;
    match res_op {
        Err(e) => Err(AsyncOpError { 0: e.to_string() }.into()),
        Ok(_) => Ok(()),
    }
}
