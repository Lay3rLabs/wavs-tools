// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {AvsWriter} from "../src/contracts/AvsWriter.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

contract DeployAvsWriter is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PK");

        // Get addresses from environment variables
        address serviceManager = vm.envAddress("WAVS_SERVICE_MANAGER_ADDRESS");
        address registryCoordinator = vm.envAddress("REGISTRY_COORDINATOR");

        console.log("=== Deploying AVS Contracts ===");
        console.log("Registry Coordinator:", registryCoordinator);
        console.log("Service Manager:", serviceManager);
        console.log("Deployer:", vm.addr(deployerPrivateKey));

        vm.startBroadcast(deployerPrivateKey);

        // Deploy AvsWriter
        AvsWriter avsWriter =
            new AvsWriter(IWavsServiceManager(serviceManager), IRegistryCoordinator(registryCoordinator));

        console.log("AvsWriter deployed at:", address(avsWriter));
        vm.stopBroadcast();
    }
}
