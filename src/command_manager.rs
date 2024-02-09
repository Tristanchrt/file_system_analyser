use serde_json;
use std::collections::HashMap;
use std::{process::Command, sync::mpsc::Sender};

pub struct CommandManager {
    pub current_dir: String,
}

impl CommandManager {
    pub fn new(current_dir: String) -> Self {
        Self { current_dir }
    }

    fn handle_output(
        &self,
        output: Result<HashMap<&str, String>, std::io::Error>,
        channel: Sender<String>,
    ) {
        match output {
            Ok(data) => {
                let serialized_data = serde_json::to_string(&data).unwrap();
                channel.send(serialized_data).unwrap();
            }
            Err(err) => {
                eprintln!("Error running command:\n{}", err);
            }
        }
    }

    pub fn directories_stats(&self, channel: Sender<String>) {
        let commands = [
            (
                "CDS",
                &["-c", &format!("ls -l {} | head -n 10", self.current_dir)],
            ),
            (
                "NSD",
                &[
                    "-c",
                    &format!("find {} -mindepth 1 -type d | wc -l", self.current_dir),
                ],
            ),
            ("SD", &["-c", &format!("du -sh {}", self.current_dir)]),
            (
                "NF",
                &["-c", &format!("find {} -type f | wc -l", self.current_dir)],
            ),
        ];

        let mut result_output = HashMap::new();

        for (key, args) in commands {
            if let Ok(output) = self.get_command_output("sh", args) {
                result_output.insert(key, output);
            }
        }
        self.handle_output(Ok(result_output), channel);
    }

    fn get_command_output(&self, command: &str, args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(command).args(args).output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
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
