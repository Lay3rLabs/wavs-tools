// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IRegistryCoordinator.sol";
import "../interfaces/IStakeRegistry.sol";
import "../interfaces/IOperatorStateRetriever.sol";

contract AvsReader {
    address public immutable registryCoordinator;
    address public immutable stakeRegistry;
    address public immutable operatorStateRetriever;

    constructor(address _registryCoordinator, address _stakeRegistry, address _operatorStateRetriever) {
        registryCoordinator = _registryCoordinator;
        stakeRegistry = _stakeRegistry;
        operatorStateRetriever = _operatorStateRetriever;
    }

    /// @notice Returns the total number of quorums.
    function getQuorumCount() external view returns (uint8) {
        return IRegistryCoordinator(registryCoordinator).QuorumCount();
    }

    /// @notice Returns list of operator addresses per quorum.
    function getOperatorAddrsInQuorumsAtCurrentBlock(uint8[] calldata quorumNumbers)
        external
        view
        returns (address[][] memory)
    {
        // Call the original function that returns Operator[][] structs
        IOperatorStateRetriever.Operator[][] memory operatorsWithStake = IOperatorStateRetriever(operatorStateRetriever)
            .GetOperatorState(registryCoordinator, quorumNumbers, uint32(block.number));

        // Convert to an array of arrays of addresses only
        address[][] memory result = new address[][](operatorsWithStake.length);

        for (uint256 i = 0; i < operatorsWithStake.length; i++) {
            result[i] = new address[](operatorsWithStake[i].length);
            for (uint256 j = 0; j < operatorsWithStake[i].length; j++) {
                result[i][j] = operatorsWithStake[i][j].operator;
            }
        }

        return result;
    }

    /// @notice Checks if an operator is registered.
    function isOperatorRegistered(address operator) external view returns (bool) {
        uint8 status = IRegistryCoordinator(registryCoordinator).GetOperatorStatus(operator);
        return status == 1; // 1 = REGISTERED
    }

    /// @notice Gets current stake of an operator in a specific quorum.
    function getCurrentStake(address operator, uint8 quorum) external view returns (uint256) {
        bytes32 operatorId = IRegistryCoordinator(registryCoordinator).GetOperatorId(operator);
        return IStakeRegistry(stakeRegistry).GetCurrentStake(operatorId, quorum);
    }

    /// @notice Gets latest stake update for an operator in a specific quorum.
    function getLatestStakeUpdate(address operator, uint8 quorum)
        external
        view
        returns (uint256 blockNumber, uint256 stake)
    {
        bytes32 operatorId = IRegistryCoordinator(registryCoordinator).GetOperatorId(operator);
        return IStakeRegistry(stakeRegistry).GetLatestStakeUpdate(operatorId, quorum);
    }

    /// @notice Gets all operators in a given quorum.
    function getOperatorsInQuorum(uint8 quorumNumber) external view returns (address[] memory) {
        return IRegistryCoordinator(registryCoordinator).GetOperatorsInQuorum(quorumNumber);
    }
}
