use core::keywords::YADQL;
use crypt::crypt::Crypt;

pub struct Stream {

}

impl Stream {

    pub fn send(&self, operation: YADQL, key: &str, value: &str) {
        //! ## send(operation: &str, key: &str, value: &str)
        //! Applies transactions being sent from this machine.
        //! It was late when I wrote this... needs fixing bad.
        let res = match operation {
            YADQL::Insert(ref k, ref v) => {
                self.insert(key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'insert', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Delete(ref k) => {
                self.delete(key);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'delete', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Update(ref k, ref v) => {
                self.update(key, value);
                let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
                let payload = format!("('operation': 'update', key: '{}', value: '{}' )", key, value);
                let crypt_sign = c.sign(c.encrypt(String::from(payload)));
                // TODO Send crypt_sign to the blockchain.
            },
            YADQL::Read(ref k) => {
                self.read(key);
            },
        };
    }

    pub fn recv(&self) {
        //! ## recv()
        //! Applies transactions downloaded to this machine.
        let next_query = String::new(); // This is a placeholder. Make sure we get this one from the EVM.
        let c = Crypt::new(String::from("test@radical-yadql.io")); // THIS CANNOT BE THE FINAL EMAIL.
        let payload = c.decrypt(c.verify(next_query));
        // We need to be able to parse ( 'operation': '', ‘key’: ‘’, ‘value’: ‘’ )
        let parser: Parser = parse(payload); // We need a second parser for this, this will be a stand-in for now.
        let ret = match *parser.keywords.get(0).unwrap() (
            YADQL::Insert(ref k, ref v) => {
                insert(k, v);
            },
            YADQL::Delete(ref k) => {
                delete(k);
            },
            YADQL::Update(ref k, ref v) => {
                update(k, v);
            },
        )
    }
}
