use pulsar::{
    message::proto::command_subscribe::SubType, message::Payload, Consumer, DeserializeMessage,
    Pulsar, TokioExecutor,
};
use serde::{de, Deserialize};
use lazy_static::lazy_static;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};


// lazy_static! {
//     static ref LOCATION_RE: Regex = Regex::new(r"persistent://?P<tenant>/?P<namespace>/?P<topic>").unwrap();
// }

#[derive(Deserialize, Debug)]
pub struct UsageMessage {
    pub timestamp: u64,
    pub topic_uri: String,
    pub messages: u64,
    pub bytes: u64,
}

// impl<'de> Deserialize<'de> for UsageMessage {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         pub struct Temp {
//             timestamp: u64,
//             topic_uri: String,
//             messages: u64,
//             bytes: u64,
//         }
//         let temp = Temp::deserialize(deserializer)?;

//         let cap = LOCATION_RE.captures(temp.topic_uri).ok_or_else(|| de::Error::custom("unable to parse topic_uri"))?;
//         let tenant = cap.name("tenant").map_or_else(|| "".to_owned(), |s| s.to_str().to_owned());
//         let namespace = cap.name("namespace").map_or_else(|| "".to_owned(), |s| s.to_str().to_owned());
//         let topic = cap.name("topic").map_or_else(|| "".to_owned(), |s| s.to_str().to_owned());
//         let hour_timestamp = temp.timestamp % 3600;
//         let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(hour_timestamp, 0), Utc);
        
//         Ok(
//             Self {
//                 datetime,
//                 messages: temp.messages,
//                 bytes: temp.bytes,
//                 tenant,
//                 namespace,
//                 topic,
//             }
//         )
//     }
// }

impl DeserializeMessage for UsageMessage {
    type Output = Result<UsageMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}