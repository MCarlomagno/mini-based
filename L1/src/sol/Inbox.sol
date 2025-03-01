// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Inbox {
    uint256 public batchId;

    mapping(uint256 => bytes[]) public batches;

    event BatchProposed(uint256 batchId, bytes[] batchData, uint256 blockNumber);
    event BatchProved(uint256 batchId, bytes[] batchData, uint256 blockNumber);

    // anyone can propose a batch
    function proposeBatch(bytes[] memory batchData, uint256 blockNumber) public {
        batches[batchId] = batchData;
        emit BatchProposed(batchId, batchData, blockNumber);
        batchId++;
    }

    // anyone can prove a batch
    function proveBatch(uint256 id, bytes memory proof, uint256 blockNumber) public {
        require(_verifyBatch(batches[id], proof), "Invalid proof");
        emit BatchProved(batchId, batches[id], blockNumber);
    }

    function _verifyBatch(bytes[] memory _batch, bytes memory _proof) private pure returns (bool) {
        // TODO: implement proof verification.
        return true;
    }
}