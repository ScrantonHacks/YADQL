#[macro_use]
extern crate serde_derive;
extern crate colored;

mod core;
mod parser;

use std::process;
use std::error::Error;
use colored::Colorize;

use parser::parser::Parser;
use parser::parser::ParserError;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

mod yadql {
    use super::Parser;
    use super::Error;
    use super::Colorize;
    use super::process;

    fn parse(query: &str) -> Parser {
        match Parser::new(query) {
            Ok(x) => x,
            Err(e) => {
                println!("Error!: {}, {}", 
                        e.description().red().bold(), e.to_string()
                );
                process::exit(1);
            }
        }
    }

    fn execute(statement: &str) -> String {
        let parser: Parser = parse(statement);
        let ret: String;
        match cmd.OP {
            YADQL::Insert => Blockchain.insert(cmd.arg0, cmd.arg1),
            YADQL::Delete => Blockchain.delete(cmd.arg0),
            YADQL::Update => Blockchain.update(cmd.arg0, cmd.arg1),
            YADQL::Read => {
                ret = Blockchain.read(cmd.arg0);
            },
        }
        ret.to_string()
    }

    fn open() {
        return yadql {};
    }
}
