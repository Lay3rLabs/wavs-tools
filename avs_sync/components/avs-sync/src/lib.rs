#[allow(warnings)]
mod bindings;
mod evm;

use alloy_primitives::{Address, U256};
use alloy_sol_macro::sol;
use anyhow::Result;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use evm::AvsContracts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Write},
};
use wavs_wasi_chain::decode_event_log_data;
use wstd::http::{Client, IntoBody, Request};
use wstd::io::AsyncRead;
use wstd::runtime::block_on;

// Define the event structure we expect from the smart contract
sol! {
    event AvsSync(
        address reader_address,
        address writer_address,
        string chain_name,
        uint256 trigger_id,
        uint256 block_height,
        string ipfs_upload_url,
        string previous_snapshot_hash
    );
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperatorStakeSnapshot {
    pub block_height: u64,
    pub timestamp: u64,
    pub operators: HashMap<Address, HashMap<u8, U256>>, // operator -> quorum -> stake
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub operators_updated: Vec<Address>,
    pub snapshot_hash: String,
    pub previous_hash: Option<String>,
    pub block_height: u64,
    pub trigger_id: u64,
}

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the trigger event
        let AvsSync { reader_address, writer_address, chain_name, trigger_id, block_height, ipfs_upload_url, previous_snapshot_hash } =
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
        eprintln!("IPFS Upload URL: {}", ipfs_upload_url);
        eprintln!("Previous Snapshot Hash: {}", previous_snapshot_hash);
        eprintln!("Reader: {:?}, Writer: {:?}", reader_address, writer_address);

        block_on(async move {
            let contracts = AvsContracts::new(&chain_name, reader_address, writer_address)?;
            let sync_result = perform_avs_sync(
                &contracts, 
                trigger_id.as_u64(), 
                block_height.as_u64(),
                &ipfs_upload_url,
                if previous_snapshot_hash.is_empty() { None } else { Some(previous_snapshot_hash) }
            ).await?;
            
            eprintln!("AVS sync completed: {} operators updated", sync_result.operators_updated.len());
            eprintln!("New snapshot hash: {}", sync_result.snapshot_hash);
            
            // Return sync result as response
            let response_data = serde_json::to_vec(&sync_result).map_err(|e| e.to_string())?;
            Ok(Some(response_data))
        })
    }
}

async fn perform_avs_sync(
    contracts: &AvsContracts, 
    trigger_id: u64, 
    block_height: u64,
    ipfs_upload_url: &str,
    previous_snapshot_hash: Option<String>
) -> Result<SyncResult, String> {
    // 1. Get the number of quorums
    let quorum_count = contracts.get_quorum_count().await?;
    eprintln!("Found {} quorums", quorum_count);
    
    if quorum_count == 0 {
        // Even with no quorums, create and store an empty snapshot
        let empty_snapshot = OperatorStakeSnapshot {
            block_height,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            operators: HashMap::new(),
        };
        
        let snapshot_hash = upload_snapshot_to_ipfs(&empty_snapshot, ipfs_upload_url).await?;
        
        return Ok(SyncResult {
            operators_updated: vec![],
            snapshot_hash,
            previous_hash: previous_snapshot_hash,
            block_height,
            trigger_id,
        });
    }
    
    // 2. Collect all operators and their stakes across quorums
    let mut current_operators = HashMap::new();
    
    for quorum in 0..quorum_count {
        eprintln!("Processing quorum {}", quorum);
        
        let operators = contracts.get_operators_in_quorum(quorum).await?;
        eprintln!("Found {} operators in quorum {}", operators.len(), quorum);
        
        if operators.is_empty() {
            continue;
        }
        
        let stakes = contracts.get_current_stakes(&operators, quorum).await?;
        
        for (operator, stake) in operators.iter().zip(stakes.iter()) {
            current_operators.entry(*operator)
                .or_insert_with(HashMap::new)
                .insert(quorum, *stake);
        }
    }
    
    eprintln!("Found {} unique operators across all quorums", current_operators.len());
    
    // 3. Create current snapshot
    let current_snapshot = OperatorStakeSnapshot {
        block_height,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        operators: current_operators.clone(),
    };
    
    // 4. Upload current snapshot to IPFS
    let snapshot_hash = upload_snapshot_to_ipfs(&current_snapshot, ipfs_upload_url).await?;
    eprintln!("Uploaded current snapshot to IPFS: {}", snapshot_hash);
    
    // 5. Compare with previous snapshot to determine what changed
    let operators_to_update = if let Some(prev_hash) = &previous_snapshot_hash {
        eprintln!("Comparing with previous snapshot: {}", prev_hash);
        compare_with_previous_snapshot(&current_snapshot, prev_hash).await?
    } else {
        eprintln!("No previous snapshot - updating all operators with non-zero stakes");
        // First run - update all operators with non-zero stakes
        current_operators
            .iter()
            .filter(|(_, quorum_stakes)| {
                quorum_stakes.values().any(|&stake| stake > 0.into())
            })
            .map(|(&operator, _)| operator)
            .collect()
    };
    
    eprintln!("Determined {} operators need updating", operators_to_update.len());
    
    // 6. Call updateOperators if there are any to update
    if !operators_to_update.is_empty() {
        contracts.update_operators(&operators_to_update).await?;
        eprintln!("Successfully triggered update for {} operators", operators_to_update.len());
    }
    
    Ok(SyncResult {
        operators_updated: operators_to_update,
        snapshot_hash,
        previous_hash: previous_snapshot_hash,
        block_height,
        trigger_id,
    })
}

/// Upload a snapshot to IPFS and return the hash
async fn upload_snapshot_to_ipfs(snapshot: &OperatorStakeSnapshot, ipfs_url: &str) -> Result<String, String> {
    let api_key = std::env::var("WAVS_ENV_LIGHTHOUSE_API_KEY")
        .map_err(|e| format!("Failed to get API key: {}", e))?;
    
    // Serialize snapshot to JSON
    let json_data = serde_json::to_string_pretty(snapshot)
        .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;
    
    // Create temporary file
    let filename = format!("avs_snapshot_{}.json", snapshot.block_height);
    let temp_path = format!("/tmp/{}", filename);
    
    // Ensure /tmp directory exists
    std::fs::create_dir_all("/tmp")
        .map_err(|e| format!("Failed to create /tmp directory: {}", e))?;
    
    // Write JSON to file
    let mut file = File::create(&temp_path)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    file.write_all(json_data.as_bytes())
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;
    
    eprintln!("Uploading snapshot to IPFS: {}", temp_path);
    
    // Read file back for upload
    let mut file = File::open(&temp_path)
        .map_err(|e| format!("Failed to open temp file: {}", e))?;
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)
        .map_err(|e| format!("Failed to read temp file: {}", e))?;
    
    // Create multipart request
    let boundary = "----RustBoundary";
    let body = format!(
        "--{}\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n\
        Content-Type: application/json\r\n\r\n",
        boundary, filename
    );
    
    let mut request_body = body.into_bytes();
    request_body.extend_from_slice(&file_bytes);
    request_body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());
    
    let request = Request::post(ipfs_url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Content-Type", &format!("multipart/form-data; boundary={}", boundary))
        .body(request_body.into_body())
        .map_err(|e| format!("Failed to create request: {}", e))?;
    
    let mut response = Client::new().send(request).await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Clean up temp file
    let _ = std::fs::remove_file(&temp_path);
    
    if response.status().is_success() {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let response_str = std::str::from_utf8(&body_buf)
            .map_err(|e| format!("Failed to convert response to string: {}", e))?;
        
        eprintln!("IPFS API Response: {}", response_str);
        
        // Try to parse Lighthouse response format
        #[allow(non_snake_case)]
        #[derive(Debug, Deserialize)]
        struct LighthouseResponse {
            Hash: String,
        }
        
        let hash = match serde_json::from_slice::<LighthouseResponse>(&body_buf) {
            Ok(resp) => resp.Hash,
            Err(_) => {
                // Fallback: try to extract hash from response text
                if let Some(start) = response_str.find("\"Hash\":\"") {
                    if let Some(end) = response_str[start + 8..].find("\"") {
                        response_str[start + 8..start + 8 + end].to_string()
                    } else {
                        return Err("Could not extract hash from response".to_string());
                    }
                } else if let Some(start) = response_str.find("\"hash\":\"") {
                    if let Some(end) = response_str[start + 8..].find("\"") {
                        response_str[start + 8..start + 8 + end].to_string()
                    } else {
                        return Err("Could not extract hash from response".to_string());
                    }
                } else {
                    return Err(format!("Could not extract hash from response: {}", response_str));
                }
            }
        };
        
        Ok(hash)
    } else {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await
            .map_err(|e| format!("Failed to read error response: {}", e))?;
        let error_body = std::str::from_utf8(&body_buf).unwrap_or("unable to read error body");
        Err(format!("Failed to upload to IPFS. Status: {:?}, Body: {}", response.status(), error_body))
    }
}

/// Download and compare with previous snapshot to determine changed operators
async fn compare_with_previous_snapshot(
    current_snapshot: &OperatorStakeSnapshot,
    previous_hash: &str
) -> Result<Vec<Address>, String> {
    // Download previous snapshot from IPFS
    let previous_snapshot = download_snapshot_from_ipfs(previous_hash).await?;
    
    let mut operators_to_update = Vec::new();
    
    // Check for new operators or stake changes
    for (operator, current_stakes) in &current_snapshot.operators {
        let should_update = if let Some(previous_stakes) = previous_snapshot.operators.get(operator) {
            // Check if stakes have changed
            current_stakes != previous_stakes
        } else {
            // New operator
            true
        };
        
        if should_update {
            operators_to_update.push(*operator);
        }
    }
    
    // Check for removed operators (operators that had stakes before but don't now)
    for operator in previous_snapshot.operators.keys() {
        if !current_snapshot.operators.contains_key(operator) {
            operators_to_update.push(*operator);
        }
    }
    
    Ok(operators_to_update)
}

/// Download a snapshot from IPFS
async fn download_snapshot_from_ipfs(hash: &str) -> Result<OperatorStakeSnapshot, String> {
    let ipfs_gateway_url = std::env::var("WAVS_ENV_IPFS_GATEWAY_URL")
        .unwrap_or_else(|_| "https://gateway.lighthouse.storage/ipfs".to_string());
    
    let url = format!("{}/{}", ipfs_gateway_url, hash);
    eprintln!("Downloading snapshot from IPFS: {}", url);
    
    let request = Request::get(&url)
        .body(Vec::new().into_body())
        .map_err(|e| format!("Failed to create request: {}", e))?;
    
    let mut response = Client::new().send(request).await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    if response.status().is_success() {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        let snapshot: OperatorStakeSnapshot = serde_json::from_slice(&body_buf)
            .map_err(|e| format!("Failed to parse snapshot JSON: {}", e))?;
        
        eprintln!("Downloaded snapshot from block {}", snapshot.block_height);
        Ok(snapshot)
    } else {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await
            .map_err(|e| format!("Failed to read error response: {}", e))?;
        let error_body = std::str::from_utf8(&body_buf).unwrap_or("unable to read error body");
        Err(format!("Failed to download from IPFS. Status: {:?}, Body: {}", response.status(), error_body))
    }
}

export!(Component with_types_in bindings);
