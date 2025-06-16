// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Test.sol";
import "../src/AvsWriter.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {console} from "forge-std/console.sol";

contract AvsWriterTest is Test {
    AvsWriter public avsWriter;

    address constant ECDSA_STAKE_REGISTRY = address(0x1);
    address public constant SERVICE_MANAGER = 0x4567890123456789012345678901234567890123;

    function setUp() public {
        avsWriter = new AvsWriter(IWavsServiceManager(SERVICE_MANAGER), ECDSAStakeRegistry(ECDSA_STAKE_REGISTRY));
    }

    function test_Constructor() public view {
        // Constructor completed successfully if we reach here
        assertTrue(address(avsWriter) != address(0));
    }

    function test_Decode() public {
        address operator = 0xE2B61A283f1638DC18B9c00F30F85BD090d601f8;
        address[][] memory operatorsPerQuorum = new address[][](1);
        operatorsPerQuorum[0] = new address[](1);
        operatorsPerQuorum[0][0] = operator;
        bytes memory quorumNumbers = hex"00";
        bytes memory payload = abi.encode(operatorsPerQuorum, quorumNumbers);

        // Decode the payload

        (address[][] memory decodedOperatorsPerQuorum, bytes memory decodedQuorumNumbers) =
            abi.decode(payload, (address[][], bytes));

        // Assert that the decoded values match the original values
        assertEq(decodedOperatorsPerQuorum.length, 1, "Operators per quorum length mismatch");
        assertEq(decodedOperatorsPerQuorum[0].length, 1, "Operators in first quorum length mismatch");
        assertEq(decodedOperatorsPerQuorum[0][0], operator, "Operator address mismatch");
        assertEq(decodedQuorumNumbers.length, 1, "Quorum numbers length mismatch");
        assertEq(decodedQuorumNumbers[0], 0x00, "Quorum number mismatch");
        // Log the decoded values for verification
        console.logString("Decoded operatorsPerQuorum[0][0]:");
        console.logAddress(decodedOperatorsPerQuorum[0][0]);
        console.logString("Decoded quorumNumbers:");
        console.logBytes(decodedQuorumNumbers);

        console.logString("Payload:");
        console.logBytes(payload);

        bytes memory envelopeBytes = hex"f969ff33000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000001e0670fd5efed2d0995e4aac01c148266fa1697939b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001000000000000000000000000e2b61a283f1638dc18b9c00f30f85bd090d601f800000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000003cfbfd00000000000000000000000000000000000000000000000000000000000000010000000000000000000000003abed6d0bf125954e26bb2bde3d3f69bf0a213b4000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000041877f496e8aad0a1f0052abf65d6c4e737247e007b1199b55a3aa7e836da4abe10d8e7dbb9f81b20a21cba1f4ed8be91070181c6d9b39c9112113193be4ee0ed71c00000000000000000000000000000000000000000000000000000000000000";

        IWavsServiceHandler.Envelope memory envelope = abi.decode(envelopeBytes, (IWavsServiceHandler.Envelope));

        assertEq(envelope.payload, payload, "Envelope payload mismatch");
    }
}