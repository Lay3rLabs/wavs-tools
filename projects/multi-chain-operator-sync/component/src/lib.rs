#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod utils;

use alloy_network::Ethereum;
use alloy_primitives::{Address, Uint};
use alloy_provider::RootProvider;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use anyhow::anyhow;
use bindings::{export, wavs::worker::layer_types::WasmResponse, Guest, TriggerAction};
use wavs_wasi_utils::{decode_event_log_data, evm::new_evm_provider};
use wstd::runtime::block_on;

use crate::{
    bindings::{
        host::{self, get_evm_chain_config},
        wavs::worker::layer_types::{
            BlockIntervalData, LogLevel, TriggerData, TriggerDataEvmContractEvent,
        },
    },
    wavs_service_manager::WavsServiceManager::WavsServiceManagerInstance,
    AllocationManager::{AllocationManagerInstance, OperatorSet},
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
        address[] signingKeyAddresses;
        uint256[] weights;
    }
});

mod wavs_service_manager {
    use alloy_sol_macro::sol;

    sol!(
        #[sol(rpc)]
        WavsServiceManager,
        "../../../abi/wavs-middleware/WavsServiceManager.sol/WavsServiceManager.json"
    );
}

sol!(
    #[sol(rpc)]
    ECDSAStakeRegistry,
    "../../../abi/eigenlayer-middleware/ECDSAStakeRegistry.sol/ECDSAStakeRegistry.json"
);

sol!(
    #[sol(rpc)]
    AllocationManager,
    "../../../abi/eigenlayer-middleware/AllocationManager.sol/AllocationManager.json"
);

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        match action.data {
            // Register + Deregister
            TriggerData::EvmContractEvent(TriggerDataEvmContractEvent {
                contract_address,
                chain_name,
                log,
                block_height,
            }) => {
                let chain_config = get_evm_chain_config(&chain_name)
                    .ok_or(format!("Could not get chain config for {chain_name}"))?;
                let endpoint = chain_config
                    .http_endpoint
                    .ok_or(format!("No http endpoint configured for {chain_name}"))?;

                let provider = new_evm_provider::<Ethereum>(endpoint);
                let stake_registry =
                    ECDSAStakeRegistryInstance::new(contract_address.into(), provider);

                block_on(async move {
                    let maybe_register_event: anyhow::Result<
                        ECDSAStakeRegistry::OperatorRegistered,
                    > = decode_event_log_data!(log.clone());
                    let maybe_deregister_event: anyhow::Result<
                        ECDSAStakeRegistry::OperatorDeregistered,
                    > = decode_event_log_data!(log.clone());
                    if let Ok(ECDSAStakeRegistry::OperatorRegistered { operator, avs: _ }) =
                        maybe_register_event
                    {
                        let result = handle_register_event(stake_registry, operator, block_height)
                            .await
                            .map_err(|e: anyhow::Error| e.to_string())?;

                        Ok(Some(WasmResponse {
                            payload: result.abi_encode(),
                            ordering: None,
                        }))
                    } else if let Ok(ECDSAStakeRegistry::OperatorDeregistered {
                        operator,
                        avs: _,
                    }) = maybe_deregister_event
                    {
                        let result =
                            handle_deregister_event(stake_registry, operator, block_height)
                                .await
                                .map_err(|e| e.to_string())?;

                        Ok(Some(WasmResponse {
                            payload: result.abi_encode(),
                            ordering: None,
                        }))
                    } else {
                        return Err(format!("Could not decode the event {log:?}"));
                    }
                })
            }
            // Update
            TriggerData::BlockInterval(BlockIntervalData {
                chain_name,
                block_height,
            }) => {
                let service_manager_address: Address = host::config_var("service_manager_address")
                    .ok_or("service_manager_address not configured")?
                    .parse()
                    .map_err(|e: alloy_primitives::hex::FromHexError| e.to_string())?;

                block_on(async move {
                    let result =
                        handle_update_event(chain_name, block_height, service_manager_address)
                            .await
                            .map_err(|e| e.to_string())?;

                    Ok(Some(WasmResponse {
                        payload: result.abi_encode(),
                        ordering: None,
                    }))
                })
            }
            _ => Err(format!(
                "Component did not expect trigger action {action:?}"
            )),
        }
    }
}

async fn handle_register_event(
    stake_registry: ECDSAStakeRegistryInstance<RootProvider>,
    operator: Address,
    block_height: u64,
) -> anyhow::Result<UpdateWithId> {
    host::log(
        LogLevel::Info,
        &format!("Querying register info for operator {operator} at block {block_height}"),
    );

    // Query the current signing key for operator
    let signing_key_address = stake_registry
        .getLatestOperatorSigningKey(operator)
        .call()
        .await?;

    host::log(
        LogLevel::Info,
        &format!("Signing key address: {signing_key_address}"),
    );

    // Get operator's stake
    let weight = stake_registry.getOperatorWeight(operator).call().await?;

    host::log(LogLevel::Info, &format!("Weight: {weight}"));

    // Get the threshold weight
    let threshold_weight = stake_registry
        .getLastCheckpointThresholdWeight()
        .call()
        .await?;

    host::log(
        LogLevel::Info,
        &format!("Threshold weight: {threshold_weight}"),
    );

    Ok(UpdateWithId {
        operators: vec![operator],
        signingKeyAddresses: vec![signing_key_address],
        weights: vec![weight],
        triggerId: block_height,
        thresholdWeight: threshold_weight,
    })
}

async fn handle_deregister_event(
    stake_registry: ECDSAStakeRegistryInstance<RootProvider>,
    operator: Address,
    block_height: u64,
) -> anyhow::Result<UpdateWithId> {
    host::log(
        LogLevel::Info,
        &format!("Querying deregister info for operator {operator} at block {block_height}"),
    );

    // Get the threshold weight
    let threshold_weight = stake_registry
        .getLastCheckpointThresholdWeight()
        .call()
        .await?;

    host::log(
        LogLevel::Info,
        &format!("Threshold weight: {threshold_weight}"),
    );

    Ok(UpdateWithId {
        triggerId: block_height,
        thresholdWeight: threshold_weight,
        operators: vec![operator],
        signingKeyAddresses: vec![Address::ZERO],
        weights: vec![Uint::ZERO],
    })
}

async fn handle_update_event(
    chain_name: String,
    block_height: u64,
    service_manager_address: Address,
) -> anyhow::Result<UpdateWithId> {
    let chain_config = get_evm_chain_config(&chain_name)
        .ok_or(anyhow!("Failed to get chain config for: {chain_name}"))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config
            .http_endpoint
            .ok_or(anyhow!("No HTTP endpoint configured"))?,
    );

    let service_manager =
        WavsServiceManagerInstance::new(service_manager_address, provider.clone());

    let stake_registry_address = service_manager.stakeRegistry().call().await?;
    host::log(
        LogLevel::Info,
        &format!("Stake registry address: {stake_registry_address}"),
    );
    let allocation_manager_address = service_manager.allocationManager().call().await?;
    host::log(
        LogLevel::Info,
        &format!("Allocation manager address: {allocation_manager_address}"),
    );

    let stake_registry = ECDSAStakeRegistryInstance::new(stake_registry_address, provider.clone());
    let allocation_manager =
        AllocationManagerInstance::new(allocation_manager_address, provider.clone());

    let threshold_weight = stake_registry
        .getLastCheckpointThresholdWeight()
        .call()
        .await?;
    host::log(
        LogLevel::Info,
        &format!("Threshold weight: {threshold_weight}"),
    );

    let operator_set = OperatorSet {
        avs: service_manager_address,
        id: 1,
    };
    let operators = allocation_manager.getMembers(operator_set).call().await?;

    let mut weights = vec![];
    let mut signing_key_addresses = vec![];
    for operator in operators.iter() {
        let weight = stake_registry.getOperatorWeight(*operator).call().await?;
        let signing_key_address = stake_registry
            .getLatestOperatorSigningKey(*operator)
            .call()
            .await?;

        host::log(
            LogLevel::Info,
            &format!(
                "Operator: {operator}, Weight: {weight}, Signing key address: {signing_key_address}"
            ),
        );

        weights.push(weight);
        signing_key_addresses.push(signing_key_address);
    }

    Ok(UpdateWithId {
        triggerId: block_height,
        thresholdWeight: threshold_weight,
        operators,
        signingKeyAddresses: signing_key_addresses,
        weights,
    })
}

export!(Component with_types_in bindings);
