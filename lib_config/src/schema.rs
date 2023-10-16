use async_trait::async_trait;
use crate::config::Config;
use crate::result::ResultE;

#[async_trait]
pub trait Schema {
    async fn create_schema(config: &Config) -> ResultE<()>;
    async fn delete_schema(config: &Config) -> ResultE<()>;
}

pub async fn schema_exists(config: &Config, table: &str) -> ResultE<bool> {

    let client = aws_sdk_dynamodb::Client::new(config.aws_config());
    let table_list = client.list_tables().send().await;

    match table_list {
        Ok(list) => Ok(list.table_names().unwrap().contains(&table.into())),
        Err(e) => Err(e.into()),
    }
}