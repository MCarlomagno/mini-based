// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Inbox {
    uint256 public batchId;

    mapping(uint256 => bytes32[]) public batches;

    event BatchProposed(uint256 batchId, bytes32[] batch);
    event BatchProved(uint256 batchId);

    // anyone can propose a batch
    function proposeBatch(bytes32[] memory batch) public {
        batches[batchId] = batch;
        emit BatchProposed(batchId, batch);
        batchId++;
    }

    // anyone can prove a batch
    function proveBatch(uint256 id, bytes memory proof) public {
        require(_verifyBatch(batches[id], proof), "Invalid proof");
        emit BatchProved(batchId);
    }

    function _verifyBatch(bytes32[] memory _batch, bytes memory _proof) private pure returns (bool) {
        // TODO: implement proof verification.
        return true;
    }
}