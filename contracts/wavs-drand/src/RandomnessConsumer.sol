// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWavsServiceManager} from "@wavs/eigenlayer/ecdsa/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/eigenlayer/ecdsa/interfaces/IWavsServiceHandler.sol";
import {WavsDrandPayload} from "./Types.sol";
import {RandomnessTrigger} from "./RandomnessTrigger.sol";

contract RandomnessConsumer is IWavsServiceHandler {
    IWavsServiceManager private _serviceManager;
    RandomnessTrigger private _trigger;

    event RandomnessReceived(address indexed requester, bytes32 randomness);

    constructor(IWavsServiceManager serviceManager, RandomnessTrigger trigger) {
        _serviceManager = serviceManager;
        _trigger = trigger;
    }

    function getServiceManager() external view returns (address) {
        return address(_serviceManager);
    }

    function handleSignedEnvelope(
        IWavsServiceHandler.Envelope calldata envelope,
        IWavsServiceHandler.SignatureData calldata signatureData
    ) external {
        _serviceManager.validate(envelope, signatureData);

        WavsDrandPayload memory payload = abi.decode(envelope.payload, (WavsDrandPayload));

        // Query the requester from the trigger contract
        address requester = _trigger.triggerToRequester(payload.triggerId);

        emit RandomnessReceived(requester, payload.randomness);
    }
}
