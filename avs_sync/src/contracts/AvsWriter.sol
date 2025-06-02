// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {IECDSAStakeRegistry} from "@eigenlayer-middleware/src/interfaces/IECDSAStakeRegistry.sol";

contract AvsWriter is IWavsServiceHandler {
    IECDSAStakeRegistry private _ecdsaStakeRegistry;
    IWavsServiceManager private _serviceManager;

    constructor(IWavsServiceManager serviceManager, IECDSAStakeRegistry ecdsaStakeRegistry) {
        _ecdsaStakeRegistry = ecdsaStakeRegistry;
        _serviceManager = serviceManager;
    }

    function handleSignedEnvelope(
        IWavsServiceHandler.Envelope calldata envelope,
        IWavsServiceHandler.SignatureData calldata signatureData
    ) external {
        _serviceManager.validate(envelope, signatureData);

        address[] memory operators = abi.decode(envelope.payload, (address[]));

        _ecdsaStakeRegistry.updateOperators(operators);
    }
}
