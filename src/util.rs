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

pub const ERR: &str = "\u{1b}[31m";
pub const SUCCESS: &str = "\u{1b}[32m";
pub const INFO: &str = "\u{1b}[33m";
pub const DEBUG: &str = "\u{1b}[35m";
pub const MSG: &str = "\u{1b}[36m";
pub const RESET: &str = "\u{1b}[0m";