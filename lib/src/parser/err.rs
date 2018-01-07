use std::io;
use std::error;
use std::fmt;

use parser::parser::yadql_grammar::ParseError;


#[derive(Debug)]
pub struct KeywordNotFound {
    pub v: String,
}

pub type KeywordNotFoundError = KeywordNotFound;

impl fmt::Display for KeywordNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl error::Error for KeywordNotFoundError {
    fn description(&self) -> &str {
        "The Keyword Could Not Be Identified"
    }
}

#[derive(Debug)]
pub enum ParserError {
    CouldNotOpenSourceCode(io::Error),
    CouldNotParseFile(ParseError),
    KeywordNotFound(KeywordNotFoundError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        match *self {
            ParserError::CouldNotOpenSourceCode(ref err) => {
                write!(f, "Could not open Source Code File! {}", err)
            }
            ParserError::CouldNotParseFile(ref err) => {
                write!(f, "Was not able to parse YADQL {}", err)
            }
            ParserError::KeywordNotFound(ref err) => {
                write!(f, "Could Not Find Keyword {}", err)
            }
        
        }
    }
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::CouldNotOpenSourceCode(ref err) => err.description(),
            ParserError::CouldNotParseFile(ref err) => err.description(),
            ParserError::KeywordNotFound(ref err) => err.description(),
        }
    }
   
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParserError::CouldNotOpenSourceCode(ref err) => Some(err),
            ParserError::CouldNotParseFile(ref err) => Some(err),
            ParserError::KeywordNotFound(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> ParserError {
        ParserError::CouldNotOpenSourceCode(err)
    }
}

impl From<ParseError> for ParserError {
    fn from(err: ParseError) -> ParserError {
        ParserError::CouldNotParseFile(err)
    }
}

impl From<KeywordNotFoundError> for ParserError {
    fn from(err: KeywordNotFoundError) -> ParserError {
        ParserError::KeywordNotFound(err)
    }
}


