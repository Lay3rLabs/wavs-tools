// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

contract RandomnessTrigger {
    /// @notice Emitted whenever randomness is requested
    event RandomnessRequested(address indexed requester);

    /// @notice Call this to request randomness
    function requestRandomness() external {
        emit RandomnessRequested(msg.sender);
    }
}
