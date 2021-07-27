use pulsar::{ message::Payload, DeserializeMessage};
use serde::{ Deserialize, Serialize};


#[derive(Deserialize,Serialize, Debug)]
pub struct UsageMessage {
    pub timestamp: u64,
    pub topic_uri: String,
    pub messages: u64,
    pub bytes: u64,
}

impl DeserializeMessage for UsageMessage {
    type Output = Result<UsageMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}