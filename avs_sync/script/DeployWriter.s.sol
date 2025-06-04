// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {AvsWriter} from "../src/contracts/AvsWriter.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

contract DeployAvsContracts is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        // Get addresses from environment variables
        address registryCoordinator = vm.envAddress("REGISTRY_COORDINATOR");
        address serviceManager = vm.envAddress("SERVICE_MANAGER");

        console.log("=== Deploying AVS Contracts ===");
        console.log("Registry Coordinator:", registryCoordinator);
        console.log("Service Manager:", serviceManager);
        console.log("Deployer:", vm.addr(deployerPrivateKey));

        // Validate addresses
        require(registryCoordinator != address(0), "REGISTRY_COORDINATOR cannot be zero address");
        require(serviceManager != address(0), "SERVICE_MANAGER cannot be zero address");
        require(registryCoordinator.code.length > 0, "REGISTRY_COORDINATOR is not a contract");
        require(serviceManager.code.length > 0, "SERVICE_MANAGER is not a contract");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy AvsWriter
        AvsWriter avsWriter =
            new AvsWriter(IWavsServiceManager(serviceManager), IRegistryCoordinator(registryCoordinator));

        console.log("AvsWriter deployed at:", address(avsWriter));
        vm.stopBroadcast();

        console.log("=== Deployment Complete ===");
    }
}
