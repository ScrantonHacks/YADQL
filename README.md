# YADQL
## Yet Another Database Query Language
#### A Blockchain Database Query Language
(pronounced YA-di-CAL, like radical but with a Y)

### Concept
IoT devices, in and of themselves, present a serious security risk in the manner that data is saved and transmitted between the various devices owned by a single person or entity. Using a smart contract running on the Ethereum blockchain, we propose the creation of an identity-provable, decentralized, transparent, and private encrypted database which shares common information and configuration between IoT devices, allowing complete control of an entity’s personal data.

### Components
Parser
Functions
Rust? (familiar with parser already, just for QL)
Config File for IOT Device (Web3 Provider, ETH acc, etc)
Private Data Security for all IOT Devices

You will need some OpenPGP client (gpg works fine) installed for encryption purposes.

### Grammar
Insert [x] [y];
Delete [x];
Update [x] [y];
Read [x];

### Identity
User identity will be a mechanic by which public keys are connected and assigned to a single user. An identity will be a data structure maintained by the smart contract that essentially logs a list of all public keys associated with an identity.

### Under the Hood
When a user adds a device to their identity on the YADQL smart contract, their client creates an RSA public/private keypair and registers the public key with the smart contract. When the device registers some piece of configuration data d, the data is packaged into a structure of the configuration:
{
    ‘key’: ‘’,
    ‘value’: ‘’,
}
Which is then RSA-encrypted and hashed, and signed by the device. The smart contract essentially acts as a ledger of all hashed and signed changes to every user’s device configurations. Upon download, the client verifies each signature by using the known public keys for the user’s identity, thereby determining which items it cares about. The verifiable hashes are decrypted and applied transactionally to the current local state of the device. If the device A cannot locally decrypt the data, it determines which device (B) owns the public key and requests the specific data be decrypted. Device B transmits the data back to device A, now re-encrypted for device A. Creation of OpenPGP keys will be up to the system or individual provisioning the device.

### Use Cases
#### 

### Immediate Issues
* Performance. It’s doubtful that our first version will have achieved a speed which may be considered acceptable.

### Future Ideas
* Add multiple layers of identity proof to verify access to an identity from a new device.
* Create mobile apps to explore an identity's data on the blockchain.
