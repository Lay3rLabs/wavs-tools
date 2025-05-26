// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IRegistryCoordinator {
    function QuorumCount() external view returns (uint8);
    function GetOperatorId(address operator) external view returns (bytes32);
    function GetOperatorFromId(bytes32 operatorId) external view returns (address);
    function GetOperatorStatus(address operator) external view returns (uint8); // 0 = never, 1 = reg, 2 = de-reg
    function GetCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint256);
    function GetOperatorsInQuorum(uint8 quorumNumber) external view returns (address[] memory);
}
