// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "@eigenlayer-middleware/src/interfaces/ISlashingRegistryCoordinator.sol";
import "@eigenlayer-middleware/src/interfaces/IStakeRegistry.sol";
import "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

contract AvsReader {
    address public immutable registryCoordinator;
    address public immutable stakeRegistry;
    address public immutable operatorStateRetriever;

    constructor(
        address _registryCoordinator,
        address _stakeRegistry,
        address _operatorStateRetriever
    ) {
        registryCoordinator = _registryCoordinator;
        stakeRegistry = _stakeRegistry;
        operatorStateRetriever = _operatorStateRetriever;
    }

    /// @notice Returns the total number of quorums.
    function getQuorumCount() external view returns (uint8) {
        return ISlashingRegistryCoordinator(registryCoordinator).quorumCount();
    }

    /// @notice Returns list of operator addresses per quorum.
    function getOperatorAddrsInQuorumsAtCurrentBlock(
        uint8[] calldata quorumNumbers
    ) external view returns (address[][] memory) {
        // Convert uint8[] to bytes
        bytes memory quorumBytes = abi.encodePacked(quorumNumbers);

        // Call the original function that returns Operator[][] structs
        OperatorStateRetriever.Operator[][]
            memory operatorsWithStake = OperatorStateRetriever(
                operatorStateRetriever
            ).getOperatorState(
                    ISlashingRegistryCoordinator(registryCoordinator),
                    quorumBytes,
                    uint32(block.number)
                );

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
    function isOperatorRegistered(
        address operator
    ) external view returns (bool) {
        uint8 status = uint8(
            ISlashingRegistryCoordinator(registryCoordinator).getOperatorStatus(
                operator
            )
        );
        return status == 1; // 1 = REGISTERED
    }

    /// @notice Gets current stake of an operator in a specific quorum.
    function getCurrentStake(
        address operator,
        uint8 quorum
    ) external view returns (uint256) {
        bytes32 operatorId = ISlashingRegistryCoordinator(registryCoordinator)
            .getOperatorId(operator);
        return
            IStakeRegistry(stakeRegistry).getCurrentStake(operatorId, quorum);
    }

    /// @notice Gets latest stake update for an operator in a specific quorum.
    function getLatestStakeUpdate(
        address operator,
        uint8 quorum
    ) external view returns (uint256 blockNumber, uint256 stake) {
        bytes32 operatorId = ISlashingRegistryCoordinator(registryCoordinator)
            .getOperatorId(operator);
        IStakeRegistry.StakeUpdate memory update = IStakeRegistry(stakeRegistry)
            .getLatestStakeUpdate(operatorId, quorum);
        return (update.updateBlockNumber, update.stake);
    }

    /// @notice Gets all operators in a given quorum.
    function getOperatorsInQuorum(
        uint8 quorumNumber
    ) external view returns (address[] memory) {
        uint8[] memory quorumNumbers = new uint8[](1);
        quorumNumbers[0] = quorumNumber;
        address[][] memory operators = this
            .getOperatorAddrsInQuorumsAtCurrentBlock(quorumNumbers);
        return operators[0];
    }

    /**
     * @notice Gets the current stake of multiple operators in the specified quorum.
     * @param operators List of operator addresses to query.
     * @param quorum The quorum number to check stake for.
     * @return stakes Array of stakes corresponding to each operator in the input list.
     */
    function getCurrentStakes(
        address[] calldata operators,
        uint8 quorum
    ) external view returns (uint256[] memory stakes) {
        stakes = new uint256[](operators.length);
        bytes32[] memory operatorIds = new bytes32[](operators.length);

        for (uint256 i = 0; i < operators.length; i++) {
            operatorIds[i] = ISlashingRegistryCoordinator(registryCoordinator)
                .getOperatorId(operators[i]);
        }

        for (uint256 i = 0; i < operators.length; i++) {
            stakes[i] = IStakeRegistry(stakeRegistry).getCurrentStake(
                operatorIds[i],
                quorum
            );
        }
    }

    /**
     * @notice Returns a list of quorums that the given operator is currently part of.
     * @param operator Address of the operator to query.
     * @return quorums Array of quorum numbers where the operator is registered.
     */
    function getQuorumsForOperator(
        address operator
    ) external view returns (uint8[] memory) {
        bytes32 operatorId = ISlashingRegistryCoordinator(registryCoordinator)
            .getOperatorId(operator);
        uint256 bitmap = ISlashingRegistryCoordinator(registryCoordinator)
            .getCurrentQuorumBitmap(operatorId);
        uint8 quorumCount = ISlashingRegistryCoordinator(registryCoordinator)
            .quorumCount();

        uint8[] memory quorums = new uint8[](quorumCount);
        uint8 count;

        for (uint8 i = 0; i < quorumCount; i++) {
            if ((bitmap & (1 << i)) != 0) {
                quorums[count++] = i;
            }
        }

        assembly {
            mstore(quorums, count)
        } // resize array
        return quorums;
    }

    /**
     * @notice Returns the stake of a specific operator in a given quorum at a specific block.
     * @param operator Address of the operator to query.
     * @param quorum Quorum number to check stake in.
     * @param blockNumber Block number to retrieve the stake from.
     * @return Stake amount of the operator in the quorum at the specified block.
     */
    function getStakeAtBlock(
        address operator,
        uint8 quorum,
        uint32 blockNumber
    ) external view returns (uint256) {
        bytes32 operatorId = ISlashingRegistryCoordinator(registryCoordinator)
            .getOperatorId(operator);
        return
            IStakeRegistry(stakeRegistry).getStakeAtBlockNumber(
                operatorId,
                quorum,
                blockNumber
            );
    }
}
