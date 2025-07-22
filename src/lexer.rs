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
        let file_content = std::fs::read_to_string(&self.filepath)
            .map_err(|e| ApolloError::new(format!("Failed to read file: {}", self.filepath), Some(0), None, None))?;
        let mut tokens: Vec<LexerToken> = Vec::new();

        //TODO: implement the rest of the lexing logic
        // iterate over every character in the file, call identifyToken(), store result in token vector or panic if errors.
        // need to be able to handle comments, strings, numbers, identifiers, keywords, operators, and punctuation, aka skipping indices within the iteration
        

        return Result::Ok(tokens); 
    }

    pub fn generate_token(&self, c: char) -> Result<LexerToken, ApolloError> {
        // This function will identify the token type based on the character and return a LexerToken
        // For now, we will just return a dummy token
        let token = LexerToken {
            token_type: "IDENTIFIER".to_string(),
            value: c.to_string(),
            line: 1,
            column: 1,
        };
        Ok(token)
    }
}