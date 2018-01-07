pragma solidity ^0.4.0;

contract Storage {
  mapping (bytes32 => string) public storg;

  function store(bytes32 key, string val) public {
    storg[key] = val;
  }
}

contract yadql_storage {
  mapping (bytes4 => Storage) public db;
  
  function store(bytes4 pubkey, bytes32 key, string val) public {
    if (db[pubkey] != address(0x0)) {
      db[pubkey].store(key, val);
    } else {
      Storage x = new Storage();
      db[pubkey] = x;
      x.store(key, val);
    }
  }
}
