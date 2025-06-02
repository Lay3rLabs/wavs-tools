// src/interfaces/IStakeRegistry.sol
pragma solidity ^0.8.20;

interface IStakeRegistry {
    struct StakeUpdate {
        uint32 updateBlockNumber;
        uint96 stake;
    }

    function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
    function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber)
        external
        view
        returns (uint256 blockNumber, uint256 stake);

    /**
     * @notice Returns the stake of an operator at a specific block number.
     * @param operatorId ID of the operator.
     * @param quorumNumber Quorum to query.
     * @param blockNumber Block number to retrieve the stake from.
     * @return Stake amount of the operator at the specified block.
     */
    function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber)
        external
        view
        returns (uint96);

    function updateOperators(address[] calldata operators) external;
}
