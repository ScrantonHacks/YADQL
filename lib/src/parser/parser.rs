use std::io::prelude::*;
use std::fmt;

use self::err::ParserError;
use core::keywords::YADQL;
use parser::*;


#[allow(dead_code)]
pub mod yadql_grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[derive(Debug)]
pub struct Parser {
    /// Where the translated DB Access String goes once they are
    /// identified by the PEG Grammar
    pub keywords: Vec<YADQL>,
}

/// Runs grammar on source file.
impl Parser {
    pub fn new(query: &str) -> Result<Parser, ParserError> {
        let source = yadql_grammar::content(query)?;
        Ok(Parser{keywords: source})
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.keywords)
    }
}

#[cfg(test)]
#[test]
fn test_parser() {
    let parser = match Parser::new("INSERT $RAND '{'yo': 42}'") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    println!("Parser: {}", parser.to_string());
}
