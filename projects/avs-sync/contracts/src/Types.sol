// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

struct AvsWriterPayload {
    address[][] operatorsPerQuorum;
    bytes quorumNumbers;
}
