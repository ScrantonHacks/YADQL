use blockchain::err::BlockchainError;

use regex::Regex;
use crypt::crypt::Crypt;
use core::keywords::YADQL;
use parser::parser::Parser;
use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

struct KeyVal {
    /// # KeyVal
    key: String,
    val: String
}

pub struct Blockchain {
    /// # struct Blockchain
    pub memory: Vec<KeyVal>
}

pub struct Success {
    /// # Success
    pub payload: String
}

impl Blockchain {
    //! # impl Blockchain
    //! Represents the current state of the blockchain. This honestly shouldn't care if we're submitting or receiving the queries; it should be able to handle upload as well as download. 
    pub fn new(provider: &str) -> Blockchain {
        let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap(); 
        let web3 = web3::Web3::new(transport); 
        let accounts = web3.eth().accounts().wait().unwrap();

        Blockchain {
            memory: Vec::new()
        }
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## insert(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Inserts a new value into the blockchain.
        //! Should throw an error if the record already exists.
        self.memory.push(KeyVal { key: key.to_string(), val: val.to_string() })?;
        Ok(Success { payload: x.val })
    }

    pub fn delete(&self, key: &str) -> Result<Success, BlockchainError> {
        //! ## delete(key: &str) -> Result<Success, BlockchainError>
        //! Marks a record as deleted.
        //! Should fail if the record doesn't exist.
        let ret = for x in self.memory.iter() {
            if x.key == key.to_string() {
                self.memory.remove(x);
                Ok(Success { payload: x.key })
            }
        };
        ret
    }

    pub fn update(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## update(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Updates a value in the local database and submits to the blockchain.
        //! Should fail if the record doesn't exist.
        let ret = for x in self.memory.iter() {
            if x.key == key.to_string() {
                x.val = value.to_string();
                Ok(Success { payload: x.val })
            }
        };
        ret
    }

    pub fn read(&self, key: &str) -> Result<Success, BlockchainError> {
        //! ## read(key: &str) -> Result<Success, BlockchainError>
        //! A method on the Blockchain struct that reads data from memory into the application's context for use. This really just worries about the local mirror of the database; we really don't care about what the blockchain looks like.
        //! Should fail if the record doesn't exist.
        let ret = for x in self.memory.iter() {
            if x.key == key.to_string() {
                Ok(Success { payload: x.val })
            }
        };
        ret
    }
}
