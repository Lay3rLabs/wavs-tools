mod avs_reader;
#[allow(warnings)]
mod bindings;

use alloy_network::Ethereum;
use alloy_primitives::Address;
use anyhow::{anyhow, Result};
use avs_reader::AvsReader;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, WasmResponse},
    Guest, TriggerAction,
};
use serde::{Deserialize, Serialize};
use wavs_wasi_utils::evm::new_evm_provider;
use wstd::runtime::block_on;

use crate::bindings::{
    host::{self, get_evm_chain_config},
    wavs::worker::layer_types::{BlockIntervalData, LogLevel},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentInput {
    pub ecdsa_stake_registry_address: String,
    pub chain_name: String,
    pub block_height: u64,
    pub lookback_blocks: Option<u64>, // How many blocks to look back for events
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOperatorsForQuorumData {
    pub operators_per_quorum: Vec<Vec<Address>>, // address[][] - operators for each quorum
    pub quorum_numbers: Vec<u8>, // bytes - quorum identifiers (always [0] for ECDSAStakeRegistry)
    pub total_operators: usize,
    pub block_height: u64,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode the trigger event
        let ComponentInput {
            ecdsa_stake_registry_address,
            chain_name,
            block_height,
            lookback_blocks,
        } = match action.data {
            TriggerData::BlockInterval(BlockIntervalData { block_height, chain_name }) => {
                let ecdsa_stake_registry_address = host::config_var("ecdsa_stake_registry_address")
                    .ok_or("ecdsa_stake_registry_address not configured")?;

                // Get lookback period (default 1000 blocks like your script)
                let lookback_blocks = host::config_var("lookback_blocks")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1000u64);

                Ok(ComponentInput {
                    ecdsa_stake_registry_address,
                    chain_name,
                    block_height,
                    lookback_blocks: Some(lookback_blocks),
                })
            }
            TriggerData::Raw(data) => serde_json::from_slice(&data).map_err(|e| e.to_string()),
            _ => return Err("Unsupported trigger data type".to_string()),
        }?;

        host::log(LogLevel::Info, &format!("Starting AVS sync for chain: {}", chain_name));
        host::log(
            LogLevel::Info,
            &format!("ECDSA Stake Registry: {}", ecdsa_stake_registry_address),
        );

        block_on(async move {
            let ecdsa_stake_registry_address = ecdsa_stake_registry_address
                .parse()
                .map_err(|e: alloy_primitives::hex::FromHexError| e.to_string())?;

            let update_data = perform_avs_sync(
                chain_name,
                block_height,
                ecdsa_stake_registry_address,
                lookback_blocks,
            )
            .await
            .map_err(|e| e.to_string())?;

            host::log(
                LogLevel::Info,
                &format!(
                    "AVS sync completed: {} total operators in quorum 0 at block {}",
                    update_data.total_operators, update_data.block_height
                ),
            );

            // Return the data needed for updateOperatorsForQuorum
            let response_data =
                serde_json::to_vec(&(update_data.operators_per_quorum, update_data.quorum_numbers))
                    .map_err(|e| e.to_string())?;
            Ok(Some(WasmResponse { payload: response_data, ordering: None }))
        })
    }
}

async fn perform_avs_sync(
    chain_name: String,
    block_height: u64,
    ecdsa_stake_registry_address: Address,
    lookback_blocks: Option<u64>,
) -> Result<UpdateOperatorsForQuorumData> {
    let chain_config = get_evm_chain_config(&chain_name)
        .ok_or(anyhow!("Failed to get chain config for: {}", chain_name))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config.http_endpoint.ok_or(anyhow!("No HTTP endpoint configured"))?,
    );

    // Create the AVS reader for ECDSAStakeRegistry
    let avs_reader = AvsReader::new(ecdsa_stake_registry_address, provider);

    // ECDSAStakeRegistry has only one quorum (quorum 0)
    let quorum_count = avs_reader.get_quorum_count().await?;
    host::log(LogLevel::Info, &format!("ECDSAStakeRegistry has {} quorum", quorum_count));

    // Get operators by querying OperatorRegistered events (like your script)
    let lookback = lookback_blocks.unwrap_or(1000);
    let from_block = if block_height > lookback { block_height - lookback } else { 0 };

    host::log(
        LogLevel::Info,
        &format!(
            "Querying OperatorRegistered events from block {} to {}",
            from_block, block_height
        ),
    );

    let active_operators = avs_reader.get_active_operators(from_block, Some(block_height)).await?;

    host::log(LogLevel::Info, &format!("Found {} active operators", active_operators.len()));

    // Log each operator with their weight
    for operator in &active_operators {
        let weight = avs_reader.get_operator_weight(*operator).await?;
        host::log(LogLevel::Debug, &format!("Operator {} weight: {}", operator, weight));
    }

    // Sort operators in ascending order (required by the contract)
    let mut sorted_operators = active_operators;
    sorted_operators.sort();

    host::log(
        LogLevel::Info,
        &format!("Found {} active operators in quorum 0", sorted_operators.len()),
    );

    // ECDSAStakeRegistry only has quorum 0
    let operators_per_quorum = vec![sorted_operators.clone()];
    let quorum_numbers = vec![0u8];

    let total_operators = sorted_operators.len();

    Ok(UpdateOperatorsForQuorumData {
        operators_per_quorum,
        quorum_numbers,
        total_operators,
        block_height,
    })
}

export!(Component with_types_in bindings);
