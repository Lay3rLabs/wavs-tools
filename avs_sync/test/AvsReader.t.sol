// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../src/contracts/AvsReader.sol";

contract AvsReaderTest is Test {
    AvsReader public avsReader;

    address constant REGISTRY_COORDINATOR = address(0x1);
    address constant STAKE_REGISTRY = address(0x2);
    address constant OPERATOR_STATE_RETRIEVER = address(0x3);

    function setUp() public {
        avsReader = new AvsReader(REGISTRY_COORDINATOR, STAKE_REGISTRY, OPERATOR_STATE_RETRIEVER);
    }

    function test_Constructor() public view {
        assertEq(avsReader.registryCoordinator(), REGISTRY_COORDINATOR);
        assertEq(avsReader.stakeRegistry(), STAKE_REGISTRY);
        assertEq(avsReader.operatorStateRetriever(), OPERATOR_STATE_RETRIEVER);
    }
}
