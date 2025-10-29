use std::fmt::Display;

use crate::tui;
use crate::util;

use tui::LoadingBar;
use util::print_debug;
use util::ApolloError;
use util::{DEBUG, ERR, INFO, MSG, RESET, SUCCESS};
use util::{DOWN, LEFT, RIGHT, UP};

pub enum TokenType {
	NUMBER,
	HEXADECIMAL,
	OCTAL,
	BINARY,
	FLOAT,
	IDENTIFIER,
	STRING,
	CHARACTER,
	LESSEQL,
	GREATEREQL,
	LESS,
	GREATER,
	LEFTSHIFTASSIGN,
	RIGHTSHIFTASSIGN,
	LEFTSHIFT,
	RIGHTSHIFT,
	INCREMENT,
	ADDASSIGN,
	PLUS,
	DECREMENT,
	SUBASSIGN,
	MINUS,
	RIGHTARROW,
	EQL,
	LAMBDA,
	ASSIGN,
	LINECOMMENT,
	BLOCKCOMMENT,
	DIVASSIGN,
	DIVIDE,
	STARASSIGN,
	STAR,
	BANGASSIGN,
	BANG,
	CARROTASSIGN,
	CARROT,
	BITORASSIGN,
	BOOLOR,
	BAR,
	BITANDASSIGN,
	BOOLAND,
	AMPERSAND,
	ATSIGN,
	ANNOTATION,
	HASH,
	MODASSIGN,
	PERCENT,
	SQUIGGLE,
	QUESTION,
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
			TokenType::HEXADECIMAL => write!(f, "HEXADECIMAL"),
			TokenType::OCTAL => write!(f, "OCTAL"),
			TokenType::BINARY => write!(f, "BINARY"),
			TokenType::FLOAT => write!(f, "FLOAT"),
			TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
			TokenType::STRING => write!(f, "STRING"),
			TokenType::CHARACTER => write!(f, "CHARACTER"),
			TokenType::LESSEQL => write!(f, "LESS-EQUAL"),
			TokenType::GREATEREQL => write!(f, "GREATER-EQUAL"),
			TokenType::LESS => write!(f, "LESS"),
			TokenType::GREATER => write!(f, "GREATER"),
			TokenType::LEFTSHIFTASSIGN => write!(f, "LEFT-SHIFT-ASSIGN"),
			TokenType::RIGHTSHIFTASSIGN => write!(f, "RIGHT-SHIFT-ASSIGN"),
			TokenType::LEFTSHIFT => write!(f, "LEFT-SHIFT"),
			TokenType::RIGHTSHIFT => write!(f, "RIGHT-SHIFT"),
			TokenType::INCREMENT => write!(f, "INCREMENT"),
			TokenType::ADDASSIGN => write!(f, "ADD-ASSIGN"),
			TokenType::PLUS => write!(f, "PLUS"),
			TokenType::DECREMENT => write!(f, "DECREMENT"),
			TokenType::SUBASSIGN => write!(f, "SUB-ASSIGN"),
			TokenType::MINUS => write!(f, "MINUS"),
			TokenType::RIGHTARROW => write!(f, "RIGHT-ARROW"),
			TokenType::EQL => write!(f, "EQUALS"),
			TokenType::LAMBDA => write!(f, "LAMBDA"),
			TokenType::ASSIGN => write!(f, "ASSIGN"),
			TokenType::LINECOMMENT => write!(f, "LINE-COMMENT"),
			TokenType::BLOCKCOMMENT => write!(f, "BLOCK-COMMENT"),
			TokenType::DIVASSIGN => write!(f, "DIVIDE-ASSIGN"),
			TokenType::DIVIDE => write!(f, "DIVIDE"),
			TokenType::STARASSIGN => write!(f, "STAR-ASSIGN"),
			TokenType::STAR => write!(f, "STAR"),
			TokenType::BANGASSIGN => write!(f, "BANG-ASSIGN"),
			TokenType::BANG => write!(f, "BANG"),
			TokenType::CARROTASSIGN => write!(f, "CARROT-ASSIGN"),
			TokenType::CARROT => write!(f, "CARROT"),
			TokenType::BITORASSIGN => write!(f, "BITWISE-OR-ASSIGN"),
			TokenType::BOOLOR => write!(f, "BOOLEAN OR"),
			TokenType::BAR => write!(f, "BAR"),
			TokenType::BITANDASSIGN => write!(f, "BITWISE-AND-ASSIGN"),
			TokenType::BOOLAND => write!(f, "BOOLEAN AND"),
			TokenType::AMPERSAND => write!(f, "AMPERSAND"),
			TokenType::ATSIGN => write!(f, "ATSIGN"),
			TokenType::ANNOTATION => write!(f, "ANNOTATION"),
			TokenType::HASH => write!(f, "HASH"),
			TokenType::MODASSIGN => write!(f, "MODULO-ASSIGN"),
			TokenType::PERCENT => write!(f, "PERCENT"),
			TokenType::SQUIGGLE => write!(f, "SQUIGGLE"),
			TokenType::QUESTION => write!(f, "QUESTION"),
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
	pub metadata: Vec<Vec<char>>, // string interpolation and annotations
	pub line: usize,
	pub column: usize,
}

impl Display for LexerToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut additional_data = "".to_string();
		if !self.metadata.is_empty() {
			additional_data.push_str(&format!("\nMetadata: {:#?}", self.metadata));
		}
		write!(
			f,
			"{}: {} (Line: {}, Column: {}){}",
			self.token_type, self.value, self.line, self.column, additional_data
		)
	}
}

impl Default for LexerToken {
	fn default() -> Self {
		LexerToken {
			token_type: TokenType::ERROR,
			value: "".to_string(),
			metadata: Vec::new(),
			line: 0,
			column: 0,
		}
	}
}

pub struct Lexer {
	filepath: String,
	mode: u8,                // 0: quiet, 1: debug, 2: verbose
	logging: bool,           // Whether to log debug messages
	output_dir: String,      // Directory for output files and logs
	loading_bar: LoadingBar, // Loading bar for visual feedback

	//TODO: Additional fields for lexer state here
	content: String,            // The content of the file being lexed
	position: usize,            // Current position in the content
	read_position: usize,       // Position of the character being read
	current_char: Option<char>, // Current character being processed

	current_line: usize, // Current line number // increment when a newline is encountered
	current_column: usize, // Current column number // increment when a character is read
}

impl Lexer {
	pub fn new(filepath: String, mode: u8, logging: bool, output_dir: String) -> Self {
		let content = std::fs::read_to_string(&filepath).map_err(|_e| {
			ApolloError::new(
				format!("Failed to read file: {filepath}"),
				Some(0),
				None,
				None,
			)
		});
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

	pub fn is_whitespace(c: char) -> bool {
		c == ' ' || c == '\r' || c == '\t'
	}

	fn read_char(&mut self) {
		if self.read_position >= self.content.len() {
			if self.mode > 0 {
				print_debug(
					"Reached end of file: ",
					&self.filepath,
					self.logging,
					&self.output_dir,
				);
			}
			self.current_char = None; // End of file
		} else {
			self.current_char = Some(self.content.chars().nth(self.read_position).unwrap());
			if self.mode > 1 {
				print_debug(
					"Reading char: ",
					&self.current_char.unwrap().to_string(),
					self.logging,
					&self.output_dir,
				);
			}
		}

		self.position = self.read_position;
		self.read_position += 1;

		self.current_column += 1;
		if self.current_char == Some('\n') {
			self.current_line += 1; // Increment line number on newline
			self.current_column = 0; // Reset column number
		}

		if self.mode > 1 {
			print_debug(
				"Current position: ",
				&self.position.to_string(),
				self.logging,
				&self.output_dir,
			);
		}
	}

	fn peek_char(&self) -> Option<char> {
		if self.read_position >= self.content.len() {
			None // No more characters to read
		} else {
			if self.mode > 1 {
				print_debug(
					"Peeking char: ",
					&self
						.content
						.chars()
						.nth(self.read_position)
						.unwrap()
						.to_string(),
					self.logging,
					&self.output_dir,
				);
			}
			Some(self.content.chars().nth(self.read_position).unwrap())
		}
	}

	pub fn begin(&mut self) -> Result<Vec<LexerToken>, ApolloError> {
		if self.mode > 0 {
			print_debug(
				"Lexing file: ",
				&self.filepath,
				self.logging,
				&self.output_dir,
			);
		} // debug msg
		let mut tokens: Vec<LexerToken> = Vec::new();
		while self.current_char.is_some() {
			let tok = self.next_token();
			let tok = tok.unwrap_or_else(|| LexerToken {
				token_type: TokenType::ERROR,
				value: "".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			});
			if self.mode > 1 {
				print_debug(
					"Token generated: ",
					&tok.to_string(),
					self.logging,
					&self.output_dir,
				);
			}
			tokens.push(tok);
			self.read_char();
			if self.mode == 0 {
				self.loading_bar.lerp(
					((self.position as f32 / self.content.len() as f32) * 100.0).round() as i32,
					false,
				);
			}
		}
		Result::Ok(tokens)
	}

	pub fn next_token(&mut self) -> Option<LexerToken> {
		if self.mode > 1 {
			print_debug(
				"Generating next token...",
				"",
				self.logging,
				&self.output_dir,
			);
		}

		if self.current_char.is_none() {
			if self.mode > 0 {
				print_debug(
					"No more characters to read.",
					"",
					self.logging,
					&self.output_dir,
				);
			}
			return Some(LexerToken {
				token_type: TokenType::EOF,
				value: "".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			});
		}

		match self.current_char.unwrap() {
			'0'..='9' => {
				//TODO: handle hexadecimal, octal, binary, and float numbers -> 0x is hex, 0o is octal, 0b is binary, and float numbers can be handled with a decimal point and an f suffix
				if self.mode > 1 {
					print_debug(
						"Found digit, parsing number...",
						"",
						self.logging,
						&self.output_dir,
					);
				}
				self.parse_number()
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				if self.mode > 1 {
					print_debug(
						"Found identifier character, parsing identifier...",
						"",
						self.logging,
						&self.output_dir,
					);
				}
				self.parse_identifier()
			}
			'"' => {
				if self.mode > 1 {
					print_debug(
						"Found string delimiter, parsing string...",
						"",
						self.logging,
						&self.output_dir,
					);
				}
				self.parse_string() // need to handle escape characters in strings
			}
			'\'' => {
				if self.mode > 1 {
					print_debug(
						"Found character delimiter, parsing character...",
						"",
						self.logging,
						&self.output_dir,
					);
				}
				// need to handle escape characters in characters, like '\uXXXXX'
				self.parse_character()
			}
			'<' | '>' => {
				// handle <, <=, >, >=, >>, <<, >>=, <<=
				let c = self.current_char.unwrap();
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						// <= or >=
						self.read_char();
						if c == '<' {
							Some(LexerToken {
								token_type: TokenType::LESSEQL,
								value: "<=".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else if c == '>' {
							Some(LexerToken {
								token_type: TokenType::GREATEREQL,
								value: ">=".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else {
							Some(LexerToken {
								token_type: TokenType::ERROR,
								value: format!("{c}{}", next_char.unwrap()).to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} // This should not happen
					}
					Some('>') => {
						// >> or >>=
						self.read_char();
						if c == '>' {
							if self.peek_char() == Some('=') {
								self.read_char();
								return Some(LexerToken {
									token_type: TokenType::RIGHTSHIFTASSIGN,
									value: ">>=".to_string(),
									metadata: Vec::new(),
									line: self.current_line,
									column: self.current_column,
								});
							}
							Some(LexerToken {
								token_type: TokenType::RIGHTSHIFT,
								value: ">>".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else {
							Some(LexerToken {
								token_type: TokenType::ERROR,
								value: format!("{c}{}", next_char.unwrap()).to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} // This should not happen
					}
					Some('<') => {
						// << or <<=
						self.read_char();
						if c == '<' {
							if self.peek_char() == Some('=') {
								self.read_char();
								return Some(LexerToken {
									token_type: TokenType::LEFTSHIFTASSIGN,
									value: "<<=".to_string(),
									metadata: Vec::new(),
									line: self.current_line,
									column: self.current_column,
								});
							}
							Some(LexerToken {
								token_type: TokenType::LEFTSHIFT,
								value: "<<".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else {
							Some(LexerToken {
								token_type: TokenType::ERROR,
								value: format!("{c}{}", next_char.unwrap()).to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} // This should not happen
					}
					_ => {
						// < or >
						if c == '<' {
							Some(LexerToken {
								token_type: TokenType::LESS,
								value: "<".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else if c == '>' {
							Some(LexerToken {
								token_type: TokenType::GREATER,
								value: ">".to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						} else {
							Some(LexerToken {
								token_type: TokenType::ERROR,
								value: c.to_string(),
								metadata: Vec::new(),
								line: self.current_line,
								column: self.current_column,
							})
						}
					}
				}
			} // handle <, <=, >, >=, >>, <<, >>=, <<=
			'+' => {
				// handle a++, a + b, a += b
				let next_char = self.peek_char();
				match next_char {
					Some('+') => {
						// ++
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::INCREMENT,
							value: "++".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('=') => {
						// +=
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::ADDASSIGN,
							value: "+=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => {
						// +
						Some(LexerToken {
							token_type: TokenType::PLUS,
							value: "+".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
				}
			} // handle a++, a + b, a += b
			'-' => {
				// handle a--, a - b, -a, ->, a -= b
				let next_char = self.peek_char();
				match next_char {
					Some('-') => {
						// --
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::DECREMENT,
							value: "--".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('>') => {
						// ->
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::RIGHTARROW,
							value: "->".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('=') => {
						// -=
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::SUBASSIGN,
							value: "-=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => {
						// -
						Some(LexerToken {
							token_type: TokenType::MINUS,
							value: "-".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
				}
			} // handle a--, a - b, -a, ->, a -= b
			'=' => {
				// handle a = b, ==, =>
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						// ==
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::EQL,
							value: "==".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('>') => {
						// =>
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::LAMBDA,
							value: "=>".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::ASSIGN,
						value: "=".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle a = b, ==, =>
			'/' => {
				// handle //a, /* a */, a / b, a /= b
				let next_char = self.peek_char();
				match next_char {
					Some('/') => {
						// //
						self.read_char(); // skip second /
						self.read_char(); // make sure to skip that second /
						let mut content: String = "".to_string();
						while let Some(c) = self.current_char {
							if c == '\r' {
								// skip carriage return
								self.read_char();
								continue;
							}
							if c == '\n' {
								self.read_char();
								break;
							} else {
								print_debug(
									"Adding new character to comment content: ",
									&c.to_string(),
									self.logging,
									&self.output_dir,
								);
								content.push(c);
								self.read_char();
							}
						}
						Some(LexerToken {
							token_type: TokenType::LINECOMMENT,
							value: content,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('*') => {
						// /* */
						self.read_char();
						self.read_char();
						let mut content: String = "".to_string();
						while let Some(c) = self.current_char {
							if c == '*' && self.peek_char() == Some('/') {
								self.read_char();
								self.read_char();
								break;
							} else {
								self.read_char();
								content.push(c);
							}
						}
						Some(LexerToken {
							token_type: TokenType::BLOCKCOMMENT,
							value: content,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('=') => {
						// /=
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::DIVASSIGN,
							value: "/=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => {
						// /
						Some(LexerToken {
							token_type: TokenType::DIVIDE,
							value: "/".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
				}
			} // handle //a, /* a */, a / b, a /= b
			'*' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::STARASSIGN,
							value: "*=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::STAR,
						value: "*".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle a * b, a *= b, etc.
			'!' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::BANGASSIGN,
							value: "!=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::BANG,
						value: "!".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle !a, a != b, etc.
			'^' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::CARROTASSIGN,
							value: "^=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::CARROT,
						value: "^".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle a ^ b, a ^= b, etc.
			'|' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						// |=
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::BITORASSIGN,
							value: "|=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('|') => {
						// boolean or
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::BOOLOR,
							value: "||".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => {
						// bitwise/lambda params
						Some(LexerToken {
							token_type: TokenType::BAR,
							value: "|".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
				}
			} // handle a | b, a |= b, a || b, lambda parameters (| a: u32 |), etc.
			'&' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::BITANDASSIGN,
							value: "&=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					Some('&') => {
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::BOOLAND,
							value: "&&".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::AMPERSAND,
						value: "&".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle a & b, a &= b, a && b, etc.
			'@' => Some(LexerToken {
				token_type: TokenType::ATSIGN,
				value: "@".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // handle pass by reference (@a)
			'#' => {
				let next_char = self.peek_char();
				match next_char {
					Some('[') => {
						// annotations
						self.read_char(); // skip hash
						self.read_char(); // skip opening bracket
						let line: usize = self.current_line;
						let column: usize = self.current_column;
						let value: String = "#[{}]".to_string();
						let mut mdata: Vec<char> = Vec::new();
						while let Some(c) = self.current_char {
							if c == ']' {
								self.read_char(); // Skip the closing bracket
								break;
							} else {
								mdata.push(c); // add all the inner characters to the metadata field for processing later
								self.read_char();
							}
						}

						Some(LexerToken {
							token_type: TokenType::ANNOTATION,
							value,
							metadata: vec![mdata],
							line,
							column,
						})
					}
					_ => {
						// could be used for custom operators?
						Some(LexerToken {
							token_type: TokenType::HASH,
							value: '#'.to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
				}
			} // handle annotations like #[extern "..."], #[entry], etc.
			'%' => {
				let next_char = self.peek_char();
				match next_char {
					Some('=') => {
						// %=
						self.read_char();
						Some(LexerToken {
							token_type: TokenType::MODASSIGN,
							value: "/=".to_string(),
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column,
						})
					}
					_ => Some(LexerToken {
						token_type: TokenType::PERCENT,
						value: "%".to_string(),
						metadata: Vec::new(),
						line: self.current_line,
						column: self.current_column,
					}),
				}
			} // handle a % b, a %= b, etc.
			'~' => Some(LexerToken {
				token_type: TokenType::SQUIGGLE,
				value: "~".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // handle ~a
			'?' => Some(LexerToken {
				token_type: TokenType::QUESTION,
				value: "?".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // idk what to use this for, but handle it anyways
			';' => Some(LexerToken {
				token_type: TokenType::SEMICOLON,
				value: ";".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // end statements
			',' => Some(LexerToken {
				token_type: TokenType::COMMA,
				value: ",".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for separating items in lists, function arguments, etc.
			':' => Some(LexerToken {
				token_type: TokenType::COLON,
				value: ":".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for type declarataions (a: u32)
			'.' => Some(LexerToken {
				token_type: TokenType::DOT,
				value: ".".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for method calls (a.b()), field access (a.b), etc.
			'(' => Some(LexerToken {
				token_type: TokenType::LEFTPAREN,
				value: "(".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for function calls (a()), grouping expressions ((a + b)), etc.
			')' => Some(LexerToken {
				token_type: TokenType::RIGHTPAREN,
				value: ")".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for closing function calls, grouping expressions, etc.
			'{' => Some(LexerToken {
				token_type: TokenType::LEFTBRACE,
				value: "{".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for starting blocks of code (if, for, while, etc.) and string interpolation "{a + b}"
			'}' => Some(LexerToken {
				token_type: TokenType::RIGHTBRACE,
				value: "}".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for closing blocks of code and string interpolation
			'[' => Some(LexerToken {
				token_type: TokenType::LEFTBRACKET,
				value: "[".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for starting arrays and indexing
			']' => Some(LexerToken {
				token_type: TokenType::RIGHTBRACKET,
				value: "]".to_string(),
				metadata: Vec::new(),
				line: self.current_line,
				column: self.current_column,
			}), // used for closing arrays and indexing
			'\n' => {
				if self.mode > 1 {
					print_debug("Found newline...", "", self.logging, &self.output_dir);
				}
				Some(LexerToken {
					token_type: TokenType::NEWLINE,
					value: "\\n".to_string(),
					metadata: Vec::new(),
					line: self.current_line,
					column: self.current_column,
				})
			}
			' ' | '\t' | '\r' => {
				if self.mode > 1 {
					print_debug(
						"Found whitespace, skipping...",
						"",
						self.logging,
						&self.output_dir,
					);
				}
				self.read_char(); // Skip whitespace character
				self.next_token() // Continue to the next character
			}
			_ => {
				if self.mode > 1 {
					print_debug(
						"Found unknown character: ",
						&self.current_char.unwrap_or(' ').to_string(),
						self.logging,
						&self.output_dir,
					);
				}
				Some(LexerToken {
					token_type: TokenType::UNKNOWN,
					value: self.current_char.unwrap_or(' ').to_string(),
					metadata: Vec::new(),
					line: self.current_line,
					column: self.current_column,
				}) // Return error token for unknown characters
			}
		}
	}

	fn backtrack(&mut self) {
		if self.mode > 1 {
			print_debug("Backtracking...", "", self.logging, &self.output_dir);
		}
		if self.position == 0 {
			self.current_char = None;
		} else {
			self.current_char = self.content.chars().nth(self.position);
		}
		self.position -= 1;
		self.read_position -= 1;
		if self.current_column == 0 {
			self.current_line -= 1; // Decrement line number if at the start of a line
			self.current_column = self.content[..self.position]
				.chars()
				.rev()
				.take_while(|&c| c != '\n')
				.count(); // Count characters until the last newline
		} else {
			self.current_column -= 1; // Decrement column number
		}
	}

	fn is_hexadecimal_digit(c: char) -> bool {
		c >= 'A' || c <= 'F' || c >= 'a' || c <= 'f'
	}

	fn is_float_digit(c: char) -> bool {
		c == '.' || c == 'f'
	}
	fn is_octal_digit(c: char) -> bool {
		c >= '0' || c <= '7'
	}
	fn is_binary_digit(c: char) -> bool {
		c == '0' || c == '1'
	}

	fn parse_number(&mut self) -> Option<LexerToken> {
		if self.mode > 1 {
			print_debug("Parsing number...", "", self.logging, &self.output_dir);
		}
		let start_position = self.position;
		let mut value = String::new();

		while let Some(c) = self.current_char {
			if c.is_ascii_digit() || Self::is_hexadecimal_digit(c) || Self::is_float_digit(c) || c != '\n'
			{
				// if number, add to number buffer and go to next char
				value.push(c);
				self.read_char();
			} else {
				self.backtrack(); // move to previous character and exit loop
				break;
			}
		}

		//TODO: add support for float literals: X.Xf

		if value.starts_with('0') {
			if !value.contains('.') {
				match value.chars().nth(1) {
					Some('x') => {
						if self.mode > 1 {
							print_debug(
								"Found hexadecimal number",
								&value,
								self.logging,
								&self.output_dir,
							);
						}
						// check if all digits are within hexadecimal range then return hexadecimal number
						for c in value.chars() {
							if !c.is_ascii_digit() || !Self::is_hexadecimal_digit(c) {
								return Some(LexerToken {
									token_type: TokenType::ERROR,
									value,
									metadata: Vec::new(),
									line: self.current_line,
									column: self.current_column - (self.position - start_position),
								});
							}
						}
						// reach here when everything succeeds
						return Some(LexerToken {
							token_type: TokenType::HEXADECIMAL,
							value,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column - (self.position - start_position),
						});
					}
					Some('o') => {
						// check if all digits are under 8 then return octal number
						if self.mode > 1 {
							print_debug("Found octal number", &value, self.logging, &self.output_dir);
						}
						for c in value.chars() {
							if !Self::is_octal_digit(c) {
								return Some(LexerToken {
									token_type: TokenType::ERROR,
									value,
									metadata: Vec::new(),
									line: self.current_line,
									column: self.current_column - (self.position - start_position),
								});
							}
						}
						return Some(LexerToken {
							token_type: TokenType::OCTAL,
							value,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column - (self.position - start_position),
						});
					}
					Some('b') => {
						// check if all digits are 0 or 1 then return binary number
						if self.mode > 1 {
							print_debug(
								"Found binary number",
								&value,
								self.logging,
								&self.output_dir,
							);
						}
						for c in value.chars() {
							if !Self::is_binary_digit(c) {
								return Some(LexerToken {
									token_type: TokenType::ERROR,
									value,
									metadata: Vec::new(),
									line: self.current_line,
									column: self.current_column - (self.position - start_position),
								});
							}
						}
						return Some(LexerToken {
							token_type: TokenType::BINARY,
							value,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column - (self.position - start_position),
						});
					}
					_ => {
						// do nothing
					}
				}
			} else if value.ends_with('f') {
				if self.mode > 1 {
					print_debug("Found float", &value, self.logging, &self.output_dir);
				}
				let mut point_found = false;
				for c in value.chars() {
					if !c.is_ascii_digit() || !Self::is_float_digit(c) {
						return Some(LexerToken {
							token_type: TokenType::ERROR,
							value,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column - (self.position - start_position),
						});
					}
					if c == '.' && !point_found {
						point_found = true;
					} else if point_found {
						return Some(LexerToken {
							token_type: TokenType::ERROR,
							value,
							metadata: Vec::new(),
							line: self.current_line,
							column: self.current_column - (self.position - start_position),
						});
					}
				}
				return Some(LexerToken {
					token_type: TokenType::FLOAT,
					value,
					metadata: Vec::new(),
					line: self.current_line,
					column: self.current_column - (self.position - start_position),
				});
			}
		}

		if self.mode > 1 {
			print_debug("Parsed integer: ", &value, self.logging, &self.output_dir);
		}
		Some(LexerToken {
			token_type: TokenType::NUMBER,
			value,
			metadata: Vec::new(),
			line: self.current_line,
			column: self.current_column - (self.position - start_position),
		})
	}

	fn parse_identifier(&mut self) -> Option<LexerToken> {
		if self.mode > 1 {
			print_debug("Parsing identifier...", "", self.logging, &self.output_dir);
		}
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

		if self.mode > 1 {
			print_debug(
				"Parsed identifier: ",
				&value,
				self.logging,
				&self.output_dir,
			);
		}
		Some(LexerToken {
			token_type: TokenType::IDENTIFIER,
			value,
			metadata: Vec::new(),
			line: self.current_line,
			column: self.current_column - (self.position - start_position), // Adjust column based on the length of the identifier
		})
	}

	fn parse_string(&mut self) -> Option<LexerToken> {
		//TODO: need to update to be able to handle escaping \{, \}, and string interpolation
		if self.mode > 1 {
			print_debug("Parsing string...", "", self.logging, &self.output_dir);
		}
		let start_position = self.position;
		let mut value = String::new();
		self.read_char(); // Skip the opening quote
		let mut metadata = Vec::new();
		// split up string into multiple sections and rejoin at the { and }
		// so it will either break at " or {, and start again with }

		while let Some(c) = self.current_char {
			if c == '"' {
				self.read_char(); // Skip the closing quote
				break;
			} else if c == '{' {
				value.push(c); // push '{' to string before splitting
				self.read_char(); // skip '{'
				let mut inner_content = Vec::new();
				while let Some(inner_c) = self.current_char {
					if inner_c == '}' {
						value.push(inner_c); // push closing character
						self.read_char();
						break;
					}
					inner_content.push(inner_c); // change metadata from Vec<Vec<LexerToken>> to Vec<Vec<char>>
					self.read_char();
				}
				metadata.push(inner_content); // push the contents of the {} to metadata
			} else {
				value.push(c);
				self.read_char();
			}
		}

		if self.mode > 1 {
			print_debug("Parsed string: ", &value, self.logging, &self.output_dir);
		}
		Some(LexerToken {
			token_type: TokenType::STRING,
			value,
			metadata,
			line: self.current_line,
			column: self.current_column - (self.position - start_position), // Adjust column based on the length of the string
		})
	}

	fn parse_character(&mut self) -> Option<LexerToken> {
		if self.mode > 1 {
			print_debug("Parsing character...", "", self.logging, &self.output_dir);
		}
		let start_position = self.position;
		let mut value = String::new();
		self.read_char(); // Skip the opening quote

		let no_escape = self.current_char != Some('\\');

		while let Some(c) = self.current_char {
			if c == '\'' {
				self.read_char(); // Skip the closing quote
				break;
			} else {
				value.push(c);
				self.read_char();
			}
		}

		if value.len() != 1 && no_escape {
			eprintln!("{ERR}Error: {MSG}Invalid character literal: {INFO}'{}'{MSG}. Expected a single character.{ERR} Located @ Line: {INFO}{}{ERR}, Column: {INFO}{}{ERR}.{RESET}", value, self.current_line, self.current_column);
			std::process::exit(1);
		}

		if self.mode > 1 {
			print_debug("Parsed character: ", &value, self.logging, &self.output_dir);
		}
		Some(LexerToken {
			token_type: TokenType::CHARACTER,
			value,
			metadata: Vec::new(),
			line: self.current_line,
			column: self.current_column - (self.position - start_position), // Adjust column based on the length of the character
		})
	}
}
