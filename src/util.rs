use std::io::Write;

pub struct ApolloError {
    message: String,
    index: Option<usize>,
    additional_info: Option<String>,
    additional_data: Option<usize>
}

impl ApolloError {
    pub fn new(message: String, index: Option<usize>, additional_info: Option<String>, additional_data: Option<usize>) -> Self {
        ApolloError {
            message,
            index,
            additional_info,
            additional_data
        }
    }

    pub fn to_string(&self) -> String {
        let mut error_message = format!("{}Error: {}{}", ERR, MSG, self.message);
        if let Some(index) = self.index {
            error_message.push_str(&format!("{} at index {}{}",ERR, INFO, index));
        }
        error_message.push_str(RESET);
        // additional data and info currently unused
        error_message
    }

    pub fn print(&self) {
        eprintln!("{}", self.to_string());
    }
}

#[inline(always)]
pub fn print_debug(message: &str, info: &str, log: bool, output_dir: &str) {
    println!("{DEBUG}{message}{INFO}{info}{RESET}");
    if log {
        let log_file_path = format!("{output_dir}/logs/debug.log");
        let mut log_file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file_path)
            .expect("Failed to open log file");
        writeln!(log_file, "{message}{info}").expect("{ERROR}Failed to write to log file{RESET}");
    }
}

pub const ERR: &str = "\u{1b}[31m";
pub const SUCCESS: &str = "\u{1b}[32m";
pub const INFO: &str = "\u{1b}[33m";
pub const DEBUG: &str = "\u{1b}[35m";
pub const MSG: &str = "\u{1b}[36m";
pub const RESET: &str = "\u{1b}[0m";

pub const UP: &str = "\u{1b}[1A";
pub const DOWN: &str = "\u{1b}[1B";
pub const LEFT: &str = "\u{1b}[1D";
pub const RIGHT: &str = "\u{1b}[1C";
pub const TOP: &str = "\u{1b}[H"; // Move cursor to the top of the terminal
pub const BOTTOM: &str = "\u{1b}[999B"; // Move cursor to the bottom of the terminal
pub const CLEAR: &str = "\u{1b}[2J"; // Clear the terminal screen