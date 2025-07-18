// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

struct WavsDrandPayload {
    uint256 triggerId;
    bytes32 randomness;
}

/// @notice Emitted whenever randomness is requested
event RandomnessRequested(uint256 indexed triggerId);
