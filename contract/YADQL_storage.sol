pragma solidity ^0.4.19;

contract YADQL_storage {
    uint[1024] data;

    function set(uint x, uint y) public {
        data[x] = y;
    }

    function get(uint x) public constant returns (uint) {
        return data[x];
    }
}
