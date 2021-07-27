mod usage_message;
mod redis_storage;
use futures::TryStreamExt;
use pulsar::{
    message::proto::command_subscribe::SubType, Consumer,
    Pulsar, TokioExecutor,
};
use std::time::Duration;
use crate::usage_message::UsageMessage;
use crate::redis_storage::RedisStorage;

#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    // Don't move on until storage connection is established
    let storage = loop {
        match RedisStorage::new().await {
            Ok(storage_engine) => break storage_engine,
            Err(e) => {
                log::error!("{}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    };

    let addr = "pulsar://127.0.0.1:6650";
    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await?;

    let mut consumer: Consumer<UsageMessage, _> = pulsar
        .consumer()
        .with_topic("persistent://public/default/usage-metrics")
        .with_options(pulsar::consumer::ConsumerOptions { initial_position: Some(1), ..Default::default() })
        .with_consumer_name("test_consumer")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("test_subscription")
        .build()
        .await?;

    while let Some(msg) = consumer.try_next().await? {
        consumer.ack(&msg).await?;
        match msg.deserialize() {
            Ok(msg) => {
                storage.save_message(msg).await.map_err(|e| pulsar::Error::Custom(e.to_string()))?;
            },
            Err(e) => {
                println!("could not deserialize message: {:?}", e);
                break;
            }
        };
    }
    Ok(())
}