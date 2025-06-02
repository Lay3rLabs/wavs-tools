// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistry.sol";

contract AvsWriter {
    IECDSAStakeRegistry public immutable ecdsaStakeRegistry;

    constructor(address _ecdsaStakeRegistryAddress) {
        ecdsaStakeRegistry = IECDSAStakeRegistry(_ecdsaStakeRegistryAddress);
    }

    /**
     * @notice Calls updateOperators on the ECDSAStakeRegistry contract.
     * @dev This should be called when operator stakes change significantly.
     * @param operators List of operator addresses to update.
     */
    function updateOperators(address[] calldata operators) external {
        ecdsaStakeRegistry.updateOperators(operators);
    }
}
