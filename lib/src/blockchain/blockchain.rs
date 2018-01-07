use blockchain::err::BlockchainError;

use crypt::crypt::Crypt;

pub struct Blockchain;
pub struct Success {
    pub payload: String
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {}
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = format!("('operation': '{}', key: '{}', value: '{}' )", "insert", key, value);
        let crypt_sign = c.sign(c.encrypt(String::from(payload)));
        // TODO Send crypt_sign to the blockchain.
    }

    pub fn delete(&self, key: &str) -> Result<Success, BlockchainError> {
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = format!("('operation': 'delete', key: '{}')", key, value);
        let crypt_sign = c.sign(c.encrypt(String::from(payload)));
        // TODO Send crypt_sign to the blockchain.    
    }

    pub fn update(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = format!("('operation': 'update', key: '{}', value: '{}' )", key, value);
        let crypt_sign = c.sign(c.encrypt(String::from(payload)));
        // TODO Send crypt_sign to the blockchain.
    }

    // TODO We probably don't need this from the blockchain... just from the local database.
    pub fn read(&self, key: &str) -> Result<Success, BlockchainError> {
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = format!("('operation': 'read', key: '{}' )", "insert", key);
        let crypt_sign = c.sign(c.encrypt(String::from(payload)));
        // TODO Send crypt_sign to the blockchain.
    }
}
