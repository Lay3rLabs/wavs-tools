// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {ECDSAStakeRegistry} from "@eigenlayer-middleware/src/unaudited/ECDSAStakeRegistry.sol";

contract AvsWriter is IWavsServiceHandler {
    ECDSAStakeRegistry private _ecdsaStakeRegistry;
    IWavsServiceManager private _serviceManager;

    constructor(IWavsServiceManager serviceManager, ECDSAStakeRegistry ecdsaStakeRegistry) {
        _ecdsaStakeRegistry = ecdsaStakeRegistry;
        _serviceManager = serviceManager;
    }

    function handleSignedEnvelope(
        IWavsServiceHandler.Envelope calldata envelope,
        IWavsServiceHandler.SignatureData calldata signatureData
    ) external {
        _serviceManager.validate(envelope, signatureData);

        (address[][] memory operatorsPerQuorum, bytes memory quorumNumbers) =
            abi.decode(envelope.payload, (address[][], bytes));

        //NOTE: any block limits we should worry about here?
        //NOTE: writer go code uses retry mechanism for this: https://github.com/Layr-Labs/eigenlayer-middleware/blob/3fb5b61076475108bd87d4e6c7352fd60b46af1c/src/interfaces/ISlashingRegistryCoordinator.sol#L362-L363
        _ecdsaStakeRegistry.updateOperatorsForQuorum(operatorsPerQuorum, quorumNumbers);
    }
}
