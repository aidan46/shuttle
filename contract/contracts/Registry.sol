// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Registry {
    string entry;

    function setCID(string memory h) public {
        entry = h;
    }

    function getCID() public view returns (string memory) {
        return entry;
    }
}
