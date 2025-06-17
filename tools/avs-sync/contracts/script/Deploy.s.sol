// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {AvsWriter} from "../src/AvsWriter.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

contract DeployAvsWriter is Script {
    function run() external {
        address serviceManagerAddress = vm.envAddress("SERVICE_MANAGER_ADDRESS");
        address stakeRegistryAddress = vm.envAddress("STAKE_REGISTRY_ADDRESS");

        require(serviceManagerAddress != address(0), "SERVICE_MANAGER_ADDRESS not set");
        require(stakeRegistryAddress != address(0), "STAKE_REGISTRY_ADDRESS not set");

        vm.startBroadcast();

        // Deploy AvsWriter contract
        AvsWriter avsWriter =
            new AvsWriter(IWavsServiceManager(serviceManagerAddress), ECDSAStakeRegistry(stakeRegistryAddress));

        vm.stopBroadcast();
    }
}
