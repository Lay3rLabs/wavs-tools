// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IRegistryCoordinator {
    function quorumCount() external view returns (uint8);
    function getOperatorId(address operator) external view returns (bytes32);
    function getOperatorFromId(bytes32 operatorId) external view returns (address);
    function getOperatorStatus(address operator) external view returns (uint8); // 0 = never, 1 = reg, 2 = de-reg
    function getCurrentQuorumBitmap(bytes32 operatorId) external view returns (uint256);
    function getOperatorsInQuorum(uint8 quorumNumber) external view returns (address[] memory);
}
