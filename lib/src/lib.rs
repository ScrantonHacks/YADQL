#[macro_use]
extern crate serde_derive;
extern crate colored;

mod core;
mod parser;

use std::process;
use std::error::Error;
use colored::Colorize;

use parser::parser::Parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
