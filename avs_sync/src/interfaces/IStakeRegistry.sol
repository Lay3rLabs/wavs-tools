// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IStakeRegistry {
    function GetCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    function GetLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256 blockNumber, uint256 stake);
}
