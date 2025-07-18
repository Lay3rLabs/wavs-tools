// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {RandomnessRequested} from "./Types.sol";

contract RandomnessTrigger {
    uint256 public triggerCounter;
    mapping(uint256 => address) public triggerToRequester;

    /// @notice Call this to request randomness
    function requestRandomness() external {
        uint256 triggerId = ++triggerCounter;
        triggerToRequester[triggerId] = msg.sender;
        emit RandomnessRequested(triggerId);
    }
}
