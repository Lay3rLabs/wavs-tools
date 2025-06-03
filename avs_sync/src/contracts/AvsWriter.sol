// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {IRegistryCoordinator} from "@eigenlayer-middleware/src/interfaces/IRegistryCoordinator.sol";

contract AvsWriter is IWavsServiceHandler {
    IRegistryCoordinator private _registryCoordinator;
    IWavsServiceManager private _serviceManager;

    constructor(IWavsServiceManager serviceManager, IRegistryCoordinator registryCoordinator) {
        _registryCoordinator = registryCoordinator;
        _serviceManager = serviceManager;
    }

    function handleSignedEnvelope(
        IWavsServiceHandler.Envelope calldata envelope,
        IWavsServiceHandler.SignatureData calldata signatureData
    ) external {
        _serviceManager.validate(envelope, signatureData);

        address[] memory operators = abi.decode(envelope.payload, (address[]));

        _registryCoordinator.updateOperators(operators);
    }
}
