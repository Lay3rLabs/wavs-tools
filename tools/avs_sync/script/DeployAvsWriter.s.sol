// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {AvsWriter} from "../src/contracts/AvsWriter.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";

contract DeployAvsWriter is Script {
    function run() external {
        // Get environment variables
        address serviceManagerAddress = vm.envAddress("WAVS_SERVICE_MANAGER_ADDRESS");
        address stakeRegistryAddress = vm.envAddress("ECDSA_STAKE_REGISTRY_ADDRESS");
        
        require(serviceManagerAddress != address(0), "WAVS_SERVICE_MANAGER_ADDRESS not set");
        require(stakeRegistryAddress != address(0), "ECDSA_STAKE_REGISTRY_ADDRESS not set");
        
        vm.startBroadcast();
        
        // Deploy AvsWriter contract
        AvsWriter avsWriter = new AvsWriter(
            IWavsServiceManager(serviceManagerAddress),
            ECDSAStakeRegistry(stakeRegistryAddress)
        );
        
        vm.stopBroadcast();
        
        console.log("AvsWriter deployed at:", address(avsWriter));
    }
}