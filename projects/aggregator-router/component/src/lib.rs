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
        let chain_name =
            host::config_var("chain_name").ok_or("chain_name config variable is required")?;
        let service_handler_str = host::config_var("service_handler")
            .ok_or("service_handler config variable is required")?;

        let address: alloy_primitives::Address = service_handler_str
            .parse()
            .map_err(|e| format!("Failed to parse service handler address: {e}"))?;

        let submit_action = SubmitAction {
            chain_name,
            contract_address: EvmAddress {
                raw_bytes: address.to_vec(),
            },
        };

        Ok(vec![AggregatorAction::Submit(submit_action)])
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
