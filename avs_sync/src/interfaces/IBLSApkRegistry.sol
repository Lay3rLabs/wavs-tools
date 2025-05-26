// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IBLSApkRegistry {
    function OperatorToPubkeyHash(address operator) external view returns (bytes32);
    function PubkeyHashToOperator(bytes32 pubkeyHash) external view returns (address);
    function OperatorToPubkey(address operator) external view returns (bytes memory g1Pubkey);
    function CurrentApk(uint8 quorumNumber) external view returns (bytes memory g1Apk);
}
