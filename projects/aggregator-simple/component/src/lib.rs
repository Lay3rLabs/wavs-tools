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
        let chains_str =
            host::config_var("chain_names").ok_or("chain_names config variable is required")?;

        let chain_names: Vec<&str> = serde_json::from_str(&chains_str)
            .map_err(|e| format!("Could not parse chain names: {e}"))?;
        if chain_names.is_empty() {
            return Err("chain_names config is empty".to_string());
        }

        let mut actions = Vec::new();

        for chain_name in chain_names {
            if host::get_evm_chain_config(chain_name).is_some() {
                // Construct key like "service_handler_ethereum"
                let handler_key = format!("service_handler_{chain_name}");

                let service_handler_str = host::config_var(&handler_key)
                    .ok_or(format!("Missing config value for key '{handler_key}'"))?;

                let address: alloy_primitives::Address = service_handler_str
                    .parse()
                    .map_err(|e| format!("Failed to parse address for '{chain_name}': {e}"))?;

                let submit_action = SubmitAction {
                    chain_name: chain_name.to_string(),
                    contract_address: EvmAddress {
                        raw_bytes: address.to_vec(),
                    },
                };

                actions.push(AggregatorAction::Submit(submit_action));
            } else if host::get_cosmos_chain_config(chain_name).is_some() {
                todo!("Cosmos support coming soon...")
            } else {
                return Err(format!("Could not get chain config for chain {chain_name}"));
            }
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
