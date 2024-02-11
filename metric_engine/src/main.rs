mod command_manager;
mod message_manager;
mod utils;
use command_manager::CommandManager;
use message_manager::MessageManager;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

const CURRENT_DIR: &str = "/home/tch";

fn main() {
    let (tx, rx) = mpsc::channel();

    let msg_manager = MessageManager::new();

    thread::spawn(move || loop {
        // Read messages from sent by command manager
        msg_manager.get_messages(&rx);
    });

    let cmd_manager = CommandManager::new(CURRENT_DIR.to_string());

    loop {
        cmd_manager.controller(tx.clone(), "1");
        cmd_manager.controller(tx.clone(), "2");
        cmd_manager.controller(tx.clone(), "4");
        sleep(Duration::from_secs(20));
    }
}
