
/// Keywords available in YADQL
/// Any operation that changes state requires a gas fee to be charged to
/// the account holder
/// Insert KEY, VAL || Requires Gas
/// Delete KEY      || Requires Gas
/// Update KEY, VAL || Requires Gas
/// READ KEY, VAL
pub enum YADQL {
    Insert(String, String), // Insert Data On to the Blockchain,
    Delete(String),         // Delete data under a key from the Blockchain
    Update(String, String), // Update data from the blockchain
    Read(String),           // Read data from the blockchain
    Nothing,                // empty space
}
