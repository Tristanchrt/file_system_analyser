mod command_manager;
mod message_manager;
mod utils;
use command_manager::CommandManager;
use message_manager::MessageManager;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;
use utils::user_choice;

const CURRENT_DIR: &str = "/home/tch";

fn main() {
    let (tx, rx) = mpsc::channel();

    let msg_manager = MessageManager::new();

    thread::spawn(move || loop {
        // Read messages from sent by command manager
        msg_manager.get_messages(&rx);
    });

    loop {
        println!("Welcom to the file system analyser !");
        println!("What do you want to do ?");
        println!("1 - Directory Statistics");
        println!("2 - Log Error");
        println!("3 - Network Statisctics");
        println!("4 - CPU & MEMORY Stastistics");

        let res = user_choice("Please enter you job");

        // controller(&res);
        let cmd_manager = CommandManager::new(CURRENT_DIR.to_string());
        cmd_manager.controller(tx.clone(), &res);
        sleep(Duration::from_secs(1));
    }
}
