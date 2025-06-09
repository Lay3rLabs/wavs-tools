// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../src/contracts/AvsWriter.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";

contract AvsWriterTest is Test {
    AvsWriter public avsWriter;

    address constant REGISTRY_COORDINATOR = address(0x1);
    address public constant SERVICE_MANAGER = 0x4567890123456789012345678901234567890123;

    function setUp() public {
        avsWriter = new AvsWriter(IWavsServiceManager(SERVICE_MANAGER), IRegistryCoordinator(REGISTRY_COORDINATOR));
    }

    function test_Constructor() public view {
        // Constructor completed successfully if we reach here
        assertTrue(address(avsWriter) != address(0));
    }
}
