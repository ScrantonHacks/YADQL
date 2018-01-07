/**
 * 
 */

use std::io::prelude::*;
use std::fs::File;
use std::fmt;

use self::err::ParserError;
use core::keywords::YADQL;
use parser::*;


#[allow(dead_code)]
pub mod yadql_grammer {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}


#[derive(Debug)]
pub struct Parser {
    /// Where the translated DB Access String goes once they are
    /// identified by the PEG Grammar
    pub types: Vec<YADQL>,
}

/// Runs grammar on source file.
impl Parser {
    pub fn new(source_file: &str) -> Result<Parser, ParserError> {
        let mut f = File::open(source_file)?;
        let mut source = String::new();
        f.read_to_string(&mut source)?;
        let source = yadql_grammar::content(source.as_ref())?;
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
    let parser = match Parser::new("src/parser/YADQL.test") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    println!("Parser: {}", parser.to_string());
}
