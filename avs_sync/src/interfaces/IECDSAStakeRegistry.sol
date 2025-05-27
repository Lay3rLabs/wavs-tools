// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IECDSAStakeRegistry {
    /**
     * @notice Updates the list of active operators based on stake changes.
     * @dev Callable only by owner or trusted WAVS component.
     * @param operators Array of operator addresses to update.
     */
    function updateOperators(address[] calldata operators) external;

    /**
     * @notice Emitted when an operator's stake is updated.
     * @param operator Address of the operator being updated.
     */
    event OperatorUpdated(address indexed operator);
}
