use blockchain::err::BlockchainError;

pub struct Blockchain;
pub struct Success {
    pub payload: String
}

use crypt::crypt::*;

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {}
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        unimplemented!();    
    }

    pub fn delete(&self, key: &str) -> Result<Success, BlockchainError> {
        unimplemented!();    
    }

    pub fn update(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        unimplemented!();    
    }

    pub fn read(&self, key: &str) -> Result<Success, BlockchainError> {
        unimplemented!();    
    }
}
