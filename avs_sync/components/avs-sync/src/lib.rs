#[allow(warnings)]
mod bindings;
mod evm;

use alloy_primitives::Address;
use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use evm::AvsContracts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wavs_wasi_chain::decode_event_log_data;
use wstd::runtime::block_on;

// Define the event structure we expect from the smart contract
sol! {
    event AvsSync(
        address reader_address,
        address writer_address,
        string chain_name,
        uint256 trigger_id,
        uint256 block_height
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub operators_updated: Vec<Address>,
    pub block_height: u64,
    pub trigger_id: u64,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the trigger event
        let AvsSync { reader_address, writer_address, chain_name, trigger_id, block_height } =
            match action.data {
                TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                    decode_event_log_data!(log)
                        .map_err(|e| format!("Failed to decode event log data: {}", e))
                }
                TriggerData::Raw(_) => {
                    return Err("Raw data is not supported yet".to_string());
                }
                _ => return Err("Unsupported trigger data type".to_string()),
            }?;

        eprintln!("Processing AVS Sync - Trigger ID: {}", trigger_id);
        eprintln!("Chain: {}", chain_name);
        eprintln!("Block Height: {}", block_height);
        eprintln!("Reader: {:?}, Writer: {:?}", reader_address, writer_address);

        block_on(async move {
            let contracts = AvsContracts::new(&chain_name, reader_address, writer_address)?;
            let sync_result = perform_avs_sync(&contracts, trigger_id.as_u64(), block_height.as_u64()).await?;
            
            eprintln!("AVS sync completed: {} operators updated", sync_result.operators_updated.len());
            
            // Return sync result as response
            let response_data = serde_json::to_vec(&sync_result).map_err(|e| e.to_string())?;
            Ok(Some(response_data))
        })
    }
}

async fn perform_avs_sync(contracts: &AvsContracts, trigger_id: u64, block_height: u64) -> Result<SyncResult, String> {
    // 1. Get the number of quorums
    let quorum_count = contracts.get_quorum_count().await?;
    eprintln!("Found {} quorums", quorum_count);
    
    if quorum_count == 0 {
        return Ok(SyncResult {
            operators_updated: vec![],
            block_height,
            trigger_id,
        });
    }
    
    // 2. Collect all operators and their stakes across quorums
    let mut all_operators = HashMap::new();
    
    for quorum in 0..quorum_count {
        eprintln!("Processing quorum {}", quorum);
        
        let operators = contracts.get_operators_in_quorum(quorum).await?;
        eprintln!("Found {} operators in quorum {}", operators.len(), quorum);
        
        if operators.is_empty() {
            continue;
        }
        
        let stakes = contracts.get_current_stakes(&operators, quorum).await?;
        
        for (operator, stake) in operators.iter().zip(stakes.iter()) {
            all_operators.insert(*operator, *stake);
        }
    }
    
    eprintln!("Found {} unique operators across all quorums", all_operators.len());
    
    // 3. For this simplified version, we'll update all operators with non-zero stakes
    // In a real implementation, this would compare against previous IPFS snapshots
    let operators_to_update: Vec<Address> = all_operators
        .iter()
        .filter(|(_, &stake)| stake > 0.into())
        .map(|(&operator, _)| operator)
        .collect();
    
    eprintln!("Determined {} operators need updating", operators_to_update.len());
    
    // 4. Call updateOperators if there are any to update
    if !operators_to_update.is_empty() {
        contracts.update_operators(&operators_to_update).await?;
        eprintln!("Successfully triggered update for {} operators", operators_to_update.len());
    }
    
    Ok(SyncResult {
        operators_updated: operators_to_update,
        block_height,
        trigger_id,
    })
}

export!(Component with_types_in bindings);
