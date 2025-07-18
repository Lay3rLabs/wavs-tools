// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWavsServiceManager} from "@wavs/eigenlayer/ecdsa/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/eigenlayer/ecdsa/interfaces/IWavsServiceHandler.sol";

contract RandomnessConsumer is IWavsServiceHandler {
    IWavsServiceManager private _serviceManager;

    event RandomnessReceived(bytes32 randomness);

    constructor(IWavsServiceManager serviceManager) {
        _serviceManager = serviceManager;
    }

    function getServiceManager() external view returns (address) {
        return address(_serviceManager);
    }

    function handleSignedEnvelope(
        IWavsServiceHandler.Envelope calldata envelope,
        IWavsServiceHandler.SignatureData calldata signatureData
    ) external {
        _serviceManager.validate(envelope, signatureData);

        bytes32 payload = abi.decode(envelope.payload, (bytes32));

        emit RandomnessReceived(payload);
    }
}
