use std::io::{stdin, stdout, Write};

pub fn user_choice(text: &str) -> String {
    let mut s = String::new();
    print!("{} {}", text, ": ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}", s);
    return s.trim().to_string();
}
