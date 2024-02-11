use std::collections::HashMap;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::{Deserialize, Serialize};

fn main() {
    let mut consumer = Consumer::from_hosts(vec!["kafka:9092".to_owned()])
        .with_topic_partitions("metric-data".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let deserialized_message: HashMap<&str, String> =
                    serde_json::from_slice(&m.value).unwrap();
                println!("RESULT FROM KAFKA {:?}", deserialized_message);
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}
