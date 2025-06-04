mod avs_reader;
#[allow(warnings)]
mod bindings;

use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_provider::Provider;
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
    wavs::worker::layer_types::LogLevel,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentInput {
    pub registry_coordinator_address: String,
    pub operator_state_retriever_address: String,
    pub chain_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOperatorsForQuorumData {
    pub operators_per_quorum: Vec<Vec<Address>>, // address[][] - operators for each quorum
    pub quorum_numbers: Vec<u8>,                 // bytes - quorum identifiers
    pub total_operators: usize,
    pub block_height: u64,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode the trigger event
        let ComponentInput {
            registry_coordinator_address,
            operator_state_retriever_address,
            chain_name,
        } = match action.data {
            TriggerData::Cron(_) => {
                let registry_coordinator_address = host::config_var("registry_coordinator_address")
                    .ok_or("registry_coordinator_address not configured")?;
                let operator_state_retriever_address =
                    host::config_var("operator_state_retriever_address")
                        .ok_or("operator_state_retriever_address not configured")?;
                let chain_name =
                    host::config_var("chain_name").ok_or("chain_name not configured")?;

                Ok(ComponentInput {
                    registry_coordinator_address,
                    operator_state_retriever_address,
                    chain_name,
                })
            }
            TriggerData::Raw(data) => serde_json::from_slice(&data).map_err(|e| e.to_string()),
            _ => return Err("Unsupported trigger data type".to_string()),
        }?;

        host::log(LogLevel::Info, &format!("Starting AVS sync for chain: {}", chain_name));
        host::log(
            LogLevel::Info,
            &format!("Registry coordinator: {}", registry_coordinator_address),
        );
        host::log(
            LogLevel::Info,
            &format!("Operator state retriever: {}", operator_state_retriever_address),
        );

        block_on(async move {
            let registry_coordinator_address = registry_coordinator_address
                .parse()
                .map_err(|e: alloy_primitives::hex::FromHexError| e.to_string())?;
            let operator_state_retriever_address = operator_state_retriever_address
                .parse()
                .map_err(|e: alloy_primitives::hex::FromHexError| e.to_string())?;

            let update_data = perform_avs_sync(
                chain_name,
                registry_coordinator_address,
                operator_state_retriever_address,
            )
            .await
            .map_err(|e| e.to_string())?;

            host::log(
                LogLevel::Info,
                &format!(
                    "AVS sync completed: {} total operators across {} quorums at block {}",
                    update_data.total_operators,
                    update_data.quorum_numbers.len(),
                    update_data.block_height
                ),
            );

            // Log operators per quorum
            for (i, operators) in update_data.operators_per_quorum.iter().enumerate() {
                if i < update_data.quorum_numbers.len() {
                    host::log(
                        LogLevel::Info,
                        &format!(
                            "Quorum {}: {} operators",
                            update_data.quorum_numbers[i],
                            operators.len()
                        ),
                    );
                }
            }

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
    registry_coordinator_address: Address,
    operator_state_retriever_address: Address,
) -> Result<UpdateOperatorsForQuorumData> {
    let chain_config = get_evm_chain_config(&chain_name)
        .ok_or(anyhow!("Failed to get chain config for: {}", chain_name))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config.http_endpoint.ok_or(anyhow!("No HTTP endpoint configured"))?,
    );

    // Get current block height
    let block_height = provider.get_block_number().await?;

    // Create the AVS reader
    let avs_reader =
        AvsReader::new(registry_coordinator_address, operator_state_retriever_address, provider);

    // Get the number of quorums
    let quorum_count = avs_reader.get_quorum_count().await?;
    host::log(LogLevel::Info, &format!("Found {} quorums", quorum_count));

    if quorum_count == 0 {
        return Ok(UpdateOperatorsForQuorumData {
            operators_per_quorum: Vec::new(),
            quorum_numbers: Vec::new(),
            total_operators: 0,
            block_height,
        });
    }

    // Collect operators for each quorum
    let mut operators_per_quorum = Vec::new();
    let mut quorum_numbers = Vec::new();
    let mut total_unique_operators = std::collections::HashSet::new();

    for quorum in 0..quorum_count {
        host::log(LogLevel::Debug, &format!("Processing quorum {}", quorum));

        let mut operators = avs_reader.get_operators_in_quorum(quorum).await?;
        host::log(
            LogLevel::Debug,
            &format!("Found {} operators in quorum {}", operators.len(), quorum),
        );

        // Sort in ascending order
        operators.sort();

        // Add to unique operators count
        for operator in &operators {
            total_unique_operators.insert(*operator);
        }

        // Add quorum data
        operators_per_quorum.push(operators);
        quorum_numbers.push(quorum);
    }

    let total_operators = total_unique_operators.len();
    host::log(
        LogLevel::Info,
        &format!("Found {} unique operators across all quorums", total_operators),
    );

    Ok(UpdateOperatorsForQuorumData {
        operators_per_quorum,
        quorum_numbers,
        total_operators,
        block_height,
    })
}

export!(Component with_types_in bindings);
