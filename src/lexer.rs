use std::fmt::Display;

use crate::util;
use crate::tui;

use util::ApolloError;
use util::{ ERR, SUCCESS, INFO, MSG, DEBUG, RESET };
use util::{ UP, DOWN, LEFT, RIGHT };
use tui::LoadingBar;
use util::print_debug;

pub enum TokenType {
    NUMBER,
    IDENTIFIER,
    STRING,
    CHARACTER,
    SEMICOLON,
    COLON,
    COMMA,
    DOT,
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    LEFTBRACKET,
    RIGHTBRACKET,
    NEWLINE,
    UNKNOWN,
    EOF,
    ERROR,
    // Add more token types as needed
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::CHARACTER => write!(f, "CHARACTER"),
            TokenType::SEMICOLON => write!(f, "SEMI-COLON"),
            TokenType::COLON => write!(f, "COLON"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::LEFTPAREN => write!(f, "LEFT-PAREN"),
            TokenType::RIGHTPAREN => write!(f, "RIGHT-PAREN"),
            TokenType::LEFTBRACE => write!(f, "LEFT-BRACE"),
            TokenType::RIGHTBRACE => write!(f, "RIGHT-BRACE"),
            TokenType::LEFTBRACKET => write!(f, "LEFT-BRACKET"),
            TokenType::RIGHTBRACKET => write!(f, "RIGHT-BRACKET"),
            TokenType::NEWLINE => write!(f, "NEWLINE"),
            TokenType::UNKNOWN => write!(f, "UNKNOWN"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::ERROR => write!(f, "ERROR"),
        }
    }
}

pub struct LexerToken {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Display for LexerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} (Line: {}, Column: {})", self.token_type, self.value, self.line, self.column)
    }
}

impl Default for LexerToken {
    fn default() -> Self {
        LexerToken {
            token_type: TokenType::ERROR,
            value: "".to_string(),
            line: 0,
            column: 0,
        }
    }
}

pub struct Lexer {
    filepath: String,
    mode: u8, // 0: quiet, 1: debug, 2: verbose
    logging: bool, // Whether to log debug messages
    output_dir: String, // Directory for output files and logs
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
    pub fn new(filepath: String, mode: u8, logging: bool, output_dir: String) -> Self {
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
            logging,
            output_dir,
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
            if self.mode > 0 { print_debug("Reached end of file: ", &self.filepath, self.logging, &self.output_dir); }
            self.current_char = None; // End of file
        } else {
            self.current_char = Some(self.content.chars().nth(self.read_position).unwrap());
            if self.mode > 1 { print_debug("Reading char: ", &self.current_char.unwrap().to_string(), self.logging, &self.output_dir); }
        }

        self.position = self.read_position;
        self.read_position += 1;

        self.current_column += 1;
        if self.current_char == Some('\n') {
            self.current_line += 1; // Increment line number on newline
            self.current_column = 0; // Reset column number
        }

        if self.mode > 1 { print_debug("Current position: ", &self.position.to_string(), self.logging, &self.output_dir); }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.content.len() {
            None // No more characters to read
        } else {
            if self.mode > 1 { print_debug("Peeking char: ", &self.content.chars().nth(self.read_position).unwrap().to_string(), self.logging, &self.output_dir); }
            Some(self.content.chars().nth(self.read_position).unwrap())
        }
    }

    pub fn begin(&mut self) -> Result<Vec<LexerToken>, ApolloError> {
        if self.mode > 0 { print_debug("Lexing file: ", &self.filepath, self.logging, &self.output_dir); } // debug msg
        let mut tokens: Vec<LexerToken> = Vec::new();
        while self.current_char.is_some() {
            let tok = self.next_token();
            let tok = tok.unwrap_or_else(|| {
                LexerToken {
                    token_type: TokenType::ERROR,
                    value: "".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                }
            });
            if self.mode > 1 { print_debug("Token generated: ", &tok.to_string(), self.logging, &self.output_dir); }
            tokens.push(tok);
            self.read_char();
            if self.mode == 0 {
                self.loading_bar.lerp(((self.position as f32 / self.content.len() as f32) * 100.0)
                                                .round() as i32, false);
            }
        }
        return Result::Ok(tokens); 
    }

    pub fn next_token(&mut self) -> Option<LexerToken> {
        if self.mode > 1 { print_debug("Generating next token...", "", self.logging, &self.output_dir); }

        if self.current_char.is_none() {
            if self.mode > 0 { print_debug("No more characters to read.", "", self.logging, &self.output_dir); }
            return Some(LexerToken {
                token_type: TokenType::EOF,
                value: "".to_string(),
                line: self.current_line,
                column: self.current_column,
            });
        }

        match self.current_char.unwrap() {
            '0'..='9' => { //TODO: handle hexadecimal, octal, binary, and float numbers -> 0x is hex, 0o is octal, 0b is binary, and float numbers can be handled with a decimal point and an f suffix
                if self.mode > 1 { print_debug("Found digit, parsing number...", "", self.logging, &self.output_dir); }
                return self.parse_number();
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                if self.mode > 1 { print_debug("Found identifier character, parsing identifier...", "", self.logging, &self.output_dir); }
                return self.parse_identifier();
            },
            '"' => {
                if self.mode > 1 { print_debug("Found string delimiter, parsing string...", "", self.logging, &self.output_dir); }
                return self.parse_string(); // need to handle escape characters in strings
            },
            '\'' => {
                if self.mode > 1 { print_debug("Found character delimiter, parsing character...", "", self.logging, &self.output_dir); }
                // need to handle escape characters in characters, like '\uXXXXX'
                return self.parse_character(); // This could be a separate method for character tokens
            },
            '<' | '>' => { return None; }, // handle <, <=, >, >=, >>, <<, >>=, <<= etc.
            '+' => { return None; }, // handle a++, a + b, a += b, etc.
            '-' => { return None; }, // handle a--, a - b, -a, ->, a -= b, etc.
            '=' => { return None; }, // handle a = b, ==, etc.
            '/' => { return None; }, // handle //a, /* a */, a / b, a /= b, etc.
            '*' => { return None; }, // handle a * b, a *= b, etc.
            '!' => { return None; }, // handle !a, a != b, etc.
            '^' => { return None; }, // handle a ^ b, a ^= b, etc.
            '|' => { return None; }, // handle a | b, a |= b, a || b, lambda parameters (| a: u32 |), etc.
            '&' => { return None; }, // handle a & b, a &= b, a && b, etc.
            '@' => { return None; }, // handle pass by reference (@a)
            '#' => { return None; }, // handle preprocessor directives like #[extern "..."], #[entry], etc.
            '%' => { return None; }, // handle a % b, a %= b, etc.
            '~' => { return None; }, // handle ~a
            '?' => { return None; }, // idk what to use this for, but handle it anyways
            ';' => { return Some(LexerToken {
                    token_type: TokenType::SEMICOLON, 
                    value: ";".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                }); 
            }, // end statements
            ',' => { return Some(LexerToken {
                    token_type: TokenType::COMMA, 
                    value: ",".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for separating items in lists, function arguments, etc.
            ':' => { return Some(LexerToken {
                    token_type: TokenType::COLON, 
                    value: ":".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for type declarataions (a: u32)
            '.' => { return Some(LexerToken {
                    token_type: TokenType::DOT, 
                    value: ".".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for method calls (a.b()), field access (a.b), etc.
            '(' => { return Some(LexerToken {
                    token_type: TokenType::LEFTPAREN, 
                    value: "(".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for function calls (a()), grouping expressions ((a + b)), etc.
            ')' => { return Some(LexerToken {
                    token_type: TokenType::RIGHTPAREN, 
                    value: "(".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for closing function calls, grouping expressions, etc.
            '{' => { return Some(LexerToken {
                    token_type: TokenType::LEFTBRACE, 
                    value: "{".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for starting blocks of code (if, for, while, etc.) and string interpolation "{a + b}"
            '}' => { return Some(LexerToken {
                    token_type: TokenType::RIGHTBRACE, 
                    value: "}".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for closing blocks of code and string interpolation
            '[' => { return Some(LexerToken {
                    token_type: TokenType::LEFTBRACKET, 
                    value: "[".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for starting arrays and indexing
            ']' => { return Some(LexerToken {
                    token_type: TokenType::RIGHTBRACKET, 
                    value: "]".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            }, // used for closing arrays and indexing
            '\n' => {
                if self.mode > 1 { print_debug("Found newline...", "", self.logging, &self.output_dir); }
                return Some(LexerToken {
                    token_type: TokenType::NEWLINE,
                    value: "\\n".to_string(),
                    line: self.current_line,
                    column: self.current_column,
                });
            },
            ' ' | '\t' | '\r' => {
                if self.mode > 1 { print_debug("Found whitespace, skipping...", "", self.logging, &self.output_dir); }
                self.read_char(); // Skip whitespace character
                return self.next_token(); // Continue to the next character
            },
            _ => {
                if self.mode > 1 { print_debug("Found unknown character: ", &self.current_char.unwrap_or(' ').to_string(), self.logging, &self.output_dir); }
                return Some(LexerToken {
                    token_type: TokenType::UNKNOWN,
                    value: self.current_char.unwrap_or(' ').to_string(),
                    line: self.current_line,
                    column: self.current_column,
                }); // Return error token for unknown characters
            }
        }
    }

    fn backtrack(&mut self) {
        if self.mode > 1 { print_debug("Backtracking...", "", self.logging, &self.output_dir); }
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

    fn parse_number(&mut self) -> Option<LexerToken> {
        if self.mode > 1 { print_debug("Parsing number...", "", self.logging, &self.output_dir); }
        let start_position = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                value.push(c);
                self.read_char();
            } else {
                self.backtrack();
                break;
            }
        }

        if self.mode > 1 { print_debug("Parsed integer: ", &value, self.logging, &self.output_dir); }
        Some(LexerToken { //TODO: Temporary, will need to handle different number types like float, hex, octal, etc.
            token_type: TokenType::NUMBER,
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the number
        })
    }
    
    //TODO: parse_hexadecimal, parse_octal, parse_binary, parse_float

    fn parse_identifier(&mut self) -> Option<LexerToken> { // there is a weird issue with this, "panic" keyword gets cut off, need to do some testing
        if self.mode > 1 { print_debug("Parsing identifier...", "", self.logging, &self.output_dir); }
        let start_position = self.position;
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.read_char();
            } else {
                self.backtrack();
                break;
            }
        }

        if self.mode > 1 { print_debug("Parsed identifier: ", &value, self.logging, &self.output_dir); }
        Some(LexerToken {
            token_type: TokenType::IDENTIFIER,
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the identifier
        })
    }

    fn parse_string(&mut self) -> Option<LexerToken> { //TODO: need to update to be able to handle escape characters like \n, \t, \", \{, \}, \uXXXX, and string interpolation
        if self.mode > 1 { print_debug("Parsing string...", "", self.logging, &self.output_dir); }
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

        if self.mode > 1 { print_debug("Parsed string: ", &value, self.logging, &self.output_dir); }
        Some(LexerToken {
            token_type: TokenType::STRING,
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the string
        })
    }

    fn parse_character(&mut self) -> Option<LexerToken> { //TODO: need to update to be able to handle escape characters like \n, \t, \', and unicode characters like \uXXXX
        if self.mode > 1 { print_debug("Parsing character...", "", self.logging, &self.output_dir); }
        let start_position = self.position;
        let mut value = String::new();
        self.read_char(); // Skip the opening quote

        while let Some(c) = self.current_char {
            if c == '\'' {
                self.read_char(); // Skip the closing quote
                break;
            } else {
                value.push(c);
                self.read_char();
            }
        }

        if value.len() != 1 {
            eprintln!("{ERR}Error: {MSG}Invalid character literal: {INFO}'{}'{MSG}. Expected a single character.{ERR} Located @ Line: {INFO}{}{ERR}, Column: {INFO}{}{ERR}.{RESET}", value, self.current_line, self.current_column);
            std::process::exit(1);
        }

        if self.mode > 1 { print_debug("Parsed character: ", &value, self.logging, &self.output_dir); }
        Some(LexerToken {
            token_type: TokenType::CHARACTER,
            value,
            line: self.current_line,
            column: self.current_column - (self.position - start_position), // Adjust column based on the length of the character
        })
    }
}
