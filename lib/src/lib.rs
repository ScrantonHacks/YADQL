#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate gpgme;
extern crate regex;
extern crate web3;
extern crate ethabi;

mod core;
mod parser;
mod blockchain;
mod crypt;

use std::process;
use std::error::Error;
use colored::Colorize;

use parser::parser::Parser;
use core::keywords::YADQL;
use blockchain::blockchain::Blockchain;
use blockchain::err::BlockchainError;
use crypt::crypt::Crypt;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

pub struct Database {
    blockchain: Blockchain,
}

impl Database {

    /// Constructor Function
    pub fn connect(provider: &str) -> Result<Database, BlockchainError> {
        let blockchain = Blockchain::new("http://localhost:8545");
        Ok(Database {
            blockchain,
        })
    }

    // TODO Accept multiple queries at a time
    pub fn execute(&self, query: &str) -> String {
        let parser: Parser = Self::parse(query);
        let ret = match *parser.keywords.get(0).unwrap() {
            YADQL::Insert(ref k, ref v) => self.blockchain.insert(k, v),
            YADQL::Delete(ref k) => self.blockchain.delete(k),
            YADQL::Update(ref k, ref v) => self.blockchain.update(k, v),
            YADQL::Read(ref k) => self.blockchain.read(k),
            _ => panic!("Nothing read in query!")
        };
        return ret.unwrap().payload;
    }

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
}
