use std::sync::mpsc::Receiver;

pub struct MessageManager {}

impl MessageManager {
    pub fn new() -> Self {
        Self {}
    }
    fn handle_message(&self, data: String) {
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
