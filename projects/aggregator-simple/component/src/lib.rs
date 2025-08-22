#[allow(warnings)]
#[rustfmt::skip]
mod bindings;
mod packet_serde;

use crate::bindings::{
    export, host,
    wasi::{
        clocks::monotonic_clock,
        keyvalue::store::{open, Bucket},
    },
    wavs::{
        aggregator::aggregator::{Duration, EvmAddress, Packet, SubmitAction, TimerAction},
        types::service::Submit,
    },
    AggregatorAction, AnyTxHash, Guest,
};
use crate::packet_serde::SerializablePacket;
use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

struct Component;

const BUCKET_NAME: &str = "aggregator";
const BATCH_KEY_PREFIX: &str = "batch:";
const DEDUP_KEY_PREFIX: &str = "dedup:";

#[derive(Serialize, Deserialize, Debug)]
struct StoredPacket {
    packet: SerializablePacket,
    timestamp: u64,
    priority: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BatchState {
    packets: Vec<StoredPacket>,
    timer_started: Option<u64>,
}

impl Component {
    fn get_config_u32(key: &str, default: u32) -> u32 {
        host::config_var(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_config_u64(key: &str, default: u64) -> u64 {
        host::config_var(key)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_config_string(key: &str) -> Option<String> {
        host::config_var(key)
    }

    fn get_bucket() -> Result<Bucket, String> {
        open(BUCKET_NAME).map_err(|e| format!("Failed to open bucket: {e:?}"))
    }

    fn is_duplicate(bucket: &Bucket, event_id: &[u8]) -> Result<bool, String> {
        let key = format!("{DEDUP_KEY_PREFIX}{}", hex::encode(event_id));
        let dedup_window = Self::get_config_u64("dedup_window_secs", 300);

        match bucket.get(&key) {
            Ok(Some(data)) => {
                if let Ok(timestamp_str) = String::from_utf8(data) {
                    if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                        let current_time = Self::current_timestamp();
                        if current_time - timestamp < dedup_window {
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            Ok(None) => Ok(false),
            Err(_) => Ok(false),
        }
    }

    fn mark_as_seen(bucket: &Bucket, event_id: &[u8]) -> Result<(), String> {
        let key = format!("{DEDUP_KEY_PREFIX}{}", hex::encode(event_id));
        let timestamp = Self::current_timestamp().to_string();
        bucket
            .set(&key, timestamp.as_bytes())
            .map_err(|e| format!("Failed to mark as seen: {e:?}"))
    }

    fn current_timestamp() -> u64 {
        // get nanoseconds from monotonic clock and convert to seconds
        monotonic_clock::now() / 1_000_000_000
    }

    fn load_batch_state(bucket: &Bucket) -> Result<BatchState, String> {
        let key = format!("{BATCH_KEY_PREFIX}state");
        match bucket.get(&key) {
            Ok(Some(data)) => serde_json::from_slice(&data)
                .map_err(|e| format!("Failed to deserialize batch: {e}")),
            Ok(None) => Ok(BatchState {
                packets: Vec::new(),
                timer_started: None,
            }),
            Err(e) => Err(format!("Failed to load batch: {e:?}")),
        }
    }

    fn save_batch_state(bucket: &Bucket, state: &BatchState) -> Result<(), String> {
        let key = format!("{BATCH_KEY_PREFIX}state");
        let data =
            serde_json::to_vec(state).map_err(|e| format!("Failed to serialize batch: {e}"))?;
        bucket
            .set(&key, &data)
            .map_err(|e| format!("Failed to save batch: {e:?}"))
    }

    fn clear_batch(bucket: &Bucket) -> Result<(), String> {
        let key = format!("{BATCH_KEY_PREFIX}state");
        bucket
            .delete(&key)
            .map_err(|e| format!("Failed to clear batch: {e:?}"))
    }

    fn packet_to_stored(packet: Packet) -> StoredPacket {
        let timestamp = Self::current_timestamp();
        let priority = if !packet.envelope.ordering.is_empty() {
            // Extract priority from ordering field if present
            packet.envelope.ordering.first().map(|&b| b as u32)
        } else {
            None
        };

        StoredPacket {
            packet: packet.into(),
            timestamp,
            priority,
        }
    }

    fn order_packets(packets: &mut [StoredPacket], order_by: &str) {
        match order_by {
            "timestamp" => packets.sort_by_key(|p| p.timestamp),
            "priority" => packets.sort_by_key(|p| p.priority.unwrap_or(u32::MAX)),
            _ => {}
        }
    }

    fn create_submit_action() -> Result<Vec<AggregatorAction>, String> {
        let workflow = host::get_workflow().workflow;
        let submit_config = match workflow.submit {
            Submit::None => return Err("No submit config".to_string()),
            Submit::Aggregator(aggregator_submit) => aggregator_submit.component.config,
        };

        if submit_config.is_empty() {
            return Err("Workflow submit component config is empty".to_string());
        }

        let mut actions = Vec::new();
        for (chain_name, service_handler_address) in submit_config {
            if host::get_evm_chain_config(&chain_name).is_some() {
                let address: Address = service_handler_address
                    .parse()
                    .map_err(|e| format!("Failed to parse address for '{chain_name}': {e}"))?;

                let submit_action = SubmitAction {
                    chain_name: chain_name.to_string(),
                    contract_address: EvmAddress {
                        raw_bytes: address.to_vec(),
                    },
                };
                actions.push(AggregatorAction::Submit(submit_action));
            } else if host::get_cosmos_chain_config(&chain_name).is_some() {
                todo!("Cosmos support coming soon...")
            } else {
                return Err(format!("Could not get chain config for chain {chain_name}"));
            }
        }
        Ok(actions)
    }
}

impl Guest for Component {
    fn process_packet(pkt: Packet) -> Result<Vec<AggregatorAction>, String> {
        let batch_size = Component::get_config_u32("batch_size", 10);
        let batch_timeout_secs = Component::get_config_u64("batch_timeout_secs", 60);
        let trigger_mode = Component::get_config_string("trigger_mode")
            .unwrap_or_else(|| "batch_full".to_string());
        let order_by = Component::get_config_string("order_by");

        let bucket = Component::get_bucket()?;

        // check for packet duplicates
        if Component::is_duplicate(&bucket, &pkt.envelope.event_id)? {
            return Ok(vec![]); // already processed, ignore
        } else {
            Component::mark_as_seen(&bucket, &pkt.envelope.event_id)?;
        }

        // if mode "immediate", just submit
        if trigger_mode == "immediate" {
            return Component::create_submit_action();
        }

        let mut batch_state = Component::load_batch_state(&bucket)?;
        batch_state.packets.push(Component::packet_to_stored(pkt));

        if let Some(ref order) = order_by {
            Component::order_packets(&mut batch_state.packets, order);
        }

        let batch_count = batch_state.packets.len() as u32;
        let actions = match trigger_mode.as_str() {
            "batch_full" => {
                // submit only when batch is full
                if batch_count >= batch_size {
                    Component::create_submit_action()?
                } else {
                    vec![]
                }
            }
            "timeout" => {
                // submit only on timeout
                if batch_state.timer_started.is_none() {
                    // first packet in batch, start timer
                    batch_state.timer_started = Some(Component::current_timestamp());
                    vec![AggregatorAction::Timer(TimerAction {
                        delay: Duration {
                            secs: batch_timeout_secs,
                        },
                    })]
                } else {
                    // timer already running, just wait
                    vec![]
                }
            }
            _ => {
                return Err(format!("Unknown trigger mode: {}", trigger_mode));
            }
        };

        Component::save_batch_state(&bucket, &batch_state)?;

        Ok(actions)
    }

    fn handle_timer_callback(_packet: Packet) -> Result<Vec<AggregatorAction>, String> {
        let bucket = Component::get_bucket()?;
        let batch_state = Component::load_batch_state(&bucket)?;

        if batch_state.packets.is_empty() {
            return Ok(vec![]);
        }

        Component::create_submit_action()
    }

    fn handle_submit_callback(
        _packet: Packet,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        let bucket = Component::get_bucket()?;

        match tx_result {
            Ok(_) => {
                Component::clear_batch(&bucket)?;
                Ok(())
            }
            Err(e) => {
                eprintln!("Submit failed: {e}");
                Ok(())
            }
        }
    }
}

mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}

export!(Component with_types_in bindings);
