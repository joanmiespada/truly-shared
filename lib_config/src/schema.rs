use async_trait::async_trait;
use aws_sdk_dynamodb::types::TableStatus;
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

pub async fn wait_until_schema_is_active(
    config: &Config,
    table_name: &str,
) -> ResultE<()> {
    let client = aws_sdk_dynamodb::Client::new(config.aws_config());
    loop {
        let resp = client
            .describe_table()
            .table_name(table_name)
            .send()
            .await?;

        match resp.table {
            Some(table) => match table.table_status {
                Some(status) if status == TableStatus::Active => break,
                _ => (),
            },
            None => (),
        };

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    Ok(())
}