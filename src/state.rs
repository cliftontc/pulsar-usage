use std::collections::HashMap;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use crate::usage_message::UsageMessage;
#[derive(Debug)]
pub struct State {
    reports: HashMap<(String, DateTime<Utc>), Report>,
}
impl State {
    pub fn save_message(&mut self, usage_message: &UsageMessage) {
        let hour_timestamp = usage_message.timestamp % 3600;
        let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(hour_timestamp as i64, 0), Utc);
        let key = (usage_message.topic_uri.clone(), datetime);
        let report = self.reports.entry(key).or_insert(Report::default());
        report.messages += usage_message.messages;
        report.bytes += usage_message.bytes;
    }
    // pub fn generate_reports(&self) -> (
    //     HashMap<String, Reports>,
    //     HashMap<String, Reports>,
    //     HashMap<String, Reports>) {
    //         let topic_reports = self.
    //     }
}

#[derive(Default, Debug)]
struct Report {
    messages: u64,
    bytes: u64
}
