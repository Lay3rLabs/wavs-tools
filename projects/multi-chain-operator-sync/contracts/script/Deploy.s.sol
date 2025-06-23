// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import "forge-std/console.sol";
import {MirrorServiceHandler} from "../src/MirrorServiceHandler.sol";
import {MirrorStakeRegistry} from "@wavs/eigenlayer/src/MirrorStakeRegistry.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";

contract DeployMirrorServiceHandler is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address mirrorStakeRegistryAddress = vm.envAddress("MIRROR_STAKE_REGISTRY_ADDRESS");

        require(mirrorStakeRegistryAddress != address(0), "SERVICE_MANAGER_ADDRESS not set");

        vm.startBroadcast(deployerPrivateKey);

        new MirrorServiceHandler(MirrorStakeRegistry(mirrorStakeRegistryAddress));

        vm.stopBroadcast();
    }
}
