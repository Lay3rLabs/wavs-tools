// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IBLSApkRegistry {
    function operatorToPubkeyHash(address operator) external view returns (bytes32);
    function pubkeyHashToOperator(bytes32 pubkeyHash) external view returns (address);
    function operatorToPubkey(address operator) external view returns (bytes memory g1Pubkey);
    function currentApk(uint8 quorumNumber) external view returns (bytes memory g1Apk);
}
