use std::io::{self, BufRead};

pub fn prompt_user() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().lock().read_line(&mut buffer);
    buffer
}
