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
            ("FREE_MEMORY", &["-c", &format!("free -h")]),
            ("TOTAL_SPACE_AVAIL", &["-c", &format!("df -h")]),
        ];

        let mut result_output = HashMap::new();

        for (key, args) in commands {
            if let Ok(output) = self.get_command_output("sh", args) {
                result_output.insert(key, output);
            }
        }
        self.handle_output(Ok(result_output), channel);
    }

    pub fn cpu_memory(&self, channel: Sender<String>) {
        let commands = [
            (
                "MEMORY_USAGE",
                &[
                    "-c",
                    "ps -eo pid,ppid,cmd,%mem,%cpu --sort=-%mem | head -n 11",
                ],
            ),
            ("PROCESS_USAGE", &["-c", "top -n 1 -b | grep 'Cpu(s)'"]),
        ];

        let mut result_output = HashMap::new();

        for (key, args) in commands {
            if let Ok(output) = self.get_command_output("sh", args) {
                result_output.insert(key, output);
            }
        }
        self.handle_output(Ok(result_output), channel);
    }

    pub fn newtork(&self, channel: Sender<String>) {
        let commands = [
            ("PORT", &["-c", "nmap 127.0.0.1"]),
            ("ALL_CONNECTIONS", &["-c", "netstat -a"]),
            ("INTERFACE_ROUTING", &["-c", "netstat -ie"]),
        ];

        let mut result_output = HashMap::new();

        for (key, args) in commands {
            if let Ok(output) = self.get_command_output("sh", args) {
                result_output.insert(key, output);
            }
        }
        self.handle_output(Ok(result_output), channel);
    }

    pub fn log(&self, channel: Sender<String>) {
        let commands = [("LOG_ERROR", &["-c", "grep 'error' /var/log/syslog"])];

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
            "2" => self.log(channel),
            "3" => self.newtork(channel),
            "4" => self.cpu_memory(channel),
            _ => println!("It's something else"),
        }
    }
}
