pragma solidity ^0.4.0;

contract Storage {
  mapping (bytes32 => string) public storg;
  
  function store(bytes32 key, string val) public {
    storg[key] = val;
  }
}
