// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Inbox {
    uint256 public batchId;

    mapping(uint256 => bytes) public batches;

    event BatchProposed(uint256 batchId, bytes batchData);
    event BatchProved(uint256 batchId);

    // anyone can propose a batch
    function proposeBatch(bytes calldata batchData) public {
        batches[batchId] = batchData;
        emit BatchProposed(batchId, batchData);
        batchId++;
    }

    // anyone can prove a batch
    function proveBatch(uint256 id, bytes memory proof) public {
        require(_verifyBatch(batches[id], proof), "Invalid proof");
        emit BatchProved(batchId);
    }

    function _verifyBatch(bytes memory _batch, bytes memory _proof) private pure returns (bool) {
        // TODO: implement proof verification.
        return true;
    }
}