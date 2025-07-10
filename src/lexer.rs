use crate::util;

use util::ApolloError;
use util::{ ERR, SUCCESS, INFO, MSG, DEBUG, RESET };


pub struct LexerToken {
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    filepath: String,
    mode: u8, // 0: quiet, 1: debug, 2: verbose

    //TODO: Additional fields for lexer state here
}

impl Lexer {
    pub fn new(filepath: String, mode: u8) -> Self {
        Lexer {
            filepath,
            mode
        }
    }

    pub fn begin(&self) -> Result<Vec<LexerToken>, ApolloError> {
        if self.mode > 0 { println!("{}Lexing file: {}{}{}", DEBUG, INFO, self.filepath, RESET); } // debug msg
        // Open the file and read its contents
        let tokens: Vec<LexerToken> = Vec::new();

        //TODO: implement the rest of the lexing logic

        return Result::Ok(tokens); 
    }
}