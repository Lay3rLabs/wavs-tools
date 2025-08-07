#[allow(warnings)]
#[rustfmt::skip]
mod bindings;

use crate::bindings::{
    export, host,
    wavs::aggregator::aggregator::{EvmAddress, SubmitAction},
    AggregatorAction, AnyTxHash, Guest, Packet,
};

struct Component;

impl Guest for Component {
    fn process_packet(_pkt: Packet) -> Result<Vec<AggregatorAction>, String> {
        // Fetch and parse comma-separated chain names
        let chains_str = host::config_var("evm_chain_names")
            .ok_or("evm_chain_names config variable is required")?;

        let evm_chain_names: Vec<String> = chains_str
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if evm_chain_names.is_empty() {
            return Err("evm_chain_names config is empty".to_string());
        }

        let mut actions = Vec::new();

        for chain_name in evm_chain_names {
            // Construct key like "evm_service_handler_ethereum"
            let handler_key = format!("evm_service_handler_{}", chain_name);

            let service_handler_str = host::config_var(&handler_key)
                .ok_or(format!("Missing config value for key '{}'", handler_key))?;

            let address: alloy_primitives::Address = service_handler_str
                .parse()
                .map_err(|e| format!("Failed to parse address for '{}': {e}", chain_name))?;

            let submit_action = SubmitAction {
                chain_name: chain_name.clone(),
                contract_address: EvmAddress {
                    raw_bytes: address.to_vec(),
                },
            };

            actions.push(AggregatorAction::Submit(submit_action));
        }

        Ok(actions)
    }

    fn handle_timer_callback(_packet: Packet) -> Result<Vec<AggregatorAction>, String> {
        Err("No timers used".to_string())
    }

    fn handle_submit_callback(
        _packet: Packet,
        tx_result: Result<AnyTxHash, String>,
    ) -> Result<(), String> {
        match tx_result {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}

export!(Component with_types_in bindings);
