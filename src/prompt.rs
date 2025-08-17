use std::io::{self, Write};

pub fn prompt(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}