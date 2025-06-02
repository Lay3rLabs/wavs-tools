#[allow(warnings)]
mod bindings;

use alloy_network::Ethereum;
use alloy_primitives::{Address, U256};
use alloy_provider::Provider;
use alloy_sol_macro::sol;
use anyhow::{anyhow, Result};
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, WasmResponse},
    Guest, TriggerAction,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use wavs_wasi_utils::evm::new_evm_provider;
use wstd::runtime::block_on;

use crate::bindings::{
    host::{self, get_evm_chain_config},
    wavs::worker::layer_types::LogLevel,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    AvsReader,
    "../../out/AvsReader.sol/AvsReader.json"
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentInput {
    pub reader_address: String,
    pub chain_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OperatorStakeSnapshot {
    pub block_height: u64,
    pub timestamp: u64,
    pub operators: HashMap<Address, HashMap<u8, U256>>, // operator -> quorum -> stake
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub operators_to_update: Vec<Address>,
    pub total_operators: usize,
    pub quorums_processed: u8,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        // Decode the trigger event
        let ComponentInput { reader_address, chain_name } = match action.data {
            TriggerData::Cron(_) => {
                let reader_address =
                    host::config_var("reader_address").ok_or("reader_address not configured")?;
                let chain_name =
                    host::config_var("chain_name").ok_or("chain_name not configured")?;

                Ok(ComponentInput { reader_address, chain_name })
            }
            TriggerData::Raw(data) => serde_json::from_slice(&data).map_err(|e| e.to_string()),
            _ => return Err("Unsupported trigger data type".to_string()),
        }?;

        host::log(LogLevel::Info, &format!("Starting AVS sync for chain: {}", chain_name));
        host::log(LogLevel::Info, &format!("Reader address: {}", reader_address));

        block_on(async move {
            let reader_address = reader_address
                .parse()
                .map_err(|e: alloy_primitives::hex::FromHexError| e.to_string())?;

            let maybe_sync_result =
                perform_avs_sync(chain_name, reader_address).await.map_err(|e| e.to_string())?;

            if let Some(sync_result) = maybe_sync_result {
                if sync_result.operators_to_update.is_empty() {
                    host::log(LogLevel::Info, "No operators need updating");
                    return Ok(None);
                }

                host::log(
                    LogLevel::Info,
                    &format!(
                        "AVS sync completed: {}/{} operators need updating across {} quorums",
                        sync_result.operators_to_update.len(),
                        sync_result.total_operators,
                        sync_result.quorums_processed
                    ),
                );

                // Return just the list of operators that need updating
                let response_data = serde_json::to_vec(&sync_result.operators_to_update)
                    .map_err(|e| e.to_string())?;
                return Ok(Some(WasmResponse { payload: response_data, ordering: None }));
            }

            host::log(LogLevel::Info, "No quorums found or no operators to process");
            Ok(None)
        })
    }
}

async fn perform_avs_sync(
    chain_name: String,
    reader_address: Address,
) -> Result<Option<SyncResult>> {
    let chain_config = get_evm_chain_config(&chain_name)
        .ok_or(anyhow!("Failed to get chain config for: {}", chain_name))?;

    let provider = new_evm_provider::<Ethereum>(
        chain_config.http_endpoint.ok_or(anyhow!("No HTTP endpoint configured"))?,
    );

    // Get current block height
    let block_height = provider.get_block_number().await?;
    let contract = AvsReader::AvsReaderInstance::new(reader_address, provider);

    // Get the number of quorums
    let quorum_count = contract.getQuorumCount().call().await?;
    host::log(LogLevel::Info, &format!("Found {} quorums", quorum_count));

    // Start with the empty snapshot
    let mut current_operators = HashMap::new();
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
    let current_snapshot =
        OperatorStakeSnapshot { block_height, timestamp, operators: current_operators.clone() };

    if quorum_count == 0 {
        host::log(LogLevel::Info, "No quorums found, saving empty snapshot");
        save_snapshot(&current_snapshot, &chain_name)?;
        return Ok(None);
    }

    for quorum in 0..quorum_count {
        host::log(LogLevel::Debug, &format!("Processing quorum {}", quorum));

        let operators = contract.getOperatorsInQuorum(quorum).call().await?;
        host::log(
            LogLevel::Debug,
            &format!("Found {} operators in quorum {}", operators.len(), quorum),
        );

        if operators.is_empty() {
            continue;
        }

        let stakes = contract.getCurrentStakes(operators.clone(), quorum).call().await?;

        for (operator, stake) in operators.iter().zip(stakes.iter()) {
            current_operators.entry(*operator).or_insert_with(HashMap::new).insert(quorum, *stake);
        }
    }

    let total_operators = current_operators.len();
    host::log(
        LogLevel::Info,
        &format!("Found {} unique operators across all quorums", total_operators),
    );

    if total_operators == 0 {
        host::log(LogLevel::Info, "No operators found across all quorums, saving empty snapshot");
        save_snapshot(&current_snapshot, &chain_name)?;
        return Ok(None);
    }

    // Load previous snapshot from filesystem
    let previous_snapshot = load_snapshot(&chain_name);

    // Determine which operators need updating
    let operators_to_update = if let Some(prev_snapshot) = previous_snapshot {
        determine_operators_to_update(&current_snapshot, &prev_snapshot)
    } else {
        // No previous snapshot - save current state and do nothing this run
        host::log(LogLevel::Info, "No previous snapshot found, saving current state for next run");

        save_snapshot(&current_snapshot, &chain_name)?;

        return Ok(None);
    };

    save_snapshot(&current_snapshot, &chain_name)?;

    Ok(Some(SyncResult { operators_to_update, total_operators, quorums_processed: quorum_count }))
}

fn get_snapshot_filename(chain_name: &str) -> String {
    format!("avs_snapshot_{}.json", chain_name)
}

fn load_snapshot(chain_name: &str) -> Option<OperatorStakeSnapshot> {
    let snapshot_filename = get_snapshot_filename(chain_name);
    let snapshot_path = Path::new(&snapshot_filename);

    if !snapshot_path.exists() {
        host::log(LogLevel::Info, "No previous snapshot found");
        return None;
    }

    match fs::read_to_string(snapshot_path) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(snapshot) => {
                host::log(
                    LogLevel::Debug,
                    &format!("Successfully loaded snapshot from {}", snapshot_filename),
                );
                Some(snapshot)
            }
            Err(e) => {
                host::log(
                    LogLevel::Warn,
                    &format!("Failed to deserialize previous snapshot: {}", e),
                );
                None
            }
        },
        Err(e) => {
            host::log(LogLevel::Warn, &format!("Failed to read snapshot file: {}", e));
            None
        }
    }
}

fn save_snapshot(snapshot: &OperatorStakeSnapshot, chain_name: &str) -> Result<()> {
    let snapshot_filename = get_snapshot_filename(chain_name);
    let snapshot_path = Path::new(&snapshot_filename);

    let serialized = serde_json::to_string_pretty(snapshot)
        .map_err(|e| anyhow!("Failed to serialize snapshot: {}", e))?;

    fs::write(snapshot_path, serialized)
        .map_err(|e| anyhow!("Failed to write snapshot file {}: {}", snapshot_filename, e))?;

    host::log(LogLevel::Debug, &format!("Successfully saved snapshot to {}", snapshot_filename));

    Ok(())
}

fn determine_operators_to_update(
    current: &OperatorStakeSnapshot,
    previous: &OperatorStakeSnapshot,
) -> Vec<Address> {
    let mut operators_to_update = Vec::new();

    // Check all current operators
    for (operator, current_stakes) in &current.operators {
        match previous.operators.get(operator) {
            Some(previous_stakes) => {
                // Operator existed before, check if stakes changed
                if current_stakes != previous_stakes {
                    operators_to_update.push(*operator);
                    host::log(LogLevel::Debug, &format!("Operator {} stakes changed", operator));
                }
            }
            None => {
                // New operator
                operators_to_update.push(*operator);
                host::log(LogLevel::Debug, &format!("New operator {} detected", operator));
            }
        }
    }

    // Check for operators that were removed (existed in previous but not in current)
    for operator in previous.operators.keys() {
        if !current.operators.contains_key(operator) {
            operators_to_update.push(*operator);
            host::log(LogLevel::Debug, &format!("Operator {} was removed", operator));
        }
    }

    // Sort operators for consistent ordering (matching Go implementation)
    operators_to_update.sort_by(|a, b| a.cmp(b));

    host::log(LogLevel::Info, &format!("{} operators need updating", operators_to_update.len()));

    operators_to_update
}

export!(Component with_types_in bindings);
