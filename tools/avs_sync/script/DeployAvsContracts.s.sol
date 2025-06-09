// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "../src/contracts/AvsWriter.sol";

contract DeployAvsContracts is Script {
    // Replace these with actual addresses from testnet
    address public constant REGISTRY_COORDINATOR = 0x1234567890123456789012345678901234567890;
    address public constant STAKE_REGISTRY = 0x2345678901234567890123456789012345678901;
    address public constant OPERATOR_STATE_RETRIEVER = 0x3456789012345678901234567890123456789012;
    address public constant ECDSA_STAKE_REGISTRY = 0x4567890123456789012345678901234567890123;
    address public constant SERVICE_MANAGER = 0x4567890123456789012345678901234567890123;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy AvsWriter
        AvsWriter avsWriter =
            new AvsWriter(IWavsServiceManager(SERVICE_MANAGER), IRegistryCoordinator(REGISTRY_COORDINATOR));
        console.log("AvsWriter deployed at:", address(avsWriter));

        vm.stopBroadcast();
    }
}
