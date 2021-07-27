use redis::{Client};
use crate::usage_message::UsageMessage;


pub struct RedisStorage {
    client: Client,
}
impl RedisStorage {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self{
            client: redis::Client::open("redis://127.0.0.1/")?
        })
    }
    pub async fn save_message(&self, msg: UsageMessage) -> anyhow::Result<()> {
        let key = msg.topic_uri;
        let hour_timestamp = msg.timestamp / 3600000 * 3600000;
        let field_m = format!("{}_m", hour_timestamp);
        let field_b = format!("{}_b", hour_timestamp);
        let delta_m = msg.messages;
        let delta_b = msg.bytes;
        let mut con = self.client.get_async_connection().await?;
        redis::cmd("HINCRBY").arg(&[&key, &field_m, &delta_m.to_string()]).query_async(&mut con).await?;
        redis::cmd("HINCRBY").arg(&[&key, &field_b, &delta_b.to_string()]).query_async(&mut con).await?;
        Ok(())
    }
}