use std::fmt::Display;

use crate::util;
use crate::tui;

use util::ApolloError;
use util::{ ERR, SUCCESS, INFO, MSG, DEBUG, RESET };
use util::{ UP, DOWN, LEFT, RIGHT };
use tui::LoadingBar;
use util::print_debug;


pub struct LexerToken {
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Display for LexerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} (Line: {}, Column: {})", self.token_type, self.value, self.line, self.column)
    }
}

pub struct Lexer {
    filepath: String,
    mode: u8, // 0: quiet, 1: debug, 2: verbose
    loading_bar: LoadingBar, // Loading bar for visual feedback

    //TODO: Additional fields for lexer state here
    content: String, // The content of the file being lexed
    position: usize, // Current position in the content
    read_position: usize, // Position of the character being read
    current_char: Option<char>, // Current character being processed

    current_line: usize, // Current line number // increment when a newline is encountered
    current_column: usize, // Current column number // increment when a character is read
}

impl Lexer {
    pub fn new(filepath: String, mode: u8) -> Self {
        let content = std::fs::read_to_string(&filepath)
            .map_err(|_e| ApolloError::new(format!("Failed to read file: {}", filepath), Some(0), None, None));
        let content = match content {
            Ok(c) => c,
            Err(e) => {
                e.print();
                std::process::exit(1);
            }
        };
        let mut l = Lexer {
            filepath,
            mode,
            loading_bar: LoadingBar::new(),
            content,
            position: 0,
            read_position: 0,
            current_char: None,
            current_line: 1,
            current_column: 0,
        };
        l.read_char(); // Initialize the first character
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.content.len() {
            if self.mode > 0 { print_debug("Reached end of file: ", &self.filepath); }
            self.current_char = None; // End of file
        } else {
            self.current_char = Some(self.content.chars().nth(self.read_position).unwrap());
            if self.mode > 1 { print_debug("Reading char: ", &self.current_char.unwrap().to_string()); }
        }

        self.position = self.read_position;
        self.read_position += 1;

        self.current_column += 1;
        if self.current_char == Some('\n') {
            self.current_line += 1; // Increment line number on newline
            self.current_column = 0; // Reset column number
        }

        if self.mode > 1 { print_debug("Current position: ", &self.position.to_string()); }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.content.len() {
            None // No more characters to read
        } else {
            if self.mode > 1 { print_debug("Peeking char: ", &self.content.chars().nth(self.read_position).unwrap().to_string()); }
            Some(self.content.chars().nth(self.read_position).unwrap())
        }
    }

    pub fn begin(&mut self) -> Result<Vec<LexerToken>, ApolloError> {
        if self.mode > 0 { print_debug("Lexing file: ", &self.filepath); } // debug msg
        let mut tokens: Vec<LexerToken> = Vec::new();
        while self.current_char.is_some() {
            let tok = self.next_token();
            if self.mode > 1 { print_debug("Token generated: ", &tok.to_string()); }
            tokens.push(tok);
            self.read_char();
            if self.mode == 0 {
                self.loading_bar.lerp(((self.position as f32 / self.content.len() as f32) * 100.0)
                                                .round() as i32, false);
            }
        }

        print!("{}", DOWN);
        return Result::Ok(tokens); 
    }

    pub fn next_token(&mut self) -> LexerToken {
        if self.mode > 1 { print_debug("Generating next token...", ""); }

        if self.current_char.is_none() {
            if self.mode > 0 { print_debug("No more characters to read.", ""); }
            return LexerToken {
                token_type: "EOF".to_string(),
                value: "".to_string(),
                line: self.current_line,
                column: self.current_column,
            };
        }

        match self.current_char.unwrap() {
            '0'..='9' => {
                if self.mode > 1 { print_debug("Found digit, parsing integer...", ""); }
                return self.parse_integer();
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                if self.mode > 1 { print_debug("Found identifier character, parsing identifier...", ""); }
                return self.parse_identifier();
            },
            '"' => {
                if self.mode > 1 { print_debug("Found string delimiter, parsing string...", ""); }
                return self.parse_string();
            },
            '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                if self.mode > 1 { print_debug("Found operator, returning as token...", ""); }
                let token = LexerToken {
                    token_type: self.current_char.unwrap().to_string(),
                    value: self.current_char.unwrap().to_string(),
                    line: self.current_line,
                    column: self.current_column,
                };
                self.read_char(); // Move to the next character
                return token;
            },
            '\n' => {
                if self.mode > 1 { print_debug("Found newline, skipping...", ""); }
                self.read_char(); // Skip newline
                return self.next_token(); // Continue to the next token
            },
            _ => {
                if self.mode > 0 { print_debug("Found unknown character, skipping...", ""); }
                self.read_char(); // Skip unknown character
            }
        }        

        //TODO: return a dummy token for now
        let token = LexerToken {
            token_type: "IDENTIFIER".to_string(),
            value: self.current_char.unwrap_or(' ').to_string(),
            line: self.current_line, // Placeholder for line number
            column: self.current_column, // Placeholder for column number
        };
        if self.mode > 1 { print_debug("Generated token: ", &token.to_string()); }
        return token;
    }

    fn backtrack(&mut self) {
        if self.mode > 1 { print_debug("Backtracking...", ""); }
        if self.position <= 0 {
            self.current_char = None;
        } else {
            self.current_char = self.content.chars().nth(self.position);
        }
        self.position -= 1;
        self.read_position -= 1;
        if self.current_column == 0 {
            self.current_line -= 1; // Decrement line number if at the start of a line
            self.current_column = self.content[..self.position].chars().rev().take_while(|&c| c != '\n').count(); // Count characters until the last newline
        } else {
            self.current_column -= 1; // Decrement column number
        }
    }

    fn parse_integer(&mut self) -> LexerToken {
        if self.mode > 1 { print_debug("Parsing number...", ""); }
        let start_position = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                value.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        if self.mode > 1 { print_debug("Parsed integer: ", &value); }
        LexerToken {
            token_type: "INT".to_string(),
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the number
        }
    }

    fn parse_identifier(&mut self) -> LexerToken {
        if self.mode > 1 { print_debug("Parsing identifier...", ""); }
        let start_position = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.read_char();
            } else {
                break;
            }
        }

        if self.mode > 1 { print_debug("Parsed identifier: ", &value); }
        LexerToken {
            token_type: "IDENT".to_string(),
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the identifier
        }
    }

    fn parse_string(&mut self) -> LexerToken {
        if self.mode > 1 { print_debug("Parsing string...", ""); }
        let start_position = self.position;
        let mut value = String::new();
        self.read_char(); // Skip the opening quote

        while let Some(c) = self.current_char {
            if c == '"' {
                self.read_char(); // Skip the closing quote
                break;
            } else {
                value.push(c);
                self.read_char();
            }
        }

        if self.mode > 1 { print_debug("Parsed string: ", &value); }
        LexerToken {
            token_type: "STR".to_string(),
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the string
        }
    }
}