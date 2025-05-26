// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IRegistryCoordinator.sol";
import "../interfaces/IStakeRegistry.sol";

contract AvsReader {
    address public immutable registryCoordinator;
    address public immutable stakeRegistry;

    constructor(address _registryCoordinator, address _stakeRegistry) {
        registryCoordinator = _registryCoordinator;
        stakeRegistry = _stakeRegistry;
    }

    function getQuorumCount() external view returns (uint8) {
        return IRegistryCoordinator(registryCoordinator).QuorumCount();
    }

    function getCurrentStake(address operator, uint8 quorum) external view returns (uint256) {
        bytes32 operatorId = IRegistryCoordinator(registryCoordinator).GetOperatorId(operator);
        return IStakeRegistry(stakeRegistry).GetCurrentStake(operatorId, quorum);
    }
}
