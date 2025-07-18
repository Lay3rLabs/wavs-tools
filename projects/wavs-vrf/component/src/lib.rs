//! WAVS VRF (Verifiable Random Function) utilities with drand integration.
//!
//! This crate provides VRF functionality for the WAVS worker system, combining
//! drand randomness with trigger-specific data to generate deterministic but
//! unpredictable random values.

#[rustfmt::skip]
#[allow(clippy::all)]
mod bindings;
mod config;
mod drand;
mod trigger;
mod utils;
mod vrf;

use alloy_sol_types::SolValue;
use anyhow::Result;
use wstd::runtime::block_on;

use crate::bindings::{export, Guest, TriggerAction, WasmResponse};
use crate::config::Config;
use crate::drand::DrandClient;
use crate::trigger::TriggerInfo;
use crate::vrf::Vrf;

struct Component;

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<WasmResponse>, String> {
        block_on(async move {
            match process_trigger(action).await {
                Ok(response) => Ok(Some(response)),
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

/// Process a trigger action and generate VRF randomness
async fn process_trigger(trigger_action: TriggerAction) -> Result<WasmResponse> {
    // Load configuration from host
    let config = Config::from_host();

    // Extract trigger information
    let trigger_info = TriggerInfo::from_trigger_action(trigger_action, &config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to extract trigger info: {}", e))?;

    // Create drand client and fetch randomness
    let drand_client = DrandClient::new(config.drand_url, config.drand_chain_hash);
    let drand_randomness = drand_client
        .get_round(trigger_info.drand_round)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get drand randomness: {}", e))?;

    // Create VRF from drand randomness + unique ID
    let vrf_sources = [
        drand_randomness.as_slice(),
        trigger_info.unique_id.as_slice(),
    ];

    let vrf = Vrf::from_sources(&vrf_sources, trigger_info.drand_round);
    let result = vrf.generate();

    let payload = result.randomness.abi_encode();

    Ok(WasmResponse {
        payload,
        ordering: None,
    })
}

export!(Component with_types_in bindings);
