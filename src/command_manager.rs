use std::{process::Command, sync::mpsc::Sender};

pub struct CommandManager {
    pub current_dir: String,
}

impl CommandManager {
    pub fn new(current_dir: String) -> Self {
        Self { current_dir }
    }

    pub fn directories_stats(&self, channel: Sender<String>) {
        let args = &["-c", "ls -l | head -n 10"];
        let output = self.exec("sh", args);
        match output {
            Ok(data) => {
                // println!("Command exec: <{}>",);
                channel
                    .send(String::from_utf8(data.stdout).unwrap())
                    .unwrap();
            }
            Err(err) => {
                eprintln!("Error running command:\n{}", err);
            }
        }
    }

    fn exec(&self, cmd: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        return Command::new(cmd)
            .args(args)
            .current_dir(&self.current_dir)
            .output();
    }

    pub fn controller(&self, channel: Sender<String>, job_id: &str) {
        match job_id {
            "1" => self.directories_stats(channel),
            "2" => println!("It's a banana!"),
            "3" => println!("It's an orange!"),
            _ => println!("It's something else"),
        }
    }
}
