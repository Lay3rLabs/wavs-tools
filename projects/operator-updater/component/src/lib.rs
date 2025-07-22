mod avs_reader;
#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use anyhow::{anyhow, Result};
use avs_reader::AvsReader;
use bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wavs_wasi_utils::evm::new_evm_provider;
use wstd::runtime::block_on;

use crate::{
    bindings::{
        host::{self, get_evm_chain_config, LogLevel},
        wavs::worker::input::{TriggerData, TriggerDataBlockInterval},
        WasmResponse,
    },
    IWavsServiceManager::IWavsServiceManagerInstance,
};

sol!(
    "../../../node_modules/@wavs/solidity/contracts/src/eigenlayer/ecdsa/interfaces/IWavsOperatorUpdateHandler.sol"
);
use IWavsOperatorUpdateHandler::OperatorUpdatePayload;

sol!(
    #[sol(rpc)]
    IWavsServiceManager,
    "../../../abi/wavs-middleware/IWavsServiceManager.sol/IWavsServiceManager.json"
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentInput {
    pub service_manager_address: Address,
    pub chain_name: String,
    pub block_height: u64,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode the trigger event
        let ComponentInput {
            service_manager_address,
            chain_name,
            block_height: _,
        } = match action.data {
            TriggerData::BlockInterval(TriggerDataBlockInterval {
                block_height,
                chain_name,
            }) => {
                let service_manager_address = host::config_var("service_manager_address")
                    .ok_or("service_manager_address not configured")?
                    .parse()
                    .map_err(|x: alloy_primitives::hex::FromHexError| x.to_string())?;

                Ok(ComponentInput {
                    service_manager_address,
                    chain_name,
                    block_height,
                })
            }
            TriggerData::Raw(data) => serde_json::from_slice(&data).map_err(|e| e.to_string()),
            _ => return Err("Unsupported trigger data type".to_string()),
        }?;
        host::log(
            LogLevel::Info,
            &format!("Starting operator update for chain {chain_name} for service manager {service_manager_address}"),
        );

        block_on(async move {
            let avs_writer_payload = perform_operator_update(chain_name, service_manager_address)
                .await
                .map_err(|e| e.to_string())?;

            if avs_writer_payload
                .operatorsPerQuorum
                .iter()
                .all(|x| x.is_empty())
            {
                return Ok(None);
            }

            // Return the data needed for updateOperatorsForQuorum
            Ok(Some(WasmResponse {
                payload: avs_writer_payload.abi_encode(),
                ordering: None,
            }))
        })
    }
}

async fn perform_operator_update(
    chain_name: String,
    service_manager_address: Address,
) -> Result<OperatorUpdatePayload> {
    let chain_config = get_evm_chain_config(&chain_name)
        .ok_or(anyhow!("Failed to get chain config for: {chain_name}"))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config
            .http_endpoint
            .ok_or(anyhow!("No HTTP endpoint configured"))?,
    );

    let service_manager =
        IWavsServiceManagerInstance::new(service_manager_address, provider.clone());

    // Get the allocation manager
    let allocation_manager_address = service_manager.getAllocationManager().call().await?;

    // Create the AVS reader
    let avs_reader = AvsReader::new(
        allocation_manager_address,
        service_manager_address,
        provider,
    );

    // Get operators from allocation manager
    let operators = avs_reader.get_active_operators().await?;

    host::log(
        LogLevel::Info,
        &format!("Found {} operators", operators.len()),
    );

    // Sort operators in ascending order (required by the contract)
    let mut sorted_operators = operators;
    sorted_operators.sort();

    host::log(
        LogLevel::Info,
        &format!(
            "Found {} active operators in quorum 0",
            sorted_operators.len()
        ),
    );

    // ECDSAStakeRegistry only has quorum 0
    Ok(OperatorUpdatePayload {
        operatorsPerQuorum: vec![sorted_operators],
        quorumNumbers: vec![0u8].into(),
    })
}

export!(Component with_types_in bindings);
