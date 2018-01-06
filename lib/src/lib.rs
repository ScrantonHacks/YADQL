#[macro_use]
extern crate serde_derive;
extern crate colored;

mod core;
mod parser;

use std::process;
use std::error::Error;
use colored::Colorize;

use parser::parser::Parser;
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

    struct yadql;
    impl yadql {
        fn execute(&self, statement: str) -> str {
            let cmd = parse(statement);
            let ret = '';
            match cmd.OP {
                YADQL::Insert => Blockchain.insert(cmd.arg0, cmd.arg1),
                YADQL::Delete => Blockchain.delete(cmd.arg0),
                YADQL::Update => Blockchain.update(cmd.arg0, cmd.arg1),
                YADQL::Read => {
                    ret = Blockchain.read(cmd.arg0);
                },
            }
            ret
        }
    }

    fn open() {
        return yadql {};
    }
}
