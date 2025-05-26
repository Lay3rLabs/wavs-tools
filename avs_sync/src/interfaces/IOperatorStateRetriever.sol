// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

struct Operator {
    address operator;
    uint256 stake;
}

interface IOperatorStateRetriever {
    function GetOperatorState(
        address registryCoordinator,
        uint8[] calldata quorumNumbers,
        uint32 blockNumber
    ) external view returns (Operator[][] memory);

    function GetOperatorState0(
        address registryCoordinator,
        bytes32 operatorId,
        uint32 blockNumber
    ) external view returns (uint256 quorumBitmap, Operator[][] memory stakes);
}
