use std::fmt::{Debug, Display};

use crate::lexer;
use crate::tui;
use crate::util;

use lexer::LexerToken;
use tui::LoadingBar;
use util::print_debug;
use util::ApolloError;
use util::{DEBUG, ERR, INFO, MSG, RESET, SUCCESS};
use util::{DOWN, LEFT, RIGHT, UP};

pub enum ParserTokenType {
	// primitives
}

pub struct ParserToken {
	pub token_type: ParserTokenType,
	pub value: Vec<LexerToken>,
	pub line: usize,
	pub column: usize,
}

pub struct Parser {
	filepath: String,
	debug_mode: u8,
	logging: bool,
	output_dir: String,
	loading_bar: LoadingBar,

	// additional private fields for parser state
	content: Vec<LexerToken>,
	position: usize,
	read_position: usize,
	current_token: Option<LexerToken>,

	// parser file position
	current_line: usize,
	current_column: usize,
}

impl Parser {
	pub fn new(
		filepath: String,
		lexer_tokens: Vec<LexerToken>,
		debug_mode: u8,
		logging: bool,
		output_dir: String,
	) -> Self {
		let mut p = Parser {
			filepath,
			debug_mode,
			logging,
			output_dir,
			loading_bar: LoadingBar::new(),
			content: lexer_tokens,
			position: 0,
			read_position: 0,
			current_token: None,
			current_line: 1,
			current_column: 0,
		};
		p.read_token();
		p
	}

	fn read_token(&mut self) {
		if self.read_position >= self.content.len() {
			if self.debug_mode > 0 {
				print_debug(
					"Reached end of token list: ",
					&self.filepath,
					self.logging,
					&self.output_dir,
				);
			}
			self.current_token = None; // end of file
		} else {
			self.current_token = Some(self.content[self.read_position].clone());
			if self.debug_mode > 1 {
				print_debug(
					"Reading token: ",
					&self.current_token.clone().unwrap().to_string(),
					self.logging,
					&self.output_dir,
				);
			}
		}
	}
}
