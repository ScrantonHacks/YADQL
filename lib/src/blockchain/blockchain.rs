use blockchain::err::BlockchainError;

use regex::Regex;
use crypt::crypt::Crypt;

struct KeyVal {
    key: i32,
    val: i32
}

pub struct Blockchain {
    pub memory: Vec<KeyVal>
}

pub struct Success {
    pub payload: String
}

impl Blockchain {
    //! # Blockchain
    //! Represents the current state of the blockchain. This honestly shouldn't care if we're submitting or receiving the queries; it should be able to handle upload as well as download. 
    pub fn new() -> Blockchain {
        Blockchain {
            memory: Vec::new()
        }
    }

    pub fn send(&self, operation: &str, key: &str, value: &str) {
        //! ## send(operation: &str, key: &str, value: &str)
        //! Applies transactions being sent from this machine.
        //! It was late when I wrote this... needs fixing bad.
        match operation (
            "insert" => {
                insert(key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'insert', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            };
            "delete" => {
                delete(key);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'delete', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            };
            "update" => {
                update(key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'update', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            };
            "read" => {
                res = read(key)
            };
        )
    }

    pub fn recv(&self) {
        //! ## recv()
        //! Applies transactions downloaded to this machine.
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = c.decrypt(c.verify(c));
        // We get a string in the format "('operation': 'update', key: '{}', value: '{}' )"
        // We want to parse the operation, key, and value out of that.
        let re = Regex::new(r"^\('operation': '(.{6})', key: ('.+'), value: ('.*') \)$").unwrap();
        let caps = re.captures(payload).unwrap()
        match caps.get(0) (
            "insert" => {
                insert(caps.get(1), caps.get(2));
            };
            "delete" => {
                delete(caps.get(1));
            };
            "update" => {
                update(caps.get(1), caps.get(2));
            };
        )
    }

    pub fn insert(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## insert(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Inserts a new value into the blockchain.
        //! Should throw an error if the record already exists.
        self.memory.push(KeyVal {key, val});
    }

    pub fn delete(&self, key: &str) -> Result<Success, BlockchainError> {
        //! ## delete(key: &str) -> Result<Success, BlockchainError>
        //! Marks a record as deleted.
        //! Should fail if the record doesn't exist.
        for x in self.memory.iter() {
            if x.key == key {
                self.memory.remove(x);
            }
        }
    }

    pub fn update(&self, key: &str, value: &str) -> Result<Success, BlockchainError> {
        //! ## update(key: &str, value: &str) -> Result<Success, BlockchainError>
        //! Updates a value in the local database and submits to the blockchain.
        //! Should fail if the record doesn't exist.
        for x in self.memory.iter() {
            if x.key == key {
                x.value = value;
            }
        }
    }

    pub fn read(&self, key: &str) -> Result<Success, BlockchainError> {
        //! ## read(key: &str) -> Result<Success, BlockchainError>
        //! A method on the Blockchain struct that reads data from memory into the application's context for use. This really just worries about the local mirror of the database; we really don't care about what the blockchain looks like.
        //! Should fail if the record doesn't exist.
        self.memory[key]
    }
}
