// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {AvsWriter} from "../src/AvsWriter.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

contract DeployAvsWriter is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");

        console.log("TODO, deploy! private key is:", deployerPrivateKey);

        // // Get addresses from environment variables
        // address serviceManager = vm.envAddress("WAVS_SERVICE_MANAGER_ADDRESS");
        // address ecdsaStakeRegistryAddress = vm.envAddress("ECDSA_STAKE_REGISTRY_ADDRESS");

        // console.log("=== Deploying AVS Contracts ===");
        // console.log("ECDSA Stake Registry:", ecdsaStakeRegistryAddress);
        // console.log("Service Manager:", serviceManager);
        // console.log("Deployer:", vm.addr(deployerPrivateKey));

        // vm.startBroadcast(deployerPrivateKey);

        // // Deploy AvsWriter
        // AvsWriter avsWriter =
        //     new AvsWriter(IWavsServiceManager(serviceManager), ECDSAStakeRegistry(ecdsaStakeRegistryAddress));

        // console.log("AvsWriter deployed at:", address(avsWriter));
        vm.stopBroadcast();
    }
}