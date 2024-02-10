use std::{sync::mpsc::Receiver, time::Duration};

use kafka::producer::{Producer, Record, RequiredAcks};

pub struct MessageManager {}

impl MessageManager {
    pub fn new() -> Self {
        Self {}
    }
    fn handle_message(&self, data: String) {
        let mut producer = Producer::from_hosts(vec!["kafka:9092".to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        producer
            .send(&Record::from_value("my-topic", data.as_bytes()))
            .unwrap();

        println!("******* Result: ********* \n{}", data);
    }
    pub fn get_messages(&self, rx: &Receiver<String>) {
        match rx.recv() {
            Ok(msg) => self.handle_message(msg),
            Err(_) => {
                println!("Error reading message");
            }
        }
    }
}
