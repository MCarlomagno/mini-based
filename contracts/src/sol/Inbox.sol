// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Inbox {

    uint256 public batchId;

    mapping(uint256 => address) public provers;

    event BatchProposed(uint256 batchId, bytes32[] batch, address prover);
    event BatchProved(uint256 batchId);

    function proposeBatch(bytes32[] memory batch, address prover) public {
        provers[batchId] = prover;
        emit BatchProposed(batchId, batch, prover);
        batchId++;
    }

    function proveBatch(bytes32[] memory batch, bytes memory proof) public {
        require(provers[batchId] == msg.sender, "Invalid prover");
        require(_verifyBatch(batch, proof), "Invalid proof");
        emit BatchProved(batchId);
    }

    function _verifyBatch(bytes32[] memory _batch, bytes memory _proof) private pure returns (bool) {
        return true;
    }
}