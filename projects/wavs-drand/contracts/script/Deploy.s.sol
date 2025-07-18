// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {RandomnessConsumer} from "../src/RandomnessConsumer.sol";
import {RandomnessTrigger} from "../src/RandomnessTrigger.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IWavsServiceManager} from "@wavs/eigenlayer/ecdsa/interfaces/IWavsServiceManager.sol";

contract DeployRandomnessContracts is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address serviceManagerAddress = vm.envAddress("SERVICE_MANAGER_ADDRESS");

        require(serviceManagerAddress != address(0), "SERVICE_MANAGER_ADDRESS not set");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy randomness trigger and consumer contracts
        RandomnessTrigger trigger = new RandomnessTrigger();
        new RandomnessConsumer(IWavsServiceManager(serviceManagerAddress), trigger);

        vm.stopBroadcast();
    }
}
