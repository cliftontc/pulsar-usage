mod usage_message;
mod state;
use serde::{Serialize, Deserialize};
use futures::TryStreamExt;
use pulsar::{
    message::proto::command_subscribe::SubType, message::Payload, Consumer, DeserializeMessage,
    Pulsar, TokioExecutor,
};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use crate::state::State;
use crate::usage_message::UsageMessage;
// #[derive(Serialize, Deserialize, Debug)]
// struct TestData {
//     timestamp: u64,
// }

// impl DeserializeMessage for TestData {
//     type Output = Result<TestData, serde_json::Error>;

//     fn deserialize_message(payload: &Payload) -> Self::Output {
//         // println!("{:?}", std::str::from_utf8(&payload.data));
//         serde_json::from_slice(&payload.data)
//     }
// }
// 1617395244367
#[tokio::main]
async fn main() -> Result<(), pulsar::Error> {
    let v = Arc::new(Mutex::new(Vec::new()));
    let vc = Arc::clone(&v);
    tokio::spawn(
        worker_thread(vc)
    );

    loop {
        thread::sleep(Duration::from_secs(10));
        let w = v.lock().unwrap();

        println!("{:?}", w.len());
        drop(w);

    }

    Ok(())
}
async fn worker_thread(v: Arc<Mutex<Vec<UsageMessage>>>) {
    let addr = "pulsar://127.0.0.1:6650";
    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await.unwrap();

    let mut consumer: Consumer<UsageMessage, _> = pulsar
        .consumer()
        .with_topic("persistent://public/default/usage-metrics")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("my-first-subscription")
        .build()
        .await.unwrap();
    // let mut counter = 0usize;
    loop{
    while let Some(msg) = consumer.try_next().await.unwrap() {
        consumer.ack(&msg).await.unwrap();
        match msg.deserialize() {
            Ok(data) => {
                // let mut w = v.write().expect("RwLock poisoned");
                let mut w = v.lock().unwrap();
                w.push(data);

                drop(w);
            },
            Err(e) => {
                println!("could not deserialize message: {:?}", e);
                break;
            }
        };
        // thread::sleep(Duration::from_secs(2));
        // v.lock().unwrap().push(counter);
        // counter += 1
    }
    }
}