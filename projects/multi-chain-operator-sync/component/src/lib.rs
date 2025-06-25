#[allow(warnings)]
mod bindings;
mod utils;

use alloy_network::Ethereum;
use alloy_primitives::{Address, Uint};
use alloy_provider::RootProvider;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};
use wavs_wasi_utils::{decode_event_log_data, evm::new_evm_provider};
use wstd::runtime::block_on;

use crate::{
    bindings::{
        host::get_evm_chain_config,
        wavs::worker::layer_types::{TriggerData, TriggerDataEvmContractEvent},
    },
    ECDSAStakeRegistry::ECDSAStakeRegistryInstance,
    IMirrorUpdateTypes::UpdateWithId,
};

sol!(interface IMirrorUpdateTypes {
    error InvalidTriggerId(uint64 expectedTriggerId);

    /// @notice DataWithId is a struct containing a trigger ID and updated operator info
    struct UpdateWithId {
        uint64 triggerId;
        uint256 thresholdWeight;
        address[] operators;
        address[] signingKeys;
        uint256[] weights;
    }
});

sol!(
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "../../../abi/eigenlayer-middleware/ECDSAStakeRegistry.sol/ECDSAStakeRegistry.json"
);

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        let TriggerDataEvmContractEvent {
            contract_address,
            chain_name,
            log,
            block_height,
        } = match action.data {
            TriggerData::EvmContractEvent(trigger_data_evm_contract_event) => {
                Ok(trigger_data_evm_contract_event)
            }
            _ => Err(format!(
                "Only evm contract event triggers are supported. Received {:?}",
                action
            )),
        }?;

        let chain_config = get_evm_chain_config(&chain_name)
            .ok_or(format!("Could not get chain config for {}", chain_name))?;
        let endpoint = chain_config
            .http_endpoint
            .ok_or(format!("No http endpoint configured for {}", chain_name))?;

        let provider = new_evm_provider::<Ethereum>(endpoint);
        let stake_registry = ECDSAStakeRegistryInstance::new(contract_address.into(), provider);

        block_on(async move {
            let maybe_register_event: anyhow::Result<ECDSAStakeRegistry::OperatorRegistered> =
                decode_event_log_data!(log.clone());
            if let Ok(register_event) = maybe_register_event {
                let ECDSAStakeRegistry::OperatorRegistered { operator, avs: _ } = register_event;

                let result = handle_register_event(stake_registry, operator, block_height)
                    .await
                    .map_err(|e| e.to_string())?;

                return Ok(Some(WasmResponse {
                    payload: result.abi_encode(),
                    ordering: None,
                }));
            } else {
                return Err(format!("Could not decode the event {:?}", log));
            }
        })
    }
}

async fn handle_register_event(
    stake_registry: ECDSAStakeRegistryInstance<RootProvider>,
    operator: Address,
    block_height: u64,
) -> anyhow::Result<UpdateWithId> {
    // Query the current signing key for operator
    let signing_key = stake_registry
        .getOperatorSigningKeyAtBlock(operator, Uint::from(block_height))
        .call()
        .await?;

    // Get operator's stake
    let weight = stake_registry
        .getOperatorWeightAtBlock(operator, block_height.try_into()?)
        .call()
        .await?;

    #[allow(unreachable_code)]
    Ok(UpdateWithId {
        operators: vec![operator],
        signingKeys: vec![signing_key],
        weights: vec![weight],
        triggerId: todo!(),
        thresholdWeight: todo!(),
    })
}

export!(Component with_types_in bindings);
