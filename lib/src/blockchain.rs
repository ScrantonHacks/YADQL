mod Blockchain {
    //! # mod Blockchain
    //! Allows interaction with the Ethereum blockchain.
    fn insert(key: str, val: str) -> str {
        //! ## insert(key: str, val: str) -> str
        //! Takes a key and value and inserts them into the blockchain.
        //! Returns completed status as a string.
        // encrypt
        // send
    }
    
    fn update(key: str, val: str) -> str {
        //! ## update(key: str, val: str) -> str
        //! Updates an existing value in the blockchain.
        //! Returns completed status as a string.
        // encrypt
        // send
    }
    
    fn delete(key: str) -> str {
        //! ## delete(key: str) -> str
        //! Removes an existing value from the blockchain.
        //! Returns completed status as a string.
        // encrypt
        // send
    }
    
    fn read(key: str) -> str {
        //! ## read(key: str) -> str
        //! Reads the value associated with `key` from the blockchain.
        //! Returns either the value if it exists, or an empty string if it doesn't.
        // can I verify this signature?
        // yes?
        //  can I decrypt it?
        //  yes?
        //  decrypt it, return it
        //  no?
        //  see who can, have them decrypt, get it back
        // no?
        //  then we really don't care
    }
}
