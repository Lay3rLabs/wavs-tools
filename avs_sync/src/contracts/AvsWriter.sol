// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "../interfaces/IStakeRegistry.sol";

contract AvsWriter {
    IStakeRegistry public immutable stakeRegistry;

    constructor(address _stakeRegistry) {
        stakeRegistry = IStakeRegistry(_stakeRegistry);
    }

    function updateOperators(address[] calldata operators) external {
        stakeRegistry.UpdateOperators(operators);
    }
}
